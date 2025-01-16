//define dimensions of a rectanglw
struct Rectangle {
    width:u32, height:u32
}

//logic to calculate area of a rectangle
impl Rectangle {
    fn area(&self)->u32 {
        //use the - operator to fetch the value of a field via the self keyword
        self.width * self.height
    }
}

fn main() {
    // instantiate the structure
    let small = Rectangle {
        width:10,
        height:20
    };
    //print the rectangle's area
    println!("width is {} \n height is {} \n are of Rectangle is {}",small.width,small.height,small.area())
}