#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::format;

use noria::SyncControllerHandle;
use rocket::response::content::Html;

// TODO: handle errors
#[get("/<user>")]
fn profile(user: String) -> Html<String> {
    // TODO: usually we would do pooling, but out of time
    let rt = tokio::runtime::Runtime::new().unwrap();
    let executor = rt.executor();

    // TODO: the address should be an environment variable or a flag
    let zookeeper_addr = "localhost:2181/test";
    //    let zookeeper_addr = "noria-zookeeper-headless:2181/demo";
    let mut db = SyncControllerHandle::from_zk(zookeeper_addr, executor).unwrap();

    let mut snapshots = db.view("snapshots").unwrap().into_sync();
    let user_clone = user.clone();
    let records = snapshots
        .lookup(&[user.to_lowercase().into()], true)
        .unwrap();

    if records.is_empty() {
        return Html(format!(
            "<h1>{}</h1><p>The user has not posted anything</p>",
            user_clone
        ));
    }

    let snap = records.first().unwrap();
    let content: String = snap.get(1).unwrap().into();
    let created_at = snap.get(2).unwrap().deep_clone();
    println!("Created at: {}", created_at);

    return Html(format!(
        "<h1>{}</h1><h3>Latest</h3><p>Posted At: {}</p><p>{}</p>",
        user_clone, created_at, content
    ));
}

fn main() {
    rocket::ignite().mount("/", routes![profile]).launch();
}
