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
