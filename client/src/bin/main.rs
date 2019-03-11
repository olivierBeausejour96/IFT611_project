extern crate ift611_user;

use ift611_user as us;
use ift611_user::shared as sh;

fn main() {
    println!("main Hello world");
    us::execute();

    let d = sh::Data { high: 32, low: 32, opening: 32, closing: 32 };

    println!("{:?}", d);
}