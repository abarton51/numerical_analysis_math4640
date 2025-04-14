# `main.rs`

This module is the *crate root* and serves as the entry point for running the solutions to the homework questions. Each question's solution is contained in a separate module name `q<number>.rs` where `<number>` is the nummber of the question.

For example, `q3.rs` contains the solution for question 3. The main module (`main.rs`) provides functionality to run specific solutions by passing the question identifier as an argument, e.g., `cargo run q3 q5` to run the solution for questions 3 and 5.

---

# File to Questions Mapping


---

# Requirements

## Without Docker

1. Install Rust: [Rust installation guide](https://www.rust-lang.org/tools/install)

## With Docker

1. Install Docker: [Docker installation guide](https://docs.docker.com/engine/install/)

---

# Running The Code

## Running without Docker

- Download zip file containing the project
- Unzip and open the project folder
  - I ship the binary targets with my code. You can build by `cargo build --release` if you'd like.
- Run a specific solution for a question. For example, to run the solution(s) for questions 4 and 5: `cargo run q4 q5`

## Running with Docker

### Linux

- Download and unzip the project zip file
- Build Docker container: `docker build -t <container_name> ~/path/to/solutions/hw4`
- Run Docker container: `docker run <container_name>`
- Solutions will be displayed in terminal

### Windows and MacOS

I don't use the OS's currently, but these steps should help:

- **With VS Code and Docker Desktop**:

1. Install Docker Desktop and ensure it's running.
2. Open the project in VS Code.
3. Install the Docker extension in VS Code.
4. Use the "Remote-Containers" extension to build a dev container (use `Ctrl+Shift+P` for these options)
5. Once the container is running, use the integrated terminal to run the container and execute the solutions.

Note for Windows users: WSL 2 is recommended.

