FROM python:3.12

WORKDIR /python-docker

COPY requirements.txt requirements.txt
RUN pip install --no-cache-dir --upgrade -r requirements.txt

COPY . .

CMD ["python", "-m" , "flask", "run", "--host=0.0.0.0"]