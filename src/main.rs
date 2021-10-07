use ejdb::{Database, bson, query::{Q, QH}};

fn main() {
    println!("Hello, world!");

    let db = Database::open("mydb.db").unwrap();
    let col = db.collection("some_collection").unwrap();

    let mut d = bson! {
        "name" => "Foo Bar",
        "count" => 10
    };
    let inserted_id = col.save(&d).unwrap();
    // col.index("name").string(true).set().unwrap();
    d.insert("_id", inserted_id.clone());

    let d2 = col.load(&inserted_id).unwrap().unwrap();
    println!("{}", d2);

    for i in col.query(Q.field("name").exists(true), QH.field("name").include()).find().unwrap() {
        if let Ok(i) = i {
            println!("{:#?}", i);
        }
    }
}
