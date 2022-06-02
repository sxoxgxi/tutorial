fn main() {
    let x = 6;
    println!("x is: {}", x);

    {
        let x = x - 2;
        println!("x is: {}", x);
    }

    let x = x + 1;
    println!("x is: {}", x);

    // let mut y = 6;
    // println!("y is: {}", y);
    // y = 7;
    // println!("y is: {}", y);
}