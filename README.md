# Numerical Analysis - MATH 4640 - Georgia Tech

Code repository for homework and possibly other material for MATH 4640 - Numerical Analysis at Georgia Tech. This is a Rust project which implements various introductory numerical methods.

## Structure

- latex
  - hw#
    - hw#.*
    - hw#.pdf
    - hw#.tex
    - math4640_hw#.pdf
  - template.tex
- LICENSE
- README.md
- sols
  - hw#
    - Dockerfile
    - Cargo.toml
    - Cargo.lock
    - src

## Running

Each homework solution is located in the `sols/` directory. Simply build and run the container from the Dockerfile in each homework folder.

Example: `docker build -t hw1 . && docker run hw1:latest`

Each homework solution directory has a README with more details on building and running.

