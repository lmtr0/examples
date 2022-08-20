package main

import (
	"net/http"

	"github.com/labstack/echo/v4"
)

func Welcome() echo.HandlerFunc {
	return func(c echo.Context) error {
		return c.String(http.StatusOK, "Welcome!")
	}
}
