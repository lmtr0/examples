namespace GraphQlDemo.Data
{
    public class Book 
    {
        public Book(string id, string title) 
        {
            this.Id = id;
            this.Title = title;      
        }
        
        public string Id { get; set; }
        public string Title { get; set; }
    }
}