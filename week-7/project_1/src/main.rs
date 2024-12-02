use std::io;
fn trapezium_area () {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the first base of the trapezium: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the second base of the trapezium: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the height of the trapezium: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let h:f32 = input3.trim().parse().expect("Not a valid number");

    let s:f32 = a + b ;
    let area:f32 = s * 0.5 * h;

    println!("Area of the trapezium is: {}",area);
}

fn rhombus_area () {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the first diagonal of the rhombus: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the second diagonal of the rhombus ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    let area:f32 =  0.5 * a * b;

    println!("Area of the rhombus is: {}",area);
}

fn parallelogram_area () {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the base of the parallelogram: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the altitude of the parallelogram: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    let area:f32 = a * b;

    println!("Area of the parallelogram is: {}",area);
}

fn cube_area () {
    let mut input1 = String::new();

    println!("Enter the length of the side of the cube: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    let area:f32 = 6.0 * (a.powf(2.0));

    println!("Area of the cube is: {}",area);
}

fn cylinder_volume () {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the radius of the cylinder: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the height of the cylinder: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    let volume:f32 = 3.142 * (a.powf(2.0)) * b;

    println!("Volume of the cylinder is: {}",volume);
}

fn main () {
    println!("This is a rust calculator prototype");

    loop {
    println!("\nWhat calculation are you performing;\n Input (\nT for area of a trapezium, \nR for area of a rhombus, \nP for area of a parallelogram, \nC for area of a cube\nor\nV for volume of a cylinder\n):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let choice = input.trim().to_uppercase();

    if choice == "T" {
        println!("You have selected the area of a trapezium");
        trapezium_area();
    }
    else if choice == "R" {
        println!("You have selected the area of a rhombus");
        rhombus_area();
    }
    else if choice == "P" {
        println!("You have selected the area of a parallelogram");
        parallelogram_area();
    }
    else if choice == "C" {
        println!("You have selected the area of a cube");
        cube_area();
    }
    else if choice == "V" {
        println!("You have selected the volume of a cylinder");
        cylinder_volume();
    }
    else {
        println!("Invalid input. Put in a correct value");
    }
    println!("\nDo you want to continue? Say y for yes or n for no. ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Invalid, please say [yes] or [no] ");
    let answer: bool = match input3.trim().to_lowercase().as_str(){
        "y" => true,
        "n" => false,
     _=> {
            println!("Invalid input; put in y or n.");
            return;
        }
    };
    if answer == false {
        println!("Thanks for testing it out");
        break;
    }

}
}