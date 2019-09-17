use noria::{SyncControllerHandle, DataType};
use chrono::NaiveDateTime;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let executor = rt.executor();

    // TODO: the address should be an environment variable or a flag
    let zookeeper_addr = "noria-zookeeper-headless:2181/demo";
    let mut db =
        SyncControllerHandle::from_zk(zookeeper_addr, executor).unwrap();

    println!("Creating tables");
    db.install_recipe("
        CREATE TABLE User (id int, name varchar(255), PRIMARY KEY (id));
        CREATE TABLE Post (id int, content varchar(255), created_by int, created_at TIMESTAMP, PRIMARY KEY(id));
    ");

    println!("Inserting rows");
    let mut user = db.table("User").unwrap().into_sync();
    user.insert(vec![0.into(), "Luke".into()]);

    let mut post = db.table("Post").unwrap().into_sync();
    post.insert(vec![0.into(), "Hello World".into(), 0.into(),
                     DataType::Timestamp(NaiveDateTime::parse_from_str("2019-01-05 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())]);
    post.insert(vec![1.into(), "911".into(), 0.into(),
                     DataType::Timestamp(NaiveDateTime::parse_from_str("2019-01-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())]);
    post.insert(vec![2.into(), "Help".into(), 0.into(),
                     DataType::Timestamp(NaiveDateTime::parse_from_str("2019-01-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())]);
    post.insert(vec![3.into(), "Thanks".into(), 0.into(),
                     DataType::Timestamp(NaiveDateTime::parse_from_str("2019-01-09 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())]);

    // TODO: create the  profile view - - name + most recent post from a user
}
