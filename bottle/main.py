from bottle import route, run, request
import json

@route('/get-ip')
def return_ip():
    client_ip = request.environ.get('REMOTE_ADDR')
    return json.dumps({"data": {"ip": client_ip}})

run(host='127.0.0.1', port=9009)
