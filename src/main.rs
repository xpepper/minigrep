use std::env;

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    let query = &arguments[1];
    let file_path = &arguments[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
