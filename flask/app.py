from flask import Flask, request, jsonify
import time

app = Flask(__name__)

@app.route('/get-ip', methods=['GET'])
def get_ip():
    ip = request.headers.get('X-Forwarded-For', request.remote_addr)
    return jsonify({'data': {'ip': ip}})

if __name__ == '__main__':
    app.run(port=9002)
