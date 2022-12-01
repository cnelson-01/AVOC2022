use std::fs;

fn main() {
    let p = std::env::current_dir();
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

    vec.sort();
    vec.reverse();

    println!("top {}", vec[0]);
    println!("second {}", vec[1]);
    println!("third {}", vec[2]);

    println!("total {}", vec[2] + vec[1] + vec[0]);
}
