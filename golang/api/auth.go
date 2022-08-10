package api

import (
	db "server/db"

	"github.com/gin-gonic/gin"
)

func authRequired() gin.HandlerFunc {
	return func(c *gin.Context) {
		// do user authentication
		c.Set("user", gin.H{
			"userid": "u_isasdasd",
		})
		c.Next()
	}
}

func AuthRoute(r *gin.Engine) {

	authorized := r.Group("/auth")
	// per group middleware! in this case we use the custom created
	// AuthRequired() middleware just in the "authorized" group.
	authorized.Use(authRequired())
	authorized.POST("", func(c *gin.Context) {
		db.Dbref.Create()

		user := c.MustGet("user").(gin.H)
		c.JSON(200, gin.H{
			"user": user,
		})
	})
}
