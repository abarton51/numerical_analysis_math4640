FROM continuumio/miniconda3:latest

WORKDIR /home/solutions

COPY environment.yml .

RUN conda env update --name base --file environment.yml --prune

COPY . .

CMD ["python", "-m", "hw4_sols"]

