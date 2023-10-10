from numpy import sort
from requests import get
from icecream import ic
import time

def get_time(fn):
    def wrapper(*args, **kwargs):
        start_time = time.perf_counter()
        result = fn(*args, **kwargs)
        end_time = time.perf_counter()
        return end_time - start_time
    return wrapper

get = get_time(get)

if __name__ == '__main__':
    times_dict = {
        "flask": {
            "value": 0, 
            "url": "http://127.0.0.1:9002/get-ip"
        },

        "django": {
            "value": 0, 
            "url": "http://127.0.0.1:9001/get-ip"
        },

        "fastapi": {
            "value": 0, 
            "url": "http://127.0.0.1:8000/get-ip"
        },

        "nestjs": {
            "value": 0, 
            "url": "http://127.0.0.1:9003/get-ip"
        },

        "express": {
            "value": 0, 
            "url": "http://127.0.0.1:3000/get-ip"
        },
        
        "actix_web": {
            "value": 0,
            "url": "http://127.0.0.1:9004/get-ip" 
        },
        
        "rocket": {
            "value": 0,
            "url": "http://127.0.0.1:9005/get-ip" 
        },
        
        "tide": {
            "value": 0,
            "url": "http://127.0.0.1:9006/get-ip"
        },
        
        "warp": {
            "value": 0,
            "url": "http://127.0.0.1:9007/get-ip"
        },
        
        "cherypi": {
            "value": 0,
            "url": "http://127.0.0.1:9008/get-ip"
        },
        
        "bootle": {
            "value": 0,
            "url": "http://127.0.0.1:9009/get-ip"
        },
        
        "tornado": {
            "value": 0,
            "url": "http://127.0.0.1:9010/get-ip"
        },
        
        "pyramid": {
            "value": 0,
            "url": "http://127.0.0.1:9011/get-ip"
        },
        
        "web.py": {
            "value": 0,
            "url": "http://127.0.0.1:9014/get-ip"
        },
        
        "echo": {
            "value": 0,
            "url": "http://127.0.0.1:9015/get-ip"
        },
        
        "fiber": {
            "value": 0,
            "url": "http://127.0.0.1:9016/get-ip"
        },
        
        "fasthttp": {
            "value": 0,
            "url": "http://127.0.0.1:9017/get-ip"
        },
        
        "gorila": {
            "value": 0,
            "url": "http://127.0.0.1:9018/get-ip"
        },
        
        "asp.net": {
            "value": 0,
            "url": "http://127.0.0.1:5090/GetIp/get-ip"
        }
        
        
    }
    for key in times_dict.keys():     
        for _ in range(1000):   
            times_dict[key]["value"] += get(times_dict[key]["url"])
        
ks = list(times_dict.keys())
ks.sort(key=lambda x: times_dict[x]["value"])

print(ks)

ic([(key, times_dict[key]["value"]) for key in ks])