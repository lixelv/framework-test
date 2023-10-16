# Global Test of Web Frameworks' Speed

In this repository, you'll find the results of a comprehensive web framework test. The test includes 19 different frameworks, and the programs were written in both Python and Rust. The scence of my web-apps is to return json with my id address, on fastapi it seems like this:
```py
from fastapi import FastAPI, Request

app = FastAPI()

@app.get('/get-ip')
async def return_ip(request: Request):
    return {"data": {"ip": request.client.host}}
```

---

## 🚀 Results

### Making 1,000 requests to each web framework server

| Framework  | Time (Python)  | Time (Rust)   |
|------------|----------------|---------------|
| [fiber](#fiber)      | 0.5874 s       | 0.7943 s      |
| [fasthttp](#fasthttp)   | 0.5944 s       | 0.7584 s      |
| [warp](#warp)       | 0.6317 s       | 0.8325 s      |
| [gorilla](#gorilla)    | 0.6454 s       | 0.7623 s      |
| [echo](#echo)       | 0.6483 s       | 0.9080 s      |
| [actix_web](#actix_web)  | 0.6773 s       | 0.8807 s      |
| [asp.net](#asp.net)   | 0.7052 s       | 0.7990 s      |
| [pyramid](#pyramid)    | 0.7240 s       | 1.0209 s      |
| [bottle](#bottle)     | 0.7282 s       | 0.9464 s      |
| [tide](#tide)       | 0.7376 s       | 0.9205 s      |
| [nestjs](#nestjs)     | 0.7749 s       | 0.9785 s      |
| [express](#express)    | 0.7993 s       | 1.0332 s      |
| [tornado](#tornado)    | 0.8386 s       | 1.0244 s      |
| [fastapi](#fastapi)    | 0.8851 s       | 1.1021 s      |
| [rocket](#rocket)     | 0.9277 s       | 1.0909 s      |
| [flask](#flask)      | 1.0108 s       | 1.2820 s      |
| [django](#django)     | 1.0276 s       | 1.2571 s      |
| [cherypy](#cherrypy)    | 1.3544 s       | 1.5128 s      |
| [web.py](#web.py)    | 4.8448 s       | 4.8946 s      |

---
# [Go](#Go)
## - [fiber](#fiber)
## - [fasthttp](#fasthttp)
## - [gorilla](#gorilla)
## - [echo](#echo)


# [Rust](#Rust)
## - [actix_web](#actix_web)
## - [warp](#warp)
## - [tide](#tide)
## - [rocket](#rocket)


# Python
## [fastapi](#fastapi)

---

## 📚 In-Depth Analysis

Now let's dive into the details. First, I will explain the languages and frameworks used in this test.

# 🐹 Go
## 📚 Base
🐹 Developed by Google, Go (or Golang) is a statically-typed, compiled language designed for simplicity and efficiency. It's the go-to language (pun intended!) for web backends, particularly because of its straightforward syntax, high performance, and strong support for concurrent programming.

### Pros:
- Simplicity: Go's syntax is easy to read and write. You can get a web server up and running in no time, making it great for startups and agile development.

- Performance: Compiled to native code, Go is blisteringly fast. It’s often compared to languages like Java for speed but has the simplicity of languages like Python.

- Concurrency: Go's built-in concurrency model, based on goroutines and channels, makes it well-suited for scalable, high-performance web servers and real-time systems.

- Static Typing: With static types, you catch errors early and make your code easier to understand and refactor, leading to more maintainable projects.

- Robust Standard Library: Go has a rich standard library that covers a lot of areas, from web servers to cryptography to data manipulation.

### Cons:
- Lack of Generics: Although planned for future versions, the current lack of generics can make data structures and algorithms a bit verbose to implement.
```go
package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
```
## 🌐 [fiber](https://github.com/gofiber/fiber) <a id="fiber"></a>
🚀 Fiber is a Go web framework designed to simplify development, similar in API to Express.js. It offers high performance and a range of convenient features for developers. Here you can see the main code of fiber app:
```go
package main

import (
	"github.com/gofiber/fiber/v2"
)

func getIP(c *fiber.Ctx) error {
	ip := c.IP()
	return c.JSON(fiber.Map{"data": fiber.Map{"ip": ip}})
}

func main() {
	app := fiber.New()
	app.Get("/get-ip", getIP)
	app.Listen(":9016")
}
```

## 🌐 [fasthttp](https://github.com/valyala/fasthttp) <a id="fasthttp"></a>
🔥 Fasthttp is another Go-based web framework that is known for its high performance. It's optimized for speed and low memory usage, which makes it useful for scalable applications. Here is the code of fasthttp:
```go
package main

import (
	"fmt"
	"log"

	"github.com/valyala/fasthttp"
)

func main() {
	if err := fasthttp.ListenAndServe(":9017", requestHandler); err != nil {
		log.Fatalf("Error in ListenAndServe: %s", err)
	}
}

func requestHandler(ctx *fasthttp.RequestCtx) {
	switch string(ctx.Path()) {
	case "/get-ip":
		ip := ctx.RemoteIP().String()
		fmt.Fprintf(ctx, `{"data": {"ip": "%s"}}`, ip)
	}
}
```

## 🌐 [gorilla](https://github.com/gorilla/mux) <a id="gorilla"></a>
🦍 Gorilla is a minimalistic web framework for Go that provides just the most essential libraries and features, allowing for more flexibility in how you structure your app. Here is ip api on this framework:
```go
package main

import (
	"encoding/json"
	"net/http"

	"github.com/gorilla/mux"
)

func main() {
	r := mux.NewRouter()
	r.HandleFunc("/get-ip", GetIPHandler)
	http.ListenAndServe(":9018", r)
}

func GetIPHandler(w http.ResponseWriter, r *http.Request) {
	ip := r.RemoteAddr
	response := map[string]map[string]string{"data": {"ip": ip}}
	json.NewEncoder(w).Encode(response)
}
```

## 🌐 [echo](https://github.com/labstack/echo) <a id="echo"></a>
🔊 Echo is a Go framework that is designed for building web applications and APIs. It's lightweight, modular, and comes with features like middleware support. app:
```go
package main

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func getIP(c echo.Context) error {
	ip := c.RealIP()
	return c.JSON(http.StatusOK, map[string]interface{}{"data": map[string]string{"ip": ip}})
}

func main() {
	e := echo.New()
	e.GET("/get-ip", getIP)
	e.Start(":9015")
}
```
## 🧮 Summarizing
Go is have very simple syntax, also go is language with fastest web frameworks. Go is really good choice for small services and api. This is really good language for webdev, so if you are searching for fast, easy framework, here they are!

# 🦀 Rust
## 📚 Base
🦀 Rust is a systems programming language that empowers you to build reliable and efficient software, free from the common pitfalls of memory-related bugs. Developed by Mozilla, Rust is designed with safety, concurrency, and practicality in mind.

## Pros:
- Zero-Cost Abstractions: Rust gives you the low-level control of C and C++ but with high-level comforts. You can write blazingly fast code without compromising on elegance or readability.

- Memory Safety: One of Rust's killer features is its ownership system, which eliminates null pointer dereferencing, data races, and ensures thread safety. Say goodbye to segmentation faults!

- Rich Type System: With its expressive type system, including algebraic data types like enum and match, Rust allows for highly maintainable and self-documenting code.

- Fearless Concurrency: Rust’s concurrency model ensures thread safety, making it a key player in building highly concurrent systems like web servers and database engines.

- Growing Ecosystem: Its package manager, Cargo, and a growing ecosystem of libraries make Rust a complete package for any kind of software development.

## Cons:
- Learning Curve: The language's emphasis on safety and zero-cost abstractions may initially feel constraining, but it pays off by preventing numerous types of bugs.

## 🌐 [actix_web](https://github.com/actix/actix-web) <a id="actix_web"></a>
🦀 Actix Web is a Rust framework that is highly concurrent and provides a range of features for building web applications. It is known for its performance and type safety. Here is the code:
```rs
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn return_ip(req: HttpRequest) -> impl Responder {
    let ip = req.peer_addr().unwrap().ip();
    format!("{{\"data\": {{\"ip\": \"{}\"}}}}", ip)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/get-ip", web::get().to(return_ip))).bind("127.0.0.1:9004")?.run().await
}
```

## 🌐 [warp](https://github.com/nirex0/Warp-Framework) <a id="warp"></a>
🌐 Warp is a web framework for Rust that focuses on composability and safety. It's designed to work seamlessly with the Rust's async features. Here is the code:
```rs
use warp::Filter;
use serde_json::json;

#[tokio::main]
async fn main() {
    // Making filter for path /get-ip
    let get_ip = warp::path("get-ip")
        .and(warp::addr::remote())  // Getting IP
        .map(|ip: Option<std::net::SocketAddr>| {
            // Making Ip json
            let ip_str = ip.map(|sock_addr| sock_addr.ip().to_string()).unwrap_or_default();
            warp::reply::json(&json!({ "data": { "ip": ip_str }}))
        });

    // Lunch server on 9007 port
    warp::serve(get_ip)
        .run(([127, 0, 0, 1], 9007))
        .await;
}
```
## 🌐 [tide](https://github.com/http-rs/tide) <a id="tide"></a>
🌊 Tide is a minimal Rust framework that aims for a modular design. It is built on top of Rust's async ecosystem and is good for building APIs. Here is the code:
```rs
use tide::{Request, prelude::*};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/get-ip").get(|req: Request<()>| async move {
        let ip = req.remote().unwrap().to_string();
        let response_body = json!({ "data": { "ip": ip } });
        Ok(response_body)
    });
    app.listen("127.0.0.1:9006").await?;
    Ok(())
}
```

## [rocket](https://github.com/SergioBenitez/Rocket) <a id="rocket"></a>
🚀 Rocket is a web framework for Rust that focuses on ease-of-use, expressibility, and speed. It offers a wide range of built-in features and is type-safe. Here is the code:
```rs
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::response::status;
use std::net::SocketAddr;

#[get("/get-ip")]
fn return_ip(remote_addr: Option<SocketAddr>) -> status::Custom<String> {
    match remote_addr {
        Some(addr) => status::Custom(Status::Ok, format!("{{\"data\": {{\"ip\": \"{}\"}}}}", addr.ip())),
        None => status::Custom(Status::InternalServerError, "Could not retrieve IP address.".to_string()),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![return_ip]).launch();
}
```

## 🧮 Summarizing
Rust offers the best of both worlds: the performance of C/C++ and the safety of higher-level languages. Its ecosystem is growing, making it ideal not only for systems programming but also for web development. With features like memory safety and zero-cost abstractions, Rust is the future of efficient, reliable software. If you're looking to write high-performance applications without the headaches of common bugs and vulnerabilities, Rust is the way to go.

# 🐍 Python
## 📚 Base
Python is a high-level, interpreted programming language known for its readability and versatility. Created by Guido van Rossum and first released in 1991, Python has become a staple in web development, data science, artificial intelligence, and much more.

## Pros:
- Readability: Python's syntax is clean and easy to understand, making it excellent for beginners and experts alike.

- Versatility: From web development to machine learning, Python's wide range of frameworks and libraries make it incredibly versatile.

- Community Support: A strong community and a plethora of third-party libraries mean you're never alone when you're coding in Python.

- Productivity: Python allows for rapid development, which is great for prototype development as well as other ad-hoc programming tasks.

- Platform-Agnostic: Python is cross-platform, enabling you to work seamlessly across major operating systems.

## Cons:
- Speed: Being an interpreted language, Python is generally slower than compiled languages like C++ or Rust.

- Global Interpreter Lock (GIL): Concurrency is not Python’s strongest point due to the GIL, although this is being mitigated in newer versions and through specific libraries.

## 🌐 [fastapi](https://github.com/tiangolo/fastapi) <a id="fastapi"></a>
🚀 1-st things 1-st I want to talk about my favorite framework, fastapi. This is the simplest framework that I've ever seen. Also this framework supports one of my projects (tg bot, that is controlling the whole computer class, 1 button, and I can power up or down all pc-s). And the best sides of this framework is: 
- speed(for python)
- simple
- supports websockets and asynchrony


And 'couse of it this is me favorite framework. Here is code from the start:
```py
from fastapi import FastAPI, Request

app = FastAPI()

@app.get('/get-ip')
async def return_ip(request: Request):
    return {"data": {"ip": request.client.host}}
```

## 🌐 [flask](https://github.com/pallets/flask) <a id="flask"></a>
🌶️ I think that Flask must rip. He doesn't support asynchrony and websockets, he is slower than the fastapi. On this framework was writed a lot, so I want to wish you rest in peace. Here is the requiem:
```py
from flask import Flask, request, jsonify

app = Flask(__name__)

@app.route('/get-ip', methods=['GET'])
def get_ip():
    ip = request.headers.get('X-Forwarded-For', request.remote_addr)
    return jsonify({'data': {'ip': ip}})

if __name__ == '__main__':
    app.run(port=9002)
```
## 🌐 [django](https://github.com/django/django) <a id="django"></a>
🎸 Django is a high-level Python framework that encourages rapid development. It has so much different things, like db working, frontenddev, and mush more! But django is too big for some small api-s, but for complete sites it is that is needed. Here is the code:
```py
from django.http import JsonResponse
from django.views import View

class GetIPView(View):
    def get(self, request, *args, **kwargs):
        ip = get_client_ip(request)
        return JsonResponse({"data": {'ip': ip}})

def get_client_ip(request):
    x_forwarded_for = request.META.get('HTTP_X_FORWARDED_FOR')
    if x_forwarded_for:
        ip = x_forwarded_for.split(',')[0]
    else:
        ip = request.META.get('REMOTE_ADDR')
    return ip
```
## 🌐 [cherrypy](https://github.com/cherrypy/cherrypy) <a id="cherrypy"></a>
🍒 CherryPy is an object-oriented web application framework using the Python programming language. It's designed for rapid development of web applications by wrapping HTTP protocol. Here is cherrypy code:
```py
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
```

## 🌐 [bottle](https://github.com/bottlepy/bottle) <a id="bottle"></a>
🍼 Bottle is a simple and lightweight WSGI micro web-framework for Python. It's useful for small applications and rapid developmen. Here is the code on bottle:)
```pt
from bottle import route, run, request
import json

@route('/get-ip')
def return_ip():
    client_ip = request.environ.get('REMOTE_ADDR')
    return json.dumps({"data": {"ip": client_ip}})

run(host='127.0.0.1', port=9009)
```

## 🌐 [pyramid](https://github.com/Pylons/pyramid) <a id="pyramid"></a>
🔺 Pyramid is a Python web framework aimed at taking small web apps into big web apps. It's flexible and modular. Here is code:
```py
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
```

## 🌐 [tornado](https://www.tornadoweb.org/en/stable/webframework.html) <a id="tornado"></a>
🌪️ Tornado is a Python web framework and asynchronous networking library. It's non-blocking and is designed to handle long-lived network connections. Here is code:
```py
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
```

## 🌐 [web.py](https://github.com/webpy/webpy) <a id="web.py"></a>
🕸️ Web.py is a minimalist web framework for Python. It's designed to be simple and versatile, enabling you to get a web app up and running with minimal lines of code. But is is sooo slow, that better use fastapi) Code:
```py
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
```
# 📘 TypeScript
## 📚 Base
TypeScript is a strongly typed superset of JavaScript that brings static typing to the often dynamic world of JS. Developed by Microsoft, TypeScript aims to make large codebases more manageable and less error-prone.

## Pros:
- Type Safety: Introduces static typing, helping to catch errors at compile-time rather than runtime.

- Better Tooling: Enhanced autocompletion, type checking, and source code documentation.

- Easy Transition: Being a superset of JavaScript, it's easy to convert an existing JS project to TypeScript.

- Strong Community: A rapidly growing community and wide industry adoption offer a lot of resources and libraries.

## Cons:
- Learning Curve: Requires understanding of types, interfaces, and other new syntax which may take time to get used to.

- Compilation: Adds an extra step to the development process, as TypeScript code has to be compiled to JavaScript.

## 🌐 [nestjs](https://github.com/nestjs/nest) <a id="nestjs"></a>
🐣 NestJS is a framework for building server-side applications. It uses TypeScript and is built with extensibility in mind, allowing for a modular architecture. Code:
```ts
import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  await app.listen(9003);
}
bootstrap();
```

## 🌐 [express](https://github.com/expressjs/express) <a id="express"></a>
🚂 Express.js is a fast, unopinionated, minimalist web framework for Node.js, widely used for building web applications and APIs. Code:
```ts
import express, { Request, Response } from 'express';

const app = express();

app.get('/get-ip', (req: Request, res: Response) => {
    const ip = req.headers['x-forwarded-for'] || req.connection.remoteAddress;
    res.json({data: {ip: ip}});
});

app.listen(3000, () => {
    console.log('Server is running on port 3000');
});
```

# 🌐 C#
## 📚 Base
C# is a statically-typed language developed by Microsoft, mainly intended for creating Windows desktop applications and games through the Unity engine. It has since grown to serve various applications including web and mobile.

## Pros:
Strongly Typed: Type-safety reduces a lot of runtime errors, making the code more robust.

- Versatile: Great for desktop, web, and game development, especially in the .NET ecosystem.

- Language Features: Offers features like LINQ, async/await, and more that help in writing clean, maintainable code.

- Strong IDE Support: Excellent tooling with IDEs like Visual Studio provides a powerful development environment.

## Cons:
- Platform Dependence: Traditionally tied to the Windows ecosystem, although .NET Core is changing this.

- Verbosity: The syntax can be more verbose compared to dynamically-typed languages, leading to longer development time.

## 🌐 [asp.net](https://github.com/nirex0/Warp-Framework) <a id="asp.net"></a>
🌐 ASP.NET is a web framework from Microsoft, designed for building robust, scalable applications on the .NET platform. It is written in C#. Code:
```cs
using Microsoft.AspNetCore.Mvc;
using System.Net;

namespace asp.net.Controllers
{
    [ApiController]
    [Route("[controller]")]
    public class GetIpController : ControllerBase
    {
        [HttpGet("get-ip")]
        public IActionResult GetIp()
        {
            var ip = HttpContext.Connection.RemoteIpAddress.ToString();
            return Ok(new { data = new { ip = ip } });
        }
    }
}
```