use std::io;

fn main() {
    let mut names: Vec<String> = Vec::new();
    let mut years: Vec<u32> = Vec::new();
    let mut inp1 = String::new();
    let mut hghindex = 0;
    let mut hghyear = 0;
    let mut morethan = false;
    

    println!("Welcome,\nThis program will determine the candidate with the highest years of programming experience");
    println!("How many people will you be interviewing today:  (in numbers)");
    io::stdin().read_line(&mut inp1).expect("Failed to read input");
    let no_of_candidates: u32 = inp1.trim().parse().expect("Invalid input");

    for i in 0..no_of_candidates
    {
        let mut inp2 = String::new();
        let mut inp3 = String::new();
        println!("\nWhat is the name of applicant number {}:   ", i+1);
        io::stdin().read_line(&mut inp2).expect("Failed to read input");
        names.push(inp2.trim().to_string());

        let ii = i as usize;
        println!("How many years of programming experience does {}, applicant number {} have (in numbers):   " ,names[ii], i+1);
        io::stdin().read_line(&mut inp3).expect("Failed to read input");
        let year: u32 = inp3.trim().parse().expect("Invalid input");
        years.push(year);
    }

    for y in 0..years.len()
    {
        let index = y as usize;
        if y == 0
        {
            hghindex = y as usize;
            hghyear = years[0];
        }
        else
        {
            if years[index] > hghyear
            {
                hghindex = index as usize;
                hghyear = years[index];
            }
        }
    }
    let mut v = vec![names[hghindex].clone()];
    let mut v2 = vec![years[hghindex].clone()];

    for z in 0..years.len()
    {
        let index = z as usize;
        
        if years[index] == hghyear && hghindex != index
        {
            morethan = true;
            v.push(names[index].clone());
            v2.push(years[index].clone());
        }
    }

    println!("\n\n\nALL APPLICANTS");

    for i in 0..names.len()
    {
        let index = i as usize;
        
        println!("{}.Applicant Name:   {}         Years of programming experience:   {}", i+1, names[index],years[index]);
    }

    if morethan
    {
        println!("\n\nThere are multiple applicants with the highest years of experience");
        println!("They are:");
        for a in 0..v.len()
        {
            let index = a as usize;
            println!("{}.Applicant Name:   {}         Years of programming experience:   {}", a+1, v[index],v2[index]);
        }
    }
    else
    {
        println!("\n\nApplicant number {} has the highest number of years of programmiung experience\nName of Applicant:   {}\nYears of programming experience:   {}", hghindex+1, names[hghindex],years[hghindex]);
    }
}