use std::io;

fn main() {
    println!("Welcome to our restaurant; this is our menu \nMenu
    
    P = Poundo Yam and Edinkaiko Soup - N3,200.00
    F = Fried Rice and Chicken        - N3,000.00
    A = Amala and Ewedu Soup          - N2,500.00
    E = Eba and Egusi Soup            - N2,000.00
    W = White Rice and stew           - N2,500.00
    ");

    let menu = [
        ("P","Poundo Yam & Edinkaiko Soup",3_200.00),
        ("F","Fried Rice & Chicken",3_000.00),
        ("A","Amala & Ewedu Soup",2_500.00),
        ("E","Eba & Egusi Soup",2_000.00),
        ("W","White Rice & Stew",2_500.00),

    ];

    let mut total_amount: f32 = 0.0;

    loop {
        println!("\nMake your order here; input (P for Poundo Yam & Edinkaiko Soup, \nF for Fried Rice & Chicken, \nA for Amala & Ewedu Soup, \nE for Eba & Egusi Soup or \nW for White Rice & Stew):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.");
        let choice = input.trim().to_uppercase();

        let cost = menu.iter().find(|&&(code, _ , _)| code == choice);
        if let Some(&(_, name, price)) = cost {
            println!("You selected: {} N {}.",name, price);

            println!("\nHow many of that would you have?");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("Enter the number you would have");
            let number: f32 = match input2.trim().parse(){
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid quantity. Please enter a valid number.");
                    continue;
                }
            };
            total_amount += number * price;

            println!("Your current balance is: {}",total_amount);
            println!("\nDo you want to continue? Say yes or no. ");
            let mut input3 = String::new();
            io::stdin().read_line(&mut input3).expect("Invalid, please say [yes] or [no] ");
            let answer: bool = match input3.trim().to_lowercase().as_str(){
                "yes" => true,
                "no" => false,
             _=> {
                    println!("Invalid input; put in either yes or no.");
                    return;
                }
            };
            if answer == false {
                break;
            }
        }
    }

    if total_amount > 10_000.00 {
        println!("Dear customer, your bill is : {}. But you are eligible to our 5% discount!",total_amount);
        let new_amount = total_amount * 0.95;
        println!("\nYour current bill is: {}. Thank you for dining with us.",new_amount);
    }
    else {
        println!("\nDear customer, your bill s : {}. Thank you for dining with us.",total_amount)
    }


}
