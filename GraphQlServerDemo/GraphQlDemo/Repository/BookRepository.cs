
using System;
using System.Collections.Generic;
using System.Linq;
using GraphQlDemo.Data;

namespace GraphQlDemo.Repository
{
    public class BookRepository
    {
        private static List<Book> Books = new(){
            new Book ("a", "b"),
            new Book ("asda", "b2"),
            new Book ("ab23", "b23"),
            new Book ("ab24", "b24"),
            new Book ("ab25", "b25"),
            new Book ("ab26", "b26"),
            new Book ("ab27", "b27"),
        };

        public List<Book> GetBooks()
        {
            return Books;
        }
        public Book GetBook(string id)
        {
            return Books.Where(x => x.Id == id).FirstOrDefault();
        }

        public Book CreateBook(string id, string title)
        {
            var book = new Book(id, title);
            Books.Add(book);
            return book;
        }
    }
}