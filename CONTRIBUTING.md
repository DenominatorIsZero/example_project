# Contributing to AI Demo Template

Thank you for your interest in improving this template! This document provides guidelines for contributing to the template itself (not for using the template - see the README for usage instructions).

## Development Guidelines

### Template Philosophy

This template is designed as a **reusable foundation** for AI demos, not a specific application. Contributions should:

- **Benefit all users**: Improvements should help everyone using the template
- **Maintain simplicity**: Avoid over-engineering or unnecessary complexity  
- **Preserve flexibility**: Keep customization points clear and accessible
- **Support both platforms**: Ensure changes work on native and WASM targets

### Contributing Process

1. **Fork and branch**: Create feature branches with descriptive names
2. **Test thoroughly**: Run `just check` and verify WASM builds work
3. **Update documentation**: Keep README and code comments current
4. **Submit focused PRs**: One improvement per pull request

### Code Standards

- **Formatting**: Use `just fmt` (standard rustfmt)
- **Quality**: Use `just lint` (clippy with warnings as errors)
- **Testing**: Use `just test` (all tests must pass)
- **Architecture**: Maintain clean separation between training/interactive/shared crates
- **WASM**: Preserve web compatibility in interactive components

## Issue Reporting

### Bug Reports

Include the following information:
- Clear reproduction steps
- System details (OS, Rust version, browser for WASM issues)
- Error messages and logs
- Whether the issue affects native builds, WASM builds, or both

### Feature Requests

Consider whether the proposed feature:
- Benefits all template users (not just specific projects)
- Maintains template simplicity and flexibility
- Fits the template's scope as a foundation rather than a specific application

## Types of Contributions Welcome

### Template Improvements
- Better error handling and user feedback
- Performance optimizations for WASM builds
- More robust model loading and validation
- Enhanced build system and development workflow

### Documentation
- Clearer customization instructions
- Better architecture explanations
- Additional troubleshooting guidance
- Code comment improvements

### Testing and Quality
- Additional test coverage
- Cross-platform validation
- Integration testing improvements
- WASM compatibility verification

## Getting Help

- **Template usage**: See README.md customization sections
- **Architecture questions**: Check docs/architecture.md
- **Development issues**: Search existing GitHub issues or create a new one

## License

This template uses the MIT License. Contributions are welcome and will be incorporated under the same license terms.