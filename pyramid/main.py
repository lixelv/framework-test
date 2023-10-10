from wsgiref.simple_server import make_server
from pyramid.config import Configurator
from pyramid.response import Response
import json

def return_ip(request):
    client_ip = request.remote_addr
    return Response(json.dumps({"data": {"ip": client_ip}}))

if __name__ == '__main__':
    with Configurator() as config:
        config.add_route('get-ip', '/get-ip')
        config.add_view(return_ip, route_name='get-ip')
        app = config.make_wsgi_app()
    server = make_server('0.0.0.0', 9011, app)
    server.serve_forever()
