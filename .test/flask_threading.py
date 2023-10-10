from threading import Thread
from requests import get as gt
from icecream import ic
import time

def get_time(fn):
    def wrapper(*args, **kwargs):
        start_time = time.perf_counter()
        result = fn(*args, **kwargs)
        end_time = time.perf_counter()
        return end_time - start_time
    return wrapper

def get(url):
    ic(get_time(gt)(url))

tasks = []
for _ in range(5):
    tasks.append(Thread(target=get, args=('http://127.0.0.1:9002/get-ip',)))
    
for task in tasks:
    task.start()
    
