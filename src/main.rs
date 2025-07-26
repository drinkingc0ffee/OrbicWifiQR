// src/bin/test_qr.rs
// Display a QR code for eff.org on first button press, then exit

use qrcode::QrCode;
use image::Luma;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::{Duration, Instant};

// Linux input_event structure size (16 bytes on 32-bit systems)
const INPUT_EVENT_SIZE: usize = 16;

#[derive(Debug, PartialEq)]
enum Event {
    KeyDown,
    KeyUp,
}

#[derive(Copy, Clone)]
struct Dimensions {
    height: u32,
    width: u32,
}

// Based on the working Orbic framebuffer code
struct OrbicFramebuffer {
    dimensions: Dimensions,
}

impl OrbicFramebuffer {
    fn new() -> Self {
        Self {
            dimensions: Dimensions {
                height: 128,
                width: 128,
            }
        }
    }

    fn dimensions(&self) -> Dimensions {
        self.dimensions
    }

    // Based on daemon/src/display/orbic.rs write_buffer implementation
    fn write_buffer(&mut self, buffer: &[(u8, u8, u8)]) {
        let mut raw_buffer = Vec::new();
        for (r, g, b) in buffer {
            let mut rgb565: u16 = (*r as u16 & 0b11111000) << 8;
            rgb565 |= (*g as u16 & 0b11111100) << 3;
            rgb565 |= (*b as u16) >> 3;
            raw_buffer.extend(rgb565.to_le_bytes());
        }
        std::fs::write("/dev/fb0", &raw_buffer).expect("Failed to write to framebuffer");
    }
}

fn parse_event(input: &[u8]) -> Option<Event> {
    if input.len() < INPUT_EVENT_SIZE {
        return None;
    }
    
    let event_type = u16::from_le_bytes([input[8], input[9]]);
    let event_value = i32::from_le_bytes([input[12], input[13], input[14], input[15]]);
    
    // EV_KEY = 1
    if event_type == 1 {
        if event_value == 1 {
            Some(Event::KeyDown)
        } else if event_value == 0 {
            Some(Event::KeyUp)
        } else {
            None
        }
    } else {
        None
    }
}

fn clear_display(fb: &mut OrbicFramebuffer) {
    let dimensions = fb.dimensions();
    let buffer = vec![(0, 0, 0); (dimensions.width * dimensions.height) as usize];
    fb.write_buffer(&buffer);
}

fn get_wifi_credentials() -> (String, String, String) {
    println!("Searching for WiFi configuration...");
    
    // Try multiple configuration sources in order of preference
    
    // 1. Try reading from the Orbic XML config file (highest priority)
    if let Ok(xml_content) = std::fs::read_to_string("/usrdata/data/usr/wlan/wlan_conf_6174.xml") {
        println!("Found Orbic XML config file");
        
        // Parse XML to find the active network (state=1)
        let mut active_ssid = String::new();
        let mut active_password = String::new();
        
        // Find Basic_0 and Basic_1 sections and check their state
        let basic_0_start = xml_content.find("<Basic_0>");
        let basic_1_start = xml_content.find("<Basic_1>");
        
        if let Some(b0_start) = basic_0_start {
            // Check Basic_0 state
            if let Some(state_start) = xml_content[b0_start..].find("<state>") {
                if let Some(state_end) = xml_content[b0_start + state_start..].find("</state>") {
                    let state_value = xml_content[b0_start + state_start + 7..b0_start + state_start + state_end].trim();
                    println!("Basic_0 state: {}", state_value);
                    
                    if state_value == "1" {
                        // Basic_0 is active, get its SSID and PSK
                        if let Some(ssid_start) = xml_content[b0_start..].find("<ssid>") {
                            if let Some(ssid_end) = xml_content[b0_start + ssid_start..].find("</ssid>") {
                                active_ssid = xml_content[b0_start + ssid_start + 6..b0_start + ssid_start + ssid_end].to_string();
                            }
                        }
                        if let Some(psk_start) = xml_content[b0_start..].find("<psk>") {
                            if let Some(psk_end) = xml_content[b0_start + psk_start..].find("</psk>") {
                                active_password = xml_content[b0_start + psk_start + 5..b0_start + psk_start + psk_end].to_string();
                            }
                        }
                        println!("Using Basic_0 (2.4GHz) - SSID: {}, Encryption: WPA2", active_ssid);
                    }
                }
            }
        }
        
        // If Basic_0 is not active, check Basic_1
        if active_ssid.is_empty() && active_password.is_empty() {
            if let Some(b1_start) = basic_1_start {
                // Check Basic_1 state
                if let Some(state_start) = xml_content[b1_start..].find("<state>") {
                    if let Some(state_end) = xml_content[b1_start + state_start..].find("</state>") {
                        let state_value = xml_content[b1_start + state_start + 7..b1_start + state_start + state_end].trim();
                        println!("Basic_1 state: {}", state_value);
                        
                        if state_value == "1" {
                            // Basic_1 is active, get its SSID and PSK
                            if let Some(ssid_start) = xml_content[b1_start..].find("<ssid>") {
                                if let Some(ssid_end) = xml_content[b1_start + ssid_start..].find("</ssid>") {
                                    active_ssid = xml_content[b1_start + ssid_start + 6..b1_start + ssid_start + ssid_end].to_string();
                                }
                            }
                            if let Some(psk_start) = xml_content[b1_start..].find("<psk>") {
                                if let Some(psk_end) = xml_content[b1_start + psk_start..].find("</psk>") {
                                    active_password = xml_content[b1_start + psk_start + 5..b1_start + psk_start + psk_end].to_string();
                                }
                            }
                            println!("Using Basic_1 (5GHz) - SSID: {}, Encryption: WPA2", active_ssid);
                        }
                    }
                }
            }
        }
        
        if !active_ssid.is_empty() && !active_password.is_empty() {
            return (active_ssid, active_password, "WPA2".to_string());
        }
    }
    
    // 2. Try reading from hostapd config (most common for hotspots)
    if let Ok(hostapd_content) = std::fs::read_to_string("/etc/hostapd/hostapd.conf") {
        println!("Found hostapd config file");
        let mut ssid = String::new();
        let mut password = String::new();
        
        for line in hostapd_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("ssid=") {
                ssid = trimmed[5..].to_string();
            } else if trimmed.starts_with("wpa_passphrase=") {
                password = trimmed[14..].to_string();
            }
        }
        
        if !ssid.is_empty() && !password.is_empty() {
            println!("Using hostapd config - SSID: {}, Encryption: WPA2", ssid);
            return (ssid, password, "WPA2".to_string());
        }
    }
    
    // 3. Try reading from wpa_supplicant config
    if let Ok(wpa_content) = std::fs::read_to_string("/etc/wpa_supplicant/wpa_supplicant.conf") {
        println!("Found wpa_supplicant config file");
        let mut ssid = String::new();
        let mut password = String::new();
        let mut in_network = false;
        
        for line in wpa_content.lines() {
            let trimmed = line.trim();
            if trimmed == "network={" {
                in_network = true;
            } else if trimmed == "}" {
                in_network = false;
                if !ssid.is_empty() && !password.is_empty() {
                    break;
                }
            } else if in_network {
                if trimmed.starts_with("ssid=") {
                    ssid = trimmed[5..].trim_matches('"').to_string();
                } else if trimmed.starts_with("psk=") {
                    password = trimmed[4..].trim_matches('"').to_string();
                }
            }
        }
        
        if !ssid.is_empty() && !password.is_empty() {
            println!("Using wpa_supplicant config - SSID: {}, Encryption: WPA2", ssid);
            return (ssid, password, "WPA2".to_string());
        }
    }
    
    // 4. Fallback to hardcoded values if no config found
    println!("No WiFi configuration found, using fallback values");
    ("OrbicHotspot".to_string(), "password123".to_string(), "WPA2".to_string())
}

fn generate_wifi_qr_code(ssid: &str, password: &str, encryption: &str) -> String {
    // Generate WiFi QR code in the standard format
    // Format: WIFI:S:<SSID>;T:<WPA|WPA2|WEP|nopass>;P:<password>;;
    
    let qr_data = format!("WIFI:S:{};T:{};P:{};;", ssid, encryption, password);
    println!("Generated WiFi QR code data: {}", qr_data);
    qr_data
}

fn display_qr_code(fb: &mut OrbicFramebuffer, qr_text: &str) -> Result<Vec<(u8, u8, u8)>, Box<dyn std::error::Error>> {
    println!("Generating QR code for WiFi connection...");
    
    // Try to create a smaller QR code with lower error correction
    let code = match QrCode::new(qr_text) {
        Ok(code) => code,
        Err(_) => {
            // If that fails, try with minimal error correction
            QrCode::with_error_correction_level(qr_text, qrcode::EcLevel::L)?
        }
    };
    
    let dimensions = fb.dimensions();
    
    // Calculate a smaller size that will fit well on the screen
    // Leave some margin around the edges
    let margin = 8u32;
    let max_qr_size = dimensions.width.min(dimensions.height) - (2 * margin);
    
    // Render QR code with margins
    let qr_image = code.render::<Luma<u8>>()
        .max_dimensions(max_qr_size, max_qr_size)
        .min_dimensions(max_qr_size, max_qr_size)
        .dark_color(Luma([0u8]))    // Black
        .light_color(Luma([255u8])) // White
        .build();
    
    println!("QR code image created: {}x{} for {}x{} screen (with {}px margins)", 
             qr_image.width(), qr_image.height(), dimensions.width, dimensions.height, margin);
    
    let mut buffer = Vec::new();
    
    // Create buffer with white background and centered QR code
    for y in 0..dimensions.height {
        for x in 0..dimensions.width {
            // Calculate QR code position (centered)
            let qr_x = x as i32 - margin as i32;
            let qr_y = y as i32 - margin as i32;
            
            if qr_x >= 0 && qr_x < qr_image.width() as i32 && 
               qr_y >= 0 && qr_y < qr_image.height() as i32 {
                // Inside QR code area
                let pixel = qr_image.get_pixel(qr_x as u32, qr_y as u32);
                let intensity = pixel[0];
                buffer.push((intensity, intensity, intensity));
            } else {
                // Outside QR code area - white background
                buffer.push((255, 255, 255));
            }
        }
    }
    
    println!("Writing WiFi QR code to framebuffer...");
    fb.write_buffer(&buffer);
    println!("WiFi QR code displayed successfully");
    
    Ok(buffer)
}

fn display_qr_code_for_duration(fb: &mut OrbicFramebuffer, qr_text: &str, duration: Duration) -> Result<(), Box<dyn std::error::Error>> {
    println!("Displaying WiFi QR code for {:?}...", duration);
    
    // Generate the QR code buffer once
    let qr_buffer = display_qr_code(fb, qr_text)?;
    
    let start_time = Instant::now();
    let refresh_interval = Duration::from_millis(100); // Refresh every 100ms
    
    while start_time.elapsed() < duration {
        // Continuously refresh the framebuffer to prevent overwrites
        fb.write_buffer(&qr_buffer);
        
        // Sleep for a short interval before next refresh
        thread::sleep(refresh_interval);
    }
    
    println!("Display duration completed - clearing display");
    clear_display(fb);
    println!("Display cleared");
    
    Ok(())
}

fn main() {
    let mut fb = OrbicFramebuffer::new();
    
    // Get WiFi credentials
    let (ssid, password, encryption) = get_wifi_credentials();
    let wifi_qr_data = generate_wifi_qr_code(&ssid, &password, &encryption);
    
    println!("WiFi Hotspot QR Code Display App v1.1");
    println!("Network: {} ({})", ssid, encryption);
    println!("Press and hold the WPS reset button (event1) for 1.5-6 seconds to display QR code");
    println!("Presses shorter than 1.5s or longer than 6s will be ignored");
    println!("Triple-press the button quickly (3 times within 2 seconds) to exit the app");
    
    // Open event1 for button monitoring
    let mut file = match File::open("/dev/input/event1") {
        Ok(f) => {
            println!("Successfully opened /dev/input/event1");
            f
        }
        Err(e) => {
            println!("Failed to open /dev/input/event1: {}", e);
            println!("Make sure you're running with root privileges via /bin/rootshell");
            return;
        }
    };
    
    let mut button_pressed = false;
    let mut press_start_time = Instant::now();
    let mut read_count = 0;
    
    // Timing constraints
    let min_press_duration = Duration::from_millis(1500); // 1.5 seconds
    let max_press_duration = Duration::from_secs(6);      // 6 seconds (changed from 3)
    
    // Triple-press exit detection
    let mut quick_press_times: Vec<Instant> = Vec::new();
    let triple_press_window = Duration::from_secs(2);     // 2 seconds window for triple-press
    let quick_press_max_duration = Duration::from_millis(500); // Max duration for a "quick" press
    
    println!("Monitoring /dev/input/event1 for button presses...");
    println!("Debug: Starting read loop with {} byte events...", INPUT_EVENT_SIZE);
    
    loop {
        read_count += 1;
        if read_count % 1000 == 0 {
            println!("Debug: Read count: {}", read_count);
        }
        
        // Read complete input_event structure
        let mut event_buffer = [0u8; INPUT_EVENT_SIZE];
        match file.read_exact(&mut event_buffer) {
            Ok(_) => {
                if let Some(event) = parse_event(&event_buffer) {
                    println!("Debug: Parsed event: {:?}", event);
                    match event {
                        Event::KeyDown => {
                            if !button_pressed {
                                button_pressed = true;
                                press_start_time = Instant::now();
                                println!("Button pressed - timing started");
                            }
                        }
                        Event::KeyUp => {
                            if button_pressed {
                                button_pressed = false;
                                let press_duration = press_start_time.elapsed();
                                println!("Button released after {:?}", press_duration);
                                
                                // Check for quick press (potential part of triple-press exit)
                                if press_duration <= quick_press_max_duration {
                                    let now = Instant::now();
                                    quick_press_times.push(now);
                                    
                                    // Clean up old press times outside the window
                                    quick_press_times.retain(|&time| now.duration_since(time) <= triple_press_window);
                                    
                                    println!("Quick press detected ({:?}) - count in window: {}", press_duration, quick_press_times.len());
                                    
                                    // Check for triple-press exit
                                    if quick_press_times.len() >= 3 {
                                        println!("🚪 Triple-press detected! Exiting WiFi QR app...");
                                        println!("Goodbye!");
                                        return;
                                    }
                                } else {
                                    // Clear quick press times for longer presses
                                    quick_press_times.clear();
                                    
                                    // Check if press duration is within valid range for QR display
                                    if press_duration >= min_press_duration && press_duration <= max_press_duration {
                                        println!("Valid press duration! Displaying WiFi QR code...");
                                        match display_qr_code_for_duration(&mut fb, &wifi_qr_data, Duration::from_secs(30)) {
                                            Ok(_) => {
                                                println!("WiFi QR code display completed. Ready for next button press.");
                                            }
                                            Err(e) => {
                                                println!("Failed to display WiFi QR code: {}", e);
                                            }
                                        }
                                    } else if press_duration < min_press_duration {
                                        println!("Press too short ({:?} < {:?}) - ignored", press_duration, min_press_duration);
                                    } else {
                                        println!("Press too long ({:?} > {:?}) - ignored", press_duration, max_press_duration);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to read input event: {} (read count: {})", e, read_count);
                // Don't sleep too long on error to avoid missing events
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

