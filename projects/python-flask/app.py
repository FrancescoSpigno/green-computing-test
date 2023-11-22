from flask import Flask
from flask import send_from_directory

app = Flask(__name__)

@app.route("/small")
def small():
    return send_from_directory('../../file-test', 'small.json')

@app.route("/medium")
def medium():
    return send_from_directory('../../file-test', 'medium.json')

@app.route("/large")
def large():
    return send_from_directory('../../file-test', 'large.json')

