package main

import (
	"fmt"
	"log"
	"net/http"
	api "server/api"
	"time"

	db "server/db"

	"github.com/gin-gonic/gin"
	"github.com/go-co-op/gocron"
)

type User struct {
	age  int
	name string
}

func main() {
	db.Dbref = db.NewDb()

	fmt.Println("Hello, world!")

	user := User{
		age:  60,
		name: "hello world",
	}

	fmt.Println(user.name)

	router := gin.Default()

	hello := map[string]int{
		"countrer": 0,
	}

	router.GET("health", func(ctx *gin.Context) {
		hello["counter"] += 1
		ctx.IndentedJSON(http.StatusNotFound, gin.H{"status": "OK"})
	})

	log.Println("hello is ", hello)

	api.Route(router)
	api.AuthRoute(router)

	s := gocron.NewScheduler(time.UTC)
	s.Every("5s").Do(func() {
		fmt.Println("running scheduled tasked every 5s")
	})

	go s.StartAsync()
	router.Run("localhost:3000")
}
