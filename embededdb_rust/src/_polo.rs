use uuid::Uuid;
use polodb_core::*;
use polodb_bson::*;

pub fn main() {
    //create file "main.db" if not exist
    let mut db: Database = Database::open("data/my.db").unwrap();
    let mut col = db.collection("Hello there").unwrap();

    let mut id: Option<String> = None;

    for i in 0..10_000 {
        
        let _id = Uuid::new_v4().to_string();
        let mut doc = mk_document!{
            "_id": _id.clone(),
            "Hello": "there",
            "Index": i,
        };

        col.insert(&mut doc).unwrap();

        if i % 3 == 0 && i % 2 == 0 {
            println!("{}", &i)
        } 

        if i == 500 {
            id = Some(_id);
        }
    }

    println!("Got {} documents", col.count().unwrap());
    println!("Finding {:#?} ", id);

    let docs = col.find_all().unwrap();
    for i in docs {
        let i = i.as_ref();
        println!("{}", i.get("_id").unwrap())
    }

    println!("Totalizing {} documents", col.count().unwrap());

    if let Some(id) = id {
        println!("Finding {} ", &id);
        let doc = mk_document!{
            "_id": id.clone()
        };
        let doc = col.find(&doc).expect(format!("Did not find document with id {}", id).as_str());
        println!("{:#?}", &doc);
    }


}
