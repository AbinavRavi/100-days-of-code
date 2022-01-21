use std::fs;
use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    let data = fs::read_to_string(filename).expect("unable to read text file i/o error");
    let occ = data.matches(query).count();
    println!("occurence of {} in {} is {}",query,filename,occ);
}
