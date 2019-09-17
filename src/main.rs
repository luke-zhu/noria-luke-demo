#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/<user>")]
fn profile(user: String) -> String {
    // TODO: this should fetch a profile - name + most recent post from a user and return it.
    // TODO: we need to know the zookeeper addr to get the connection/controllerhandle
    "Hello World!".into()
}

// TODO: Build a simple RSS web server
fn main() {
    rocket::ignite().mount("/", routes![profile]).launch();
}
