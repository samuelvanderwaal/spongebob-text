use std::env;
use rand;

fn main() {
    let mut output = String::new();

    for arg in env::args().skip(1) {
        let s: String = arg.chars().map(|c| {
            if rand::random() {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        }).collect();
        output.push_str(&s);
        output.push_str(" ");
    }

    println!("{}", output);
}