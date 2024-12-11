fn main () {

    // Using Vec::new()
    let v : Vec<i64> = Vec::new();

    //printing the size of vector
    println!("\nThe length of Vec::new is {}",v.len());

    // Using macro
    let v =  vec!["Grace", "Effiong", "Basil", "Kareem", "Susane"];

    // printing the size of vector
    println!("\nThe length of vec macro is: { }",v.len());
}