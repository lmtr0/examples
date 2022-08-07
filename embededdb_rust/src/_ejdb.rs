// use ejdb::{CollectionOptions, Database, bson, query::{Q, QH}};

// fn main() {
//     let db = Database::open("data/mydb").unwrap();
    
//     let col = db.collection_with_options("some_collection", CollectionOptions {
//         large: false,
//         compressed: true,
//         records: 128_000,
//         cached_records: 0,
//     }).unwrap();

//     let mut index: String = String::new();
//     for i in 0..1000000 {
//         let d = bson! {
//             "name" => "Foo Bar",
//             "count" => i
//         };

//         let id = col.save(&d).unwrap();
//         if i == 5 {
//             index = id.to_string()
//         }
//     }
//     // col.index("name").string(false).set().unwrap();
//     println!("Count {:#?}", col.query(Q.empty(), QH.empty()).count().unwrap());
    
//     { // by id
//         let now = std::time::SystemTime::now();
//         println!("Finding {}", &index);
//         println!("{:#?}", col.query(Q.id(index.as_str()), QH.empty()).find_one().unwrap().unwrap());
//         let since = std::time::SystemTime::now().duration_since(now).unwrap();
//         println!("Took {:?} to find", &since);
//     }
    
//     { // random field
//         let now = std::time::SystemTime::now();
//         println!("Finding {}", &index);
//         println!("{:#?}", col.query(Q.field("count").eq(1), QH.empty()).find_one().unwrap().unwrap());
//         let since = std::time::SystemTime::now().duration_since(now).unwrap();
//         println!("Took {:?} to find", &since);
//     }
    
// }