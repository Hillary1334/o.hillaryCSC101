//  A rust program to calculate the area of trapezium
use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new(); 
    let mut input3 = String::new(); 
    
    println!("Enter Height: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let base:i32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter Base1: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let base:i32 = input2.trim().parse().expect("Not a valid number");

     println!("Enter Base2: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let base:i32 = input3.trim().parse().expect("Not a valid number");

    let area = ((input1) / 2)*(input2 + &input3);
    println!("Area of a trapezium is: {}",area);
}