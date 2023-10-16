# Global test of frameworks speed
I made a big web framework test. I used 19 frameworks for the test. And I made test programms on python and rust, results is here:

**Here is result of making 1000 requests on framework server:**
| Framework  | Time (Python)  | Time (Rust)   |
|------------|----------------|---------------|
| fiber      | 0.5874 s       | 0.7943 s      |
| fasthttp   | 0.5944 s       | 0.7584 s      |
| warp       | 0.6317 s       | 0.8325 s      |
| gorilla    | 0.6454 s       | 0.7623 s      |
| echo       | 0.6483 s       | 0.9080 s      |
| actix_web  | 0.6773 s       | 0.8807 s      |
| asp.net    | 0.7052 s       | 0.7990 s      |
| pyramid    | 0.7240 s       | 1.0209 s      |
| bottle     | 0.7282 s       | 0.9464 s      |
| tide       | 0.7376 s       | 0.9205 s      |
| nestjs     | 0.7749 s       | 0.9785 s      |
| express    | 0.7993 s       | 1.0332 s      |
| tornado    | 0.8386 s       | 1.0244 s      |
| fastapi    | 0.8851 s       | 1.1021 s      |
| rocket     | 0.9277 s       | 1.0909 s      |
| flask      | 1.0108 s       | 1.2820 s      |
| django     | 1.0276 s       | 1.2571 s      |
| cherypi    | 1.3544 s       | 1.5128 s      |
| web.py     | 4.8448 s       | 4.8946 s      |

And now lets go into the details. 1-st of all I'll explain everything about language and frameworks:

## Go
## Rust
## Python
## TypeScript
## JavaScript
## C#