use std::fs; 

fn main() {
    let contents = fs::read_to_string("/proc/stat")
        .expect("Should be a valid path");
    println!("Path content: \n{}", contents);
}
