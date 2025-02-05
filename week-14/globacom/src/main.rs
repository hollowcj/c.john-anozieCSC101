use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    println!("=== Role Selection ===");
    println!("Available roles:");
    println!("1. Admin");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    loop {
        println!("Please select your role (1-5):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let role_num: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        let role = match role_num {
            1 => "admin",
            2 => "project manager",
            3 => "employee",
            4 => "customer",
            5 => "vendor",
            _ => {
                println!("Please select a valid role!");
                continue;
            }
        };
        let mut file = File::open("./globacom_dbase1.sql")?;
        let mut buffer = String::new();
        println!("=== Found Tables ===");
        file.read_to_string(&mut buffer)?;
        
        let lines: Vec<&str> = buffer.lines().collect();
        let mut in_create_statement = false;
        
        for line in &lines {
            let line = line.trim();
            
            if role == "admin" {
                if line.to_lowercase().starts_with("create table") {
                    in_create_statement = true;
                    println!("\n{}", line);
                } else if in_create_statement && line.ends_with(';') {
                    println!("{}", line);
                    in_create_statement = false;
                } else if in_create_statement {
                    println!("{}", line);
                }
                
                if line.to_lowercase().starts_with("copy") {
                    in_create_statement = true;
                    println!("\n{}", line);
                } else if in_create_statement && line.ends_with('.') {
                    println!("{}", line);
                    in_create_statement = false;
                } else if in_create_statement {
                    println!("{}", line);
                }
            }
            else if role == "project manager" {
                if line.to_lowercase().starts_with("create table public.project") {
                    in_create_statement = true;
                    println!("\n{}", line);
                } else if in_create_statement && line.ends_with(';') {
                    println!("{}", line);
                    in_create_statement = false;
                } else if in_create_statement {
                    println!("{}", line);
                }
                
                if line.to_lowercase().starts_with("copy public.project") {
                    in_create_statement = true;
                    println!("\n{}", line);
                } else if in_create_statement && line.ends_with('.') {
                    println!("{}", line);
                    in_create_statement = false;
                } else if in_create_statement {
                    println!("{}", line);
                }
            }
    
            else if role == "employee" {
                if line.to_lowercase().starts_with("create table public.staff") {
                    in_create_statement = true;
                    println!("\n{}", line);
                } else if in_create_statement && line.ends_with(';') {
                    println!("{}", line);
                    in_create_statement = false;
                } else if in_create_statement {
                    println!("{}", line);
                }
                
                if line.to_lowercase().starts_with("copy public.staff") {
                    in_create_statement = true;
                    println!("\n{}", line);
                } else if in_create_statement && line.ends_with('.') {
                    println!("{}", line);
                    in_create_statement = false;
                } else if in_create_statement {
                    println!("{}", line);
                }
            }
    
            else if role == "customer" {
                if line.to_lowercase().starts_with("create table public.customer") {
                    in_create_statement = true;
                    println!("\n{}", line);
                } else if in_create_statement && line.ends_with(';') {
                    println!("{}", line);
                    in_create_statement = false;
                } else if in_create_statement {
                    println!("{}", line);
                }
                
                if line.to_lowercase().starts_with("copy public.customer") {
                    in_create_statement = true;
                    println!("\n{}", line);
                } else if in_create_statement && line.ends_with('.') {
                    println!("{}", line);
                    in_create_statement = false;
                } else if in_create_statement {
                    println!("{}", line);
                }
            }
            
            else if role == "vendor" {
                if line.to_lowercase().starts_with("create table public.dataplan") {
                    in_create_statement = true;
                    println!("\n{}", line);
                } else if in_create_statement && line.ends_with(';') {
                    println!("{}", line);
                    in_create_statement = false;
                } else if in_create_statement {
                    println!("{}", line);
                }
                
                if line.to_lowercase().starts_with("copy public.dataplan") {
                    in_create_statement = true;
                    println!("\n{}", line);
                } else if in_create_statement && line.ends_with('.') {
                    println!("{}", line);
                    in_create_statement = false;
                } else if in_create_statement {
                    println!("{}", line);
                }
            }
        }
        
        break;
    }
    
    Ok(())
}