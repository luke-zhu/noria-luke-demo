use chrono::NaiveDateTime;
use noria::{DataType, SyncControllerHandle};
use clap::{App, Arg};

// TODO: handle errors
fn main() {
    let matches = App::new("initdb")
        .about("Populates the database. Run this only once")
        .arg(Arg::with_name("zookeeper")
            .short("z")
            .long("zookeeper")
            .takes_value(true)
            .default_value("127.0.0.1:2181/demo"))
        .get_matches();

    let zookeeper_addr = matches.value_of("zookeeper").unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let executor = rt.executor();
    let mut db = SyncControllerHandle::from_zk(zookeeper_addr, executor).unwrap();

    println!("Creating tables");
    db.install_recipe("
        CREATE TABLE users (uid int, name varchar(255), PRIMARY KEY (uid));
        CREATE TABLE posts (pid int, content varchar(255), created_by int, created_at TIMESTAMP, PRIMARY KEY(pid));
    ");

    println!("Inserting rows");
    let mut users = db.table("users").unwrap().into_sync();
    users.insert(vec![0.into(), "luke".into()]);

    let mut posts = db.table("posts").unwrap().into_sync();
    posts.insert(vec![
        0.into(),
        "Hello World".into(),
        0.into(),
        DataType::Timestamp(
            NaiveDateTime::parse_from_str("2019-01-05 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
    ]);
    posts.insert(vec![
        1.into(),
        "911".into(),
        0.into(),
        DataType::Timestamp(
            NaiveDateTime::parse_from_str("2019-01-06 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
    ]);
    posts.insert(vec![
        2.into(),
        "Help".into(),
        0.into(),
        DataType::Timestamp(
            NaiveDateTime::parse_from_str("2019-01-08 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
    ]);
    posts.insert(vec![
        3.into(),
        "Thanks".into(),
        0.into(),
        DataType::Timestamp(
            NaiveDateTime::parse_from_str("2019-01-09 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
    ]);

    println!("Creating view");
    db.extend_recipe(
        "
        QUERY snapshots: \
            SELECT users.name, posts.content, posts.created_at \
            FROM users, posts \
            WHERE users.uid = posts.created_by \
            AND users.name = ? \
            ORDER BY posts.created_at \
            LIMIT 1;
    ",
    );
}
