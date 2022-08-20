package graphql

import (
	"fmt"

	"github.com/graphql-go/graphql"
	"github.com/graphql-go/handler"
)

func NewHandler() (*handler.Handler, error) {
	schema, err := graphql.NewSchema(
		graphql.SchemaConfig{
			Query: newQuery(),
		},
	)
	if err != nil {
		fmt.Println("Failed to initialize graphql server")
		return nil, err
	}

	fmt.Println("initialized graphql server")
	return handler.New(&handler.Config{
		Schema: &schema,
		Pretty: true,
		// GraphiQL: true,
		Playground: true,
	}), nil
}
