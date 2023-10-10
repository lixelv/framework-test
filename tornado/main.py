import tornado.ioloop
import tornado.web

class ReturnIPHandler(tornado.web.RequestHandler):
    def get(self):
        self.write({"data": {"ip": self.request.remote_ip}})

def make_app():
    return tornado.web.Application([
        (r"/get-ip", ReturnIPHandler),
    ])

if __name__ == "__main__":
    app = make_app()
    app.listen(9010)
    tornado.ioloop.IOLoop.current().start()
