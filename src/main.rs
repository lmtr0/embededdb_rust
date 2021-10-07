use ejdb::{Database, bson, query::{Q, QH}};

fn main() {
    println!("Hello, world!");

    let db = Database::open("data/mydb").unwrap();
    let col = db.collection("some_collection").unwrap();

    let mut index: String = String::new();
    for i in 0..10 {
        let d = bson! {
            "name" => "Foo Bar",
            "count" => i
        };

        let id = col.save(&d).unwrap();
        if i == 5 {
            index = id.to_string()
        }
    }
    // col.index("name").string(false).set().unwrap();
    println!("Count {:#?}", col.query(Q.empty(), QH.empty()).count().unwrap());
    
    { // by id
        let now = std::time::SystemTime::now();
        println!("Finding {}", &index);
        println!("{:#?}", col.query(Q.id(index.as_str()), QH.empty()).find_one().unwrap().unwrap());
        let since = std::time::SystemTime::now().duration_since(now).unwrap();
        println!("Took {:?} to find", &since);
    }
    
    { // random field
        let now = std::time::SystemTime::now();
        println!("Finding {}", &index);
        println!("{:#?}", col.query(Q.field("count").eq(1), QH.empty()).find_one().unwrap().unwrap());
        let since = std::time::SystemTime::now().duration_since(now).unwrap();
        println!("Took {:?} to find", &since);
    }
    
}
