fn main() {
    let a = 20;
    let b = 30;

    if (a > 10) && (b > 10) {
        println!("true");
    }
    let c = 0;
    let d = 30;

    if (c>10) || (d>30){
        println!("true");
    }
    let is_elder = false;

    if !is_elder {
        println!("Not elder");
    }
}
