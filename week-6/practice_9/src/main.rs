fn main() {
    let A:i32 = 10;
    let B:i32 = 20;

    println!("Value of A:{} ",A);
    println!("Value of B : {} ",B);

    let mut res = A>B ;
    println!("A greater than B: {}", res);

    let mut res = A<B ;
    println!("A less than B: {}", res);

    let mut res = A>=B ;
    println!("A greater than or equal to B: {}", res);

    let mut res = A<=B ;
    println!("A lesser than or equal to B: {}", res);

    let mut res = A==B ;
    println!("A is equal to B: {}", res);

    let mut res = A!=B ;
    println!("A is not equal to B: {}", res);
}
