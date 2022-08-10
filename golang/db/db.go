package db

import "fmt"

var Dbref Db

type Db struct {
	client bool
}

func NewDb() Db {
	return Db{
		client: false,
	}
}

func (c Db) Create() {
	fmt.Println("client is", c.client)
}
