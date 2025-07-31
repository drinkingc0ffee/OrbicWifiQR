// Minimal Orbic WiFi QR Code Display App
// - Hold menu button (event1) 1.5–6s: show QR for 30s
// - QR encodes active WiFi SSID/PSK from /usrdata/data/usr/wlan/wlan_conf_6174.xml
// - Triple-press exit

use qrcode::QrCode;
use image::Luma;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::{Duration, Instant};

const INPUT_EVENT_SIZE: usize = 16;

#[derive(Debug, PartialEq)]
enum Event {
    KeyDown,
    KeyUp,
}

struct OrbicFramebuffer;
impl OrbicFramebuffer {
    fn new() -> Self { Self }
    fn write_buffer(&mut self, buffer: &[(u8, u8, u8)]) {
        let mut raw_buffer = Vec::new();
        for (r, g, b) in buffer {
            let mut rgb565: u16 = (*r as u16 & 0b11111000) << 8;
            rgb565 |= (*g as u16 & 0b11111100) << 3;
            rgb565 |= (*b as u16) >> 3;
            raw_buffer.extend(rgb565.to_le_bytes());
        }
        std::fs::write("/dev/fb0", &raw_buffer).ok();
    }
}

fn parse_event(input: &[u8]) -> Option<Event> {
    if input.len() < INPUT_EVENT_SIZE { return None; }
    let event_type = u16::from_le_bytes([input[8], input[9]]);
    let event_value = i32::from_le_bytes([input[12], input[13], input[14], input[15]]);
    if event_type == 1 {
        if event_value == 1 { Some(Event::KeyDown) }
        else if event_value == 0 { Some(Event::KeyUp) } else { None }
    } else { None }
}

fn clear_display(fb: &mut OrbicFramebuffer) {
    fb.write_buffer(&vec![(255, 255, 255); 128 * 128]);
}

fn get_wifi_credentials() -> (String, String, String) {
    let xml = std::fs::read_to_string("/usrdata/data/usr/wlan/wlan_conf_6174.xml").unwrap_or_default();
    let (mut ssid, mut psk, mut encryption) = (String::new(), String::new(), "WPA2".to_string());
    for tag in ["<Basic_0>", "<Basic_1>"] {
        if let Some(start) = xml.find(tag) {
            if let Some(state_start) = xml[start..].find("<state>") {
                if let Some(state_end) = xml[start + state_start..].find("</state>") {
                    let state = &xml[start + state_start + 7..start + state_start + state_end];
                    if state.trim() == "1" {
                        if let Some(ssid_start) = xml[start..].find("<ssid>") {
                            if let Some(ssid_end) = xml[start + ssid_start..].find("</ssid>") {
                                ssid = xml[start + ssid_start + 6..start + ssid_start + ssid_end].to_string();
                            }
                        }
                        if let Some(psk_start) = xml[start..].find("<psk>") {
                            if let Some(psk_end) = xml[start + psk_start..].find("</psk>") {
                                psk = xml[start + psk_start + 5..start + psk_start + psk_end].to_string();
                            }
                        }
                        // Determine encryption type based on security and encrypt values
                        if let Some(security_start) = xml[start..].find("<security>") {
                            if let Some(security_end) = xml[start + security_start..].find("</security>") {
                                let security = &xml[start + security_start + 10..start + security_start + security_end];
                                if let Some(encrypt_start) = xml[start..].find("<encrypt>") {
                                    if let Some(encrypt_end) = xml[start + encrypt_start..].find("</encrypt>") {
                                        let encrypt = &xml[start + encrypt_start + 9..start + encrypt_start + encrypt_end];
                                        // security=3, encrypt=2 typically means WPA2-PSK
                                        if security.trim() == "3" && encrypt.trim() == "2" {
                                            encryption = "WPA2".to_string();
                                        } else if security.trim() == "2" {
                                            encryption = "WPA".to_string();
                                        } else if security.trim() == "1" {
                                            encryption = "WEP".to_string();
                                        } else {
                                            encryption = "nopass".to_string();
                                        }
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    (ssid, psk, encryption)
}

fn generate_wifi_qr_code(ssid: &str, password: &str, encryption: &str) -> String {
    // Escape special characters as per ZXing specification
    let escape_special_chars = |s: &str| -> String {
        s.replace("\\", "\\\\")
         .replace(";", "\\;")
         .replace(",", "\\,")
         .replace("\"", "\\\"")
         .replace(":", "\\:")
    };
    
    let escaped_ssid = escape_special_chars(ssid);
    let escaped_password = escape_special_chars(password);
    
    // Format according to ZXing WiFi specification: WIFI:T:<type>;S:<ssid>;P:<password>;;
    let qr_data = format!("WIFI:T:{};S:{};P:{};;", encryption, escaped_ssid, escaped_password);
    println!("DEBUG: Generated QR data: '{}'", qr_data);
    println!("DEBUG: Original SSID: '{}', Escaped SSID: '{}'", ssid, escaped_ssid);
    qr_data
}

fn display_qr_code(fb: &mut OrbicFramebuffer, qr_text: &str) {
    // Try with lower error correction level for easier scanning
    if let Ok(code) = QrCode::with_error_correction_level(qr_text, qrcode::EcLevel::L) {
        // Use 120x120 size for reliability, but center it on 128x128 display
        let image = code.render::<Luma<u8>>().max_dimensions(120, 120).build();
        let mut buffer = vec![(255, 255, 255); 128 * 128];
        
        // Calculate centering offsets (120x120 centered on 128x128 = 4 pixel margin on each side)
        let qr_width = image.width() as i32;
        let qr_height = image.height() as i32;
        let offset_x = (128 - qr_width) / 2;
        let offset_y = (128 - qr_height) / 2;
        
        for y in 0..qr_height {
            for x in 0..qr_width {
                let pixel = image.get_pixel(x as u32, y as u32)[0];
                let color = if pixel == 0 { (0, 0, 0) } else { (255, 255, 255) };
                let buffer_x = (x + offset_x) as usize;
                let buffer_y = (y + offset_y) as usize;
                if buffer_x < 128 && buffer_y < 128 {
                    let idx = buffer_y * 128 + buffer_x;
                    buffer[idx] = color;
                }
            }
        }
        fb.write_buffer(&buffer);
    }
}

fn main() {
    println!("=== ORBIC WIFI QR APP STARTING ===");
    let mut fb = OrbicFramebuffer::new();
    let mut file = File::open("/dev/input/event1").expect("open /dev/input/event1");
    let mut button_pressed = false;
    let mut press_start_time = Instant::now();
    let min_press = Duration::from_millis(1500);
    let max_press = Duration::from_secs(6);
    let mut quick_press_times: Vec<Instant> = Vec::new();
    let triple_press_window = Duration::from_secs(2);
    let quick_press_max = Duration::from_millis(500);
    loop {
        let mut event_buffer = [0u8; INPUT_EVENT_SIZE];
        if file.read_exact(&mut event_buffer).is_ok() {
            if let Some(event) = parse_event(&event_buffer) {
                match event {
                    Event::KeyDown => {
                        if !button_pressed {
                            button_pressed = true;
                            press_start_time = Instant::now();
                        }
                    }
                    Event::KeyUp => {
                        if button_pressed {
                            button_pressed = false;
                            let press_duration = press_start_time.elapsed();
                            if press_duration <= quick_press_max {
                                let now = Instant::now();
                                quick_press_times.push(now);
                                quick_press_times.retain(|&t| now.duration_since(t) <= triple_press_window);
                                if quick_press_times.len() >= 3 {
                                    clear_display(&mut fb);
                                    return;
                                }
                            } else {
                                quick_press_times.clear();
                                if press_duration >= min_press && press_duration <= max_press {
                                    let (ssid, password, encryption) = get_wifi_credentials();
                                    if !ssid.is_empty() && !password.is_empty() {
                                        let qr = generate_wifi_qr_code(&ssid, &password, &encryption);
                                        for _ in 0..300 { display_qr_code(&mut fb, &qr); thread::sleep(Duration::from_millis(100)); }
                                        
                                        clear_display(&mut fb);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            thread::sleep(Duration::from_millis(50));
        }
    }
}
