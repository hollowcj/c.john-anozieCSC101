use std::io::Write;

fn main() {
    let announce = "Week 9 - Rust File Input and Output\n";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n".asbytes()).expect("write failed");
    file.write_all(announce.asbytes()).expect("write failed");
    file.write_all(dept.asbytes()).expect("write failed");
    println!("\nData written to file.");
}
