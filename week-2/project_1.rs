fn main () {
   let p:f64 = 520000000.0;
   let r:f64 = 0.1;
   let t:f64 = 5.0;
   let n:f64 = 1.0;

   // Amount
   let a = p * (1.0_f64 + (r / n)).powf(t);
   println!("The amount is {}", a);

   // Compound Interest
   let ci = a - p;
   println!("The compound interest is {}", ci);

   }