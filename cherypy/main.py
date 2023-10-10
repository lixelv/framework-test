import cherrypy

class ReturnIP(object):
    @cherrypy.expose
    @cherrypy.tools.json_out()
    def get_ip(self):
        return {"data": {"ip": cherrypy.request.remote.ip}}

if __name__ == '__main__':
    cherrypy.config.update({'server.socket_host': '0.0.0.0', 
                            'server.socket_port': 9008})
    cherrypy.quickstart(ReturnIP(), '/get-ip')
