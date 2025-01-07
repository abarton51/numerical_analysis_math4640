FROM continuumio/miniconda3:latest

WORKDIR /home/solutions

COPY environment.yml .

RUN conda env create -f environment.yml

SHELL ["conda", "run", "-n", "numerical_analysis", "/bin/bash", "-c"]

COPY . .

CMD ["python", "-m", "hw1_sols"]

