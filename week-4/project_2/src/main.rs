use std::io;

fn main() {

    println!("\nPlease Enter your age,");
    let mut age = String::new();
      io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Input is not an integer");


    println!("Enter your experience in years:");
    let mut experience_years = String::new();
    io::stdin().read_line(&mut experience_years).expect("Failed to read input");
    let experience_years:i32 = experience_years.trim().parse().expect("Input not valid");
    
    let is_experienced:bool = if experience_years >= 3 {
        true
    } else {
        false
    };

    let mut annual_incentive:i32 = 0;

    if is_experienced == true && age >= 40 {
        annual_incentive = 1560000;
        println!("Your annual incentive is {} naira",annual_incentive);

    }
    else if is_experienced == true && age >= 30 && age < 40 {
        annual_incentive = 1480000;
        println!("Your annual incentive is {} naira",annual_incentive);
    }
    else if is_experienced == true && age < 28 {
        annual_incentive = 1300000;
        println!("Your annual incentive is {} naira",annual_incentive);

    }
    else if is_experienced == false {
        annual_incentive = 100000;
        println!("Your annual incentive is {} naira",annual_incentive);

    }
    else {
        println!("Unable to determine your annual incentive");
    }
}