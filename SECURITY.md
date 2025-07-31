# Security Policy

## Supported Versions

Use this section to tell people about which versions of your project are
currently being supported with security updates.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability in OrbicWifiQR, please follow these steps:

### 1. **DO NOT** create a public GitHub issue
Security vulnerabilities should be reported privately to avoid potential exploitation.

### 2. Report the vulnerability
Please email security details to: security@example.com

Include the following information:
- **Description**: Clear description of the vulnerability
- **Steps to reproduce**: Detailed steps to reproduce the issue
- **Impact**: Potential impact of the vulnerability
- **Suggested fix**: If you have ideas for fixing the issue
- **Affected versions**: Which versions are affected
- **Device information**: Target device details if relevant

### 3. Response timeline
- **Initial response**: Within 48 hours
- **Status update**: Within 1 week
- **Fix timeline**: Depends on severity and complexity

### 4. Disclosure
- Vulnerabilities will be disclosed via GitHub Security Advisories
- CVE numbers will be requested for significant issues
- Patches will be released as soon as possible

## Security Considerations

### Current Security Features
- **Static linking**: Reduces attack surface by eliminating dynamic library dependencies
- **Minimal dependencies**: Only essential crates are used
- **Root access**: Required for device control, but limited scope
- **No network access**: App doesn't communicate over network

### Known Limitations
- **Root access required**: App needs root permissions for device control
- **Device-specific**: Only works on Orbic devices with 128x128 displays
- **WiFi credentials**: Reads from device configuration files
- **No encryption**: QR codes contain WiFi credentials in plain text

### Best Practices
- **Regular updates**: Keep the app updated to latest version
- **Device security**: Ensure device has proper security measures
- **Network security**: Use strong WiFi passwords
- **Physical security**: Protect devices from unauthorized access

## Security Checklist

### For Contributors
- [ ] No hardcoded credentials in code
- [ ] No sensitive data in logs
- [ ] Input validation for all user inputs
- [ ] Error messages don't leak sensitive information
- [ ] Dependencies are up to date
- [ ] Code follows security best practices

### For Users
- [ ] Use latest version of the app
- [ ] Keep device firmware updated
- [ ] Use strong WiFi passwords
- [ ] Monitor device access
- [ ] Report suspicious activity

## Security Updates

Security updates will be released as patch versions (e.g., 0.1.1, 0.1.2) and will be clearly marked as security fixes in the changelog.

## Contact

For security-related questions or concerns:
- **Email**: security@example.com
- **GitHub**: Create a private security advisory
- **Response time**: Within 48 hours for urgent issues

---

**Note**: This security policy is designed to protect both users and contributors. We appreciate responsible disclosure and will work with reporters to address security issues promptly. 