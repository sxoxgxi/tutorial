use std::io;

fn main() {

    // string conversion to integers
    let mut string_age = String::new();
    io::stdin().read_line(&mut string_age).expect("Error reading the line");
    let int_age: i64 = string_age.trim().parse().unwrap();
    println!("string age: {}", string_age);
    println!("int age adding 1: {}", int_age + 1);

    // integers type casting and conversion
    let x = 34.0f32; // let x = 34.0_f32
    let y = 35.0f32; 
    println!("Addition: {}", x + y);    
   
    let a = 34_000 as i64;
    let b = 10_i64; 
    println!("Multiplication: {}", a * b);    
   
    let c = 30_i64;
    let d = 3_i32; 
    println!("Division: {}", c / (d as i64)); 

    let x: u8 = 5; // 0 - 255
    let y: i8 = 10; // -128 - 127

    let z = x + y; 
    println!("{}", z); // Err: cannot add u8 with i8

    // user input
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("failed to read line");
    // println!("{}", input);

    // let mut nirmal = String::new();
    // io::stdin().read_line(&mut nirmal).expect("failed to read the line");
    // println!("meow {}", nirmal);

    // integers
    // let x: i32 = 56;
    // let y: u32 = 56;

    // float
    // let floating_point: f32 = 10.92;
    
    // boolean 
    // let false_boolean: bool = false;
    // let true_boolean: bool = true;

    // character
    // let letter: char = 'a';

    // tuple
    // let tup: (i32, bool, char)= (1, true, 'a');
    // println!("first one: {}", tup.1);

    // let mut tup2: (i8, bool, char)= (1, true, 'a');
    // tup2.0 = 5;
    // println!("first one: {}", tup2.0);

    // array
    // let mut arr = [1, 2, 3, 4, 5];
    // arr[4] = 3;
    // println!("{}", arr[4]);

    // let mut arr2: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("{}", arr2[3])

    // const SECONDS_PER_MINUTE: u32 = 60;
    // println!("{}", SECONDS_PER_MINUTE);

    // let x = 6;
    // println!("x is: {}", x);

    // {
    //     let x = x - 2;
    //     println!("x is: {}", x);
    // }

    // let x = x + 1;
    // println!("x is: {}", x);

    // let mut y = 6;
    // println!("y is: {}", y);
    // y = 7;
    // println!("y is: {}", y);
}