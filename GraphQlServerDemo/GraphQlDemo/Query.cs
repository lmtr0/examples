using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Http;
using GraphQlDemo.Data;
using GraphQlDemo.Repository;
using HotChocolate;
using HotChocolate.Utilities;

namespace GraphQlDemo.Queries
{
    public class Query
    {

        // Can be an async task of list of books
        public List<Book> AllBooks([Service]IHttpContextAccessor httpContext, [Service] BookRepository repo) 
        {
            // we can access the request like this to for example, authenticate the user
            if(httpContext.HttpContext?.Request.Method == "POST")
                System.Console.WriteLine("Post Request");
            if (httpContext.HttpContext?.Request.Headers != null)
                foreach (var header in httpContext.HttpContext?.Request.Headers)
                {
                    Console.WriteLine($"{header.Key} == {header.Value}");
                }


            return repo.GetBooks();
        }

        public Book OneBookById(string id, [Service] BookRepository repo)
        {
            return repo.GetBook(id);
        }
    }
}