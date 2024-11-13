use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Enter the coefficients of a quadratic equation:");
    println!("a =");
    let mut a = String::new();
    io::stdin().read_line(&mut a)?;
    
    println!("b =");
    let mut b = String::new();
    io::stdin().read_line(&mut b)?;
    
    println!("c =");
    let mut c = String::new();
    io::stdin().read_line(&mut c)?;

    let a: f64 = a.trim().parse()?;
    let b: f64 = b.trim().parse()?;
    let c: f64 = c.trim().parse()?;

    let discriminant = b.powi(2) - 4.0 * a * c;
    
    match discriminant {
        n if n > 0.0 => {
            let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
            println!("The equation has two distinct real roots:\n{} and {}", root1, root2);
        },
        0 => {
            let root = -b / (2.0 * a);
            println!("The equation has one real root: {}", root);
        },
        _ => {
            let imaginary_part = discriminant.sqrt() / (2.0 * a);
            println!("The equation has no real roots. Complex roots: {} ± {}i", -b / (2.0 * a), imaginary_part);
        }
    }

    Ok(())
}