package fields

import (
	model "server/models"

	"github.com/graphql-go/graphql"
)

var user = graphql.NewObject(
	graphql.ObjectConfig{
		Name: "User",
		Fields: graphql.Fields{
			"id":   &graphql.Field{Type: graphql.ID},
			"name": &graphql.Field{Type: graphql.String},
			"age":  &graphql.Field{Type: graphql.String},
		},
		Description: "Users data",
	},
)

func NewUsers() *graphql.Field {
	return &graphql.Field{
		Type: graphql.NewList(user),
		Resolve: func(p graphql.ResolveParams) (i interface{}, e error) {
			var u []*model.User = []*model.User{
				{
					Id:   "id1",
					Name: "User 1",
					Age:  15,
				},
				{
					Id:   "id2",
					Name: "User 2",
					Age:  32,
				},
				{
					Id:   "id2",
					Name: "User 3",
					Age:  24,
				},
			}

			return u, nil
		},
		Description: "user",
	}
}
