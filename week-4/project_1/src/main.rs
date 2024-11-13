use std::io;

fn main() {
    println!("Enter a value for a");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter a value for b");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f64 = input2.trim().parse().expect("Not a valid number");

    println!("Enter a value for c");

    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f64 = input3.trim().parse().expect("Not a valid number");

    let discriminant = b.powi(2) - (4.0 * a * c);

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);


        println!("The are two distinct roots:");
        println!("Root 1 is: {}",root1);
        println!("Root 2 is: {}",root2);
    }
    else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("There is one real root: {}", root);
    }
    else{
        println!("There are no real roots");
    }

}