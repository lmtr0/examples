package graphql

import (
	"log"

	"github.com/labstack/echo/v4"
)

func Route(e *echo.Echo) {
	h, err := NewHandler()
	if err != nil {
		log.Fatal(err)
	}

	e.POST("/gql", echo.WrapHandler(h))
	e.GET("/gql", echo.WrapHandler(h))
}
