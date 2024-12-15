# Contributing to Jaafar
We’re excited that you’re interested in contributing to **Jaafar**! Your contributions help us improve and expand this project for everyone. Below are guidelines to help you get started.

## Code of Conduct
By participating in this project, you agree to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it to ensure a welcoming and inclusive environment for all contributors.

## How Can I Contribute?
### Reporting Bugs
If you find a bug:

1. Search [open issues](https://github.com/ilittlebig/jaafar/issues) to see if it has already been reported.
2. If not, create a new issue with the following details:
   - Clear and descriptive title.
   - Steps to reproduce the issue.
   - Expected and actual behavior.
   - Any screenshots or logs, if applicable.

### Suggesting Features

We welcome suggestions for new features! Please:

1. Search existing [feature requests](https://github.com/ilittlebig/jaafar/issues) to avoid duplicates.
2. Open a new issue and provide:
   - A clear and descriptive title.
   - Explanation of the feature and its benefits.
   - Any potential use cases or examples.

### Improving Documentation

If you notice incomplete, outdated, or unclear documentation:

- Open an issue or create a pull request with your proposed improvements.

### Contributing Code

We welcome code contributions for bug fixes, new features, and enhancements. Please follow the [Pull Request Process](#pull-request-process) below.


## Development Setup

1. **Fork the Repository**: Click the "Fork" button on the repository page.
2. **Clone Your Fork**: Clone your forked repository locally.
   ```bash
   git clone https://github.com/your-username/jaafar.git
   cd jaafar
   ````
3. **Install Dependencies**: Run the following command to install project dependencies.
   ```bash
   npm install
   ````
4. **Run the Project**: Start the application in development mode using Tauri.
   ```bash
   npm run tauri dev
   ````
5. **Run Tests**: Make sure all tests pass before submitting your changes.
   ```bash
   npm run test
   ````

## Pull Request Process
1. **Create a Branch**: Use a descriptive name for your branch, such as:
    ```bash
    git checkout -b feat/add-new-feature
    ````
2. **Make Your Changes**: Ensure your code is clean, well-documented, and adheres to the project’s coding standards.
3. **Run Tests**: Verify that all tests pass locally.
4. **Commit Your Changes**: Write a meaningful commit message.
    ```bash
    git commit -m "feat: add new feature description"
    ````
5. **Push to Your Fork**: Push your branch to your forked repository.
    ```bash
    git push origin feat/add-new-feature
    ````
6. **Open a Pull Request**: Go to the original repository and create a pull request. Include:
    - A clear title and description of the changes.
    - Reference to any related issues (e.g., Closes #123).

## License
By contributing, you agree that your contributions will be licensed under the same license as the project: [MIT License](LICENSE).
