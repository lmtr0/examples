package graphql

import (
	"server/graphql/fields"

	"github.com/graphql-go/graphql"
)

func newQuery() *graphql.Object {
	return graphql.NewObject(graphql.ObjectConfig{
		Name: "Query",
		Fields: graphql.Fields{
			"users": fields.NewUsers(),
		},
	})
}
