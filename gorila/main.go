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
