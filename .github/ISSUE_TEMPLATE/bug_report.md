---
name: Bug report
about: Create a report to help us improve
title: '[BUG] '
labels: ['bug']
assignees: ''

---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Install the app using `./install.sh`
2. Hold menu button for '...' seconds
3. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Actual behavior**
A clear and concise description of what actually happened.

**Device Information**
- **Device Model**: [e.g., Orbic Speed]
- **Architecture**: [e.g., armv7l]
- **OS Version**: [e.g., Android 8.1]
- **Display Size**: [e.g., 128x128]

**Logs**
Please include relevant logs:
```bash
# Check if app is running
adb shell "ps aux | grep orbic-wifi-qr"

# View app logs
adb shell "cat /data/orbic-wifi-qr/orbic-wifi-qr.log"

# Check service status
adb shell "/bin/rootshell /etc/init.d/orbic-wifi-qr status"
```

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment**
- **Host OS**: [e.g., macOS 14.0]
- **Docker Version**: [e.g., 24.0.5]
- **ADB Version**: [e.g., 34.0.5]
- **Rust Version**: [e.g., 1.86.0]

**Additional context**
Add any other context about the problem here.

**Checklist**
- [ ] I have searched existing issues for similar problems
- [ ] I have provided all required device information
- [ ] I have included relevant logs
- [ ] I have tested on actual Orbic hardware
- [ ] I have verified the WiFi configuration exists 