fn main() {
    //name vector
    let names = vec!["Mary","Sam", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    // age vector
    let ages = vec![16,17,19,22,20,21,18,23];


    println!("\nAge Allocation:\n");

    //loop to iterate elements in vector
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }
}