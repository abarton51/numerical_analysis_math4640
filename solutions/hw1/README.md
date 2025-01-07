# `hw1_sols`

This module is designed as an entrypoint to run each solution. Each solution is a module titled something of the format `q1.py`, which would be the solution for question 1. To run `hw1_sols` as a package, simply run `python -m hw1_sols`. Running just this module does nothing however, as it looks for arguments following the `-q` flag. This flag stands for "questions". Simply pass in the modules for each question that you want to run (excluding the extension).

For example: `python -m hw1_sols -q q1 q2 q5` will run the solutions for questions 1, 2, and 5.

# Running without Docker

- Download zip file
- Unzip and open somewhere
- If you have conda, go to `hw1` and run `conda create env -f environment.yml`. This will create a conda environment titled `hw1` for this homework.
- Run `conda activate hw1` to activate environment.
- Run `python -m hw1_sols -q <question_modules>`. Replace `<question_modules>` with the actual module names (excluding the extension).

# Running with Docker

## Linux

- Download zip file
- Unzip
- Build Docker container: `docker build -t hw1 ~/path/to/hw1`
- Run Docker container interactively: `docker run -it hw1 /bin/bash`
- Run solution modules for each question. Example: `python -m hw1_sols -q q1`

## Windows

- Download zip file
- Unzip somewhere
- Open `hw1` with VSCode
- Build and Open container (you can use `Ctrl + Shift + P` and click ">Dev Containers: Rebuild Container"
- Run solution modules for each question. Example: `python -m hw1_sols -q q1`

## MacOS

I don't know. Sorry.
