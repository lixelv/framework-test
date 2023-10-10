import web
import json

urls = (
    '/get-ip', 'get_ip'
)

app = web.application(urls, globals())

class get_ip:
    def GET(self):
        client_ip = web.ctx['ip']
        web.header('Content-Type', 'application/json')
        return json.dumps({"data": {"ip": client_ip}})

if __name__ == "__main__":
    web.httpserver.runsimple(app.wsgifunc(), ("0.0.0.0", 9014))
