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
