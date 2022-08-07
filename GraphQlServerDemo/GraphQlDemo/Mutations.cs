using GraphQlDemo.Data;
using GraphQlDemo.Repository;
using HotChocolate;
using HotChocolate.Utilities;

namespace GraphQlDemo
{
    public class Mutations
    {
        public Book CreateBook(string id, string title, [Service] BookRepository repo) => repo.CreateBook(id, title);
    }
}