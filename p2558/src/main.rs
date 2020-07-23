use std::io::{stdin, BufRead};

fn main() {
    //let mut input = String::new();
    let stdin = stdin();
    let mut iterator = stdin.lock().lines();
    let a = iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let b = iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    println!("{}", a +b);
}
