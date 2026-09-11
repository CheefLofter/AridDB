

from flask import Flask, render_template, send_from_directory
import os

app = Flask(__name__)

@app.route("/")
def hello_world():
    return render_template("index.html")

@app.route("/docs")
def docs():
    return render_template("docs.html")

@app.route("/assets/<path:filename>")
def asset_file(filename):
    return send_from_directory(os.path.join(app.root_path, "Assets"), filename)







if __name__ == ("__main__"):
    app.run(debug=True)
