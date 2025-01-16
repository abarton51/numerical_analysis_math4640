# Numerical Analysis Homework

This repository contains solutions to numerical analysis homework questions in Rust.

## Setup

1. Install Docker.
2. Clone this repository.
3. Build the Docker image:

    ```bash
    docker build -t rust-hw1 .
    ```

4. Run a specific question:

    ```bash
    docker run --rm rust-hw1 -q q1
    docker run --rm rust-hw1 -q q2
    docker run --rm rust-hw1 -q q3
    ```

## Structure

- `src/main.rs`: The entry point to run questions.
- `src/q1.rs`, `src/q2.rs`, etc.: Solutions for each question.

