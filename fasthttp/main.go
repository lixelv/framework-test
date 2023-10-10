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
