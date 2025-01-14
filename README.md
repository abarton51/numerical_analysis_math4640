# Numerical Analysis - MATH 4640 - Georgia Tech

Code repository for homework and possibly other material for MATH 4640 - Numerical Analysis at Georgia Tech.

## Structure

- latex
  - hw1
    - hw1.*
    - hw1.pdf
    - hw1.tex
    - math4640_hw1.pdf
  - hw2
  - hw3
  - hw4
  - template.tex
- LICENSE
- README.md
- solutions
  - hw1
    - Dockerfile
    - environment.yml
    - hw1_sols.py
    - q3.py
    - README.md
  - hw2
  - hw3
  - hw4

## Running

Each homework solution is located in the `solutions/` directory. Simply build and run the container from the Dockerfile in each homework folder.

Example: `docker build -t hw1 .`

Once inside the container's shell, run `python -m s -q <question_module`. For example, `python -m s -q q3` would run the solution for question 3. Solutions are displayed in terminal.

