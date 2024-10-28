fn main () {
   let t:f64 = 450000.00;
   let m:f64 = 1500000.00;
   let h:f64 = 750000.00;
   let d:f64 = 2850000.00;
   let a:f64 = 250000.00;

   // Sum and average
   let s = t * 2.0 + m + h * 3.0 + d * 3.0 + a;
   println!("The sum is {}", s);
   let average = s / 10.0;
   println!("The average is {}",average);

   }