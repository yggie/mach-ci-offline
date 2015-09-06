extern crate iron;
extern crate mount;
extern crate staticfile;

use std::path::Path;

use iron::Iron;
use mount::Mount;
use staticfile::Static;

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("public/")));

    println!("Debugging server running on http://localhost:8888/");
    Iron::new(mount).http("127.0.0.1:8888").unwrap();
}
