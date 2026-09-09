use std::io;
use chrono::Local;

mod variables;

const AUTHOR: &str = "Krishan Aggarwal";

fn main() {
    // because the println ends with ! which means its a marco
    println!("=============================================");
    println!("Name:     {}", AUTHOR); // Constant variable usage
    println!("Goal:     Become a Rust Systems Engineer");
    println!("Country:  Bharat");
    println!("Start Date: {}, Today's Date: {}", "2026-08-05", Local::now().format("%Y-%m-%d"));
    println!("=============================================");
    println!();
    println!("=============================================");
    println!("Features & Concepts Covered:");
    println!("1. Variables");
    println!("2. Value Moving");
    println!("=============================================");

    let mut input = String::new();
    println!("Enter option to demonstrate:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let selection: i32 = input.trim().parse().expect("Please type a number!");

    match selection {
        1 => variables::variables(),
        2 => value_moving(),
        _ => println!("Invalid option selected!"),
    }
}

fn value_moving() {
    let x = 5;
    let y = x; // Here the value of x is copied to y. This is because i32 implements the Copy trait.
    println!("x: {}, y: {}", x, y);

    let s1 = String::from("Hello");
    let s2 = s1; // Here the value of s1 is moved to s2. This is because String does not implement the Copy trait.
    // println!("s1: {}, s2: {}", s1, s2); // This line will result in error as s1 is no longer valid.
    println!("s2: {}", s2);
}