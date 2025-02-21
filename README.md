# Numerical Analysis - MATH 4640 - Georgia Tech

Code repository for homework and possibly other material for MATH 4640 - Numerical Analysis at Georgia Tech.

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
- solutions
  - python 
    - hw#
      - Dockerfile
      - environment.yml
      - hw#_sols.py
      - q#.py
      - README.md
  - rust
    - hw#
      - Dockerfile
      - Cargo.toml
      - Cargo.lock
      - src
      - target

## Running

Each homework solution is located in the `solutions/` directory. Simply build and run the container from the Dockerfile in each homework folder.

Example: `docker build -t hw1 .`

Once inside the container's shell, run `python -m s -q <question_module`. For example, `python -m s -q q3` would run the solution for question 3. Solutions are displayed in terminal.

Each homework solution directory has a README with more details on building and running.

