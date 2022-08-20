package main

import (
	"fmt"
	"net/http"
	"server/graphql"

	"github.com/labstack/echo/v4"
)

func main() {
	// Echo instance
	e := echo.New()

	// Routes
	e.GET("/", Welcome())

	// graphql handler
	fmt.Println("Graphql Endpoint")
	graphql.Route(e)
	fmt.Println("Ended Graphql Endpoint")

	// Start server
	e.Logger.Fatal(e.Start(":3000"))
}

// Handler
func hello(c echo.Context) error {
	return c.String(http.StatusOK, "Hello, World!")
}
