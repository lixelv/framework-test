from fastapi import FastAPI, Request

app = FastAPI()

@app.get('/get-ip')
async def return_ip(request: Request):
    return {"data": {"ip": request.client.host}}
