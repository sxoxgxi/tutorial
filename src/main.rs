fn main() {
    // integers
    let x: i32 = 56;
    let y: u32 = 56;

    // float
    let floating_point: f32 = 10.92;
    
    // boolean 
    let false_boolean: bool = false;
    let true_boolean: bool = true;

    // character
    let letter: char = 'a';

    // tuple
    let tup: (i32, bool, char)= (1, true, 'a');
    println!("first one: {}", tup.1);

    let mut tup2: (i8, bool, char)= (1, true, 'a');
    tup2.0 = 5;
    println!("first one: {}", tup2.0);

    // array
    let mut arr = [1, 2, 3, 4, 5];
    arr[4] = 3;
    println!("{}", arr[4]);

    let mut arr2: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr2[3])

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