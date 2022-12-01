use std::fs;

fn main() {
    let p = std::env::current_dir();
    println!("the current dir is {}", p.expect("REASON").display());
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut vec = Vec::new();
    vec.push(0);

    for line in contents.lines() {
        if line.eq("") {
            vec.push(0);
        } else {
            if let Some(last) = vec.last().cloned() {
                vec.pop();
                vec.push(last + line.parse::<i32>().unwrap());
            }
        }
    }

    let max_val = vec.iter().max();
    match max_val {
        Some(min) => println!("Max value: {}", min),
        None => println!("Vector is empty"),
    }
}
