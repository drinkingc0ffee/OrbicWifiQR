# Contributing to OrbicWifiQR

Thank you for your interest in contributing to OrbicWifiQR! This document provides guidelines and information for contributors.

## Code of Conduct

This project is committed to providing a welcoming and inclusive environment for all contributors. Please be respectful and considerate in all interactions.

## How Can I Contribute?

### Reporting Bugs

- Use the GitHub issue tracker
- Include detailed steps to reproduce the bug
- Provide device information (architecture, OS version)
- Include relevant logs and error messages
- Check if the issue has already been reported

### Suggesting Enhancements

- Use the GitHub issue tracker with the "enhancement" label
- Describe the feature and its benefits
- Consider backward compatibility
- Provide implementation suggestions if possible

### Pull Requests

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes**:
   - Follow the coding style (see below)
   - Add tests if applicable
   - Update documentation
4. **Test your changes**:
   - Build the project: `./build.sh`
   - Test on target device if possible
5. **Commit your changes**: `git commit -m 'Add feature description'`
6. **Push to your fork**: `git push origin feature/your-feature-name`
7. **Create a Pull Request**

## Development Setup

### Prerequisites

- Rust 1.86+
- Docker
- ADB (Android Debug Bridge)
- Target device for testing

### Local Development

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/OrbicWifiQR.git
   cd OrbicWifiQR
   ```

2. **Build the project**:
   ```bash
   ./build.sh
   ```

3. **Install on device**:
   ```bash
   ./install.sh
   ```

### Testing

- Test on actual Orbic hardware when possible
- Verify QR code generation and display
- Test button interactions
- Check service startup and management

## Coding Standards

### Rust Code Style

- Follow Rust formatting guidelines: `cargo fmt`
- Use `cargo clippy` for linting
- Write clear, documented code
- Use meaningful variable and function names
- Add comments for complex logic

### Commit Messages

- Use clear, descriptive commit messages
- Start with a verb (Add, Fix, Update, etc.)
- Keep the first line under 50 characters
- Add detailed description if needed

### Documentation

- Update README.md for user-facing changes
- Add inline comments for complex code
- Update installation instructions if needed
- Document new configuration options

## Project Structure

```
OrbicWifiQR/
├── src/
│   └── main.rs              # Main application
├── build.sh                 # Build script
├── install.sh               # Installation script
├── devenv.dockerfile        # Docker environment
├── Cargo.toml              # Dependencies
├── README.md               # Project documentation
├── LICENSE                 # MIT License
├── CONTRIBUTING.md         # This file
└── .gitignore             # Git ignore rules
```

## Areas for Contribution

### High Priority

- **Bug fixes**: Stability and reliability improvements
- **Documentation**: Better user guides and troubleshooting
- **Testing**: More comprehensive test coverage
- **Error handling**: Better error messages and recovery

### Medium Priority

- **Performance**: Optimize QR code generation and display
- **Features**: Additional WiFi configuration support
- **UI improvements**: Better visual feedback
- **Logging**: Enhanced debugging capabilities

### Low Priority

- **Code cleanup**: Refactoring and optimization
- **Documentation**: Additional examples and tutorials
- **Tools**: Development and deployment scripts

## Testing Guidelines

### Manual Testing

1. **Installation**: Test the install script on clean devices
2. **Functionality**: Verify QR code display and button controls
3. **Service management**: Test start/stop/restart commands
4. **Error conditions**: Test with missing WiFi config, etc.

### Automated Testing

- Add unit tests for core functions
- Add integration tests for device interaction
- Test cross-compilation for different targets

## Release Process

1. **Version bump**: Update version in Cargo.toml
2. **Changelog**: Document changes in README.md
3. **Tag release**: Create git tag with version
4. **Test release**: Verify installation and functionality
5. **Publish**: Create GitHub release

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Documentation**: Check README.md and inline comments

## License

By contributing to OrbicWifiQR, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to OrbicWifiQR! 🎉 