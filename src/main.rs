use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[3];

    println!("Searching for query {}, in file {}.", query, filename);
}
