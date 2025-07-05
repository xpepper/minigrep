use std::env;

fn main() {
    let _arguments = env::args().collect::<Vec<_>>();
    print!("Hello!");
}
