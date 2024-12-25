use std::io::Write;
use std::io::Read;

fn write_vectors_to_file(lager: &[&str], stout: &[&str], nonalcoholic: &[&str]) {
    let mut file = std::fs::File::create("drinks.txt").expect("file creation failed");
    file.write_all("Lager:  ".as_bytes()).expect("write failed");
    
    for beer in lager  {
        file.write_all(beer.as_bytes()).expect("write failed");
        file.write_all(", ".as_bytes()).expect("write failed");
    }
    
    file.write_all("\n\nStout:  ".as_bytes()).expect("write failed");
    
    for beer in stout {
        file.write_all(beer.as_bytes()).expect("write failed");
        file.write_all(", ".as_bytes()).expect("write failed");
    }
    
    file.write_all("\n\nNon-alcoholic:  ".as_bytes()).expect("write failed");
    
    for beer in nonalcoholic {
        file.write_all(beer.as_bytes()).expect("write failed");
        file.write_all(", ".as_bytes()).expect("write failed");
    }
}

fn main() {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo", "Williams"];
    let nonalcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    write_vectors_to_file(&lager, &stout, &nonalcoholic);
    let mut read = String::new();
    let mut files = std::fs::File::open("drinks.txt").expect("opening failed");
    files.read_to_string(&mut read).expect("read failed");
    println!("{}", read);
}