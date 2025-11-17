# Contributing to Cosmic Eyes

Thank you for your interest in contributing to Cosmic Eyes! This document provides guidelines and information for contributors.

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Help create a positive community

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/yourusername/cosmic-eyes.git`
3. Add upstream remote: `git remote add upstream https://github.com/originaluser/cosmic-eyes.git`
4. Create a new branch: `git checkout -b feature/your-feature-name`

## Development Setup

### Prerequisites
- Rust 1.70 or later
- COSMIC Desktop environment (for testing)
- `just` task runner (optional): `cargo install just`

### Building
```bash
just build      # Debug build
just run        # Run the applet
just check      # Run clippy linter
just fmt        # Format code
just test       # Run tests
```

## Making Changes

### Before You Start
1. Check if an issue exists for what you want to do
2. If not, create an issue to discuss the change
3. Get feedback before starting major work

### Coding Standards

#### Rust Style
- Follow the official [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `just fmt` before committing
- Run `just check` and fix all clippy warnings
- Use meaningful variable and function names
- Add doc comments for public APIs

#### Example:
```rust
/// Starts a break of the specified type
///
/// # Arguments
/// * `break_type` - The type of break to start (Short or Long)
///
/// # Examples
/// ```
/// timer.start_break(BreakType::Short).await;
/// ```
pub async fn start_break(&self, break_type: BreakType) {
    // Implementation
}
```

#### File Organization
- Keep files focused and modular
- Put related functionality in the same module
- Use `mod.rs` for module entry points
- Keep files under 500 lines when possible

### Commit Messages

Use clear, descriptive commit messages following this format:

```
<type>: <short summary>

<optional detailed description>

<optional footer>
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat: add postpone duration configuration option

Added a new config option to customize how long breaks are
postponed. Defaults to 5 minutes.

Closes #42
```

```
fix: timer not resetting after long break

The timer was not properly resetting the next break time after
a long break completed. This fixes the issue by calling
reset_timer() at the end of end_break().
```

### Testing

#### Writing Tests
- Add unit tests for new functionality
- Test edge cases and error conditions
- Use descriptive test names

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_postpone_adds_correct_duration() {
        let config = Config::default();
        let timer = TimerService::new(config);
        let before = timer.time_until_short_break().await;

        timer.postpone_break(BreakType::Short).await;

        let after = timer.time_until_short_break().await;
        assert!(after > before);
    }
}
```

#### Running Tests
```bash
just test                    # Run all tests
cargo test -- --nocapture   # Show output
cargo test timer::tests     # Run specific module tests
```

### Documentation

- Update `README.md` for user-facing changes
- Update `CLAUDE.md` for developer-facing changes
- Add doc comments for public APIs
- Include examples in doc comments

## Pull Request Process

### Before Submitting
1. Ensure code compiles: `just build`
2. Run linter: `just check`
3. Format code: `just fmt`
4. Run tests: `just test`
5. Update documentation
6. Test the applet manually

### PR Description Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
Describe how you tested this

## Checklist
- [ ] Code compiles without errors
- [ ] Linter passes (just check)
- [ ] Tests pass (just test)
- [ ] Documentation updated
- [ ] Tested manually in COSMIC Desktop

## Screenshots (if applicable)
Add screenshots showing the changes
```

### Review Process
1. Maintainers will review your PR
2. Address any feedback or requested changes
3. Once approved, your PR will be merged
4. Your contribution will be credited in release notes

## Areas for Contribution

### Good First Issues
Look for issues labeled `good first issue`:
- Documentation improvements
- Small bug fixes
- Adding tests
- Code cleanup

### Feature Ideas
- Break exercises and animations
- Statistics tracking
- Calendar integration
- Notification sounds
- Multi-monitor support
- Plugin system
- Themes and customization

### Bug Fixes
Check the issue tracker for bugs to fix

## Architecture Overview

Quick overview for contributors:

```
cosmic-eyes/
├── src/
│   ├── main.rs          # Applet entry point, runs the COSMIC applet
│   ├── applet/          # Panel applet UI and state management
│   ├── break_screen/    # Break overlay window
│   ├── cli/             # Command-line interface
│   ├── config.rs        # Configuration loading/saving (RON format)
│   └── timer.rs         # Core timer logic with async/await
```

Key patterns:
- **MVU (Model-View-Update)**: UI pattern from iced/libcosmic
- **Async/Await**: Timer service uses tokio for async operations
- **Arc<RwLock>**: Thread-safe shared state
- **Messages**: User interactions converted to messages for state updates

## Communication

- **Issues**: For bug reports and feature requests
- **Pull Requests**: For code contributions
- **Discussions**: For questions and general discussion

## Recognition

All contributors will be:
- Listed in release notes
- Added to a CONTRIBUTORS file
- Credited in the about section of the app

Thank you for contributing to Cosmic Eyes! 👁️
