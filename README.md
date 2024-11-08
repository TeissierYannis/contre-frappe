Voici la documentation mise à jour, incluant les liens pour Bevy.

---

# Project Setup Guide

## Prerequisites

1. **Install Rust**: Ensure you have Rust installed. You can install it using [Rustup](https://rustup.rs/):
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. **Verify Installation**:
    ```bash
    rustc --version
    cargo --version
    ```

3. **Dependencies**: This project uses the Bevy game engine. To ensure compatibility, review the [Bevy documentation](https://bevyengine.org/).

## Cloning the Project

Clone the repository locally:
```bash
git clone <repository-url>
cd <project-directory>
```

## Building the Project

To build the project:
```bash
cargo build
```

For a release build:
```bash
cargo build --release
```

## Running the Project

Run the project using:
```bash
cargo run
```

For a release run:
```bash
cargo run --release
```

## Development Tools

- **cargo check**: Quickly check the code for errors without building binaries.
    ```bash
    cargo check
    ```

- **cargo fmt**: Format code based on Rust conventions.
    ```bash
    cargo fmt
    ```

- **cargo clippy**: Catch common mistakes with the Rust linter.
    ```bash
    cargo clippy
    ```

## Useful Links

### Rust Resources
- [Official Rust Book](https://doc.rust-lang.org/book/): A comprehensive guide to learning Rust.
- [Crates.io](https://crates.io/): The Rust package registry, where you can explore libraries.
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/): Practical Rust examples to improve familiarity.
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/): Detailed Rust standard library documentation.

### Bevy Resources
- [Bevy Official Website](https://bevyengine.org/): Bevy's official site, with news and updates.
- [Bevy Documentation](https://docs.rs/bevy/): Detailed documentation for the Bevy game engine.
- [Bevy GitHub Repository](https://github.com/bevyengine/bevy): The GitHub repository for Bevy, where you can report issues or contribute.
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/): A comprehensive guide and reference for Bevy’s main features.

## Troubleshooting

If you encounter issues with dependencies, try updating them:
```bash
cargo update
```

For further support, refer to the [Rust documentation](https://doc.rust-lang.org/) or [Bevy documentation](https://bevyengine.org/).
