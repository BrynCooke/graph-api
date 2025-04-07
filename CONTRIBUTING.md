# Contributing Guidelines

Thank you for your interest in contributing to this project! This document outlines the basic guidelines for
contributing.

## Filing Issues

Issues are welcome to be filed. They **must** include a minimal reproduction using SimpleGraph.

I have a full time job not working on this library, so if you want to see things
happen then please make a contribution. I work for cold hard cash ;-).

## Requirements for Code Contributions

### 1. Testing Requirements

All contributions **must** include:

- Unit tests for new functionality
- Update existing tests if modifying current functionality
- Tests must pass in the CI pipeline

### 2. Code Quality

- Follow the project's coding style and conventions
- Write clear, documented code
- Updated docs where needed
- Ensure your code compiles without warnings
- Linting must pass in the CI pipeline

### 3. Documentation Guidelines

#### General Documentation

- Keep documentation clear, concise, and accurate
- Use examples to illustrate concepts
- Update existing documentation when changing functionality
- All public APIs should have documentation

#### Walker Documentation

- Each walker step must include visual diagrams to help users understand what the step does
- Diagrams should clearly illustrate the before and after state of the traversal
- Use consistent notation across all walker step documentation
- Explain each parameter and its effect on the traversal flow
- Include code examples showing the step in isolation and in combination with other steps

### 4. Pull Request Process

1. Fork the repository
2. Create a new branch for your feature/fix
3. Write your code and tests
4. Ensure all tests pass locally (`just test`)
5. Submit a Pull Request with:
    - Clear description of changes
    - Reference to any related issues
    - Explanation of test coverage

### 5. Test Guidelines

- Each new function or feature must have corresponding tests
- Test both success and failure cases
- Include edge cases in your test scenarios
- Document complex test scenarios

## Getting Started

1. Clone the repository
2. Install development dependencies:
    1. `cargo install just`
    2. `cargo install cargo-nextest`
3. Run `just test` to ensure everything works
4. Create your branch and start coding

## Questions?

If you have questions about contributing, please open an issue in the repository.