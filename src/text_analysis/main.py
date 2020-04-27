from flask import Flask, jsonify, request
from words_frequency import get_words_frequency
from keywords import get_keywords

app = Flask(__name__)

@app.route('/freq', methods=['POST'])
def freq():
    if request.method == 'POST':
        if request.json:
            req_json = request.json
            word_frequency = get_words_frequency(req_json['text'])
            return jsonify(word_frequency)

# @app.route('/keywords', methods=['POST'])
# def keywords():
#     if request.method == 'POST':
#         if request.json:
#             req_json = request.json
#             keywords = get_keywords(req_json['text'])
#             return jsonify([])

