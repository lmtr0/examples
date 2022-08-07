# GraphQlDemo [![Build Status](https://cloud.drone.io/api/badges/litch0/GraphQlServerDemo/status.svg)](https://cloud.drone.io/litch0/GraphQlServerDemo)

This projects is a test project that i created to test the new technology GraphQl

Your are free to use it as a template, because it comes with a basic Query, Mutation, Repository and Type out of the box.

# How To Run
## 2 Steps
- `dotnet restore`
- `dotnet run -p GraphQlDemo/GraphQlDemo.csproj`

## Suggestions to Test the app

### To return Everything:
```graphql
query{
  allBooks{
    id
    title
  }
}
```
### To return a specific book
```graphql
query {
  oneBookById(id: "asda")
  {
    id, 
    title
  }
}
```

### Create a book
```graphql
mutation {
  createBook(id: "my cool id", title: "my cool title")
}

```
