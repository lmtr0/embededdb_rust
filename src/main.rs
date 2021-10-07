use ejdb::{Database, bson, query::{Q, QH}};

fn main() {
    println!("Hello, world!");

    let db = Database::open("mydb.db").unwrap();
    let col = db.collection("some_collection").unwrap();

    let mut index: String = String::new();
    for i in 0..1000000 {
        let d = bson! {
            "name" => "Foo Bar",
            "count" => i
        };

        let id = col.save(&d).unwrap();
        if i == 1000 {
            index = id.to_string()
        }
    }
    // col.index("name").string(false).set().unwrap();
    println!("Finding {}", &index);
    println!("{:#?}", col.query(Q.id(index.as_str()), QH.empty()).find_one().unwrap().unwrap());
    println!("Count {:#?}", col.query(Q.empty(), QH.empty()).count().unwrap());
}
