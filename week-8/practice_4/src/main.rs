fn main() {
    //name vector
    let name = vec!["Mary","Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    // age vector
    let age = vec![16,17,19,22,20,21,18,23];

    println!("\nAge Allocation:\n");

    //loop to iterate elements in vector
    for i in 0..age.len()
    {
        // iterating through i on the vector
        print!("{} is {} years old\n",name[i], age[i]);
    }
}