const AUTHOR: &str = "Krishan Aggarwal";

fn main() {
    // because the println ends with ! which means its a marco
    println!("Hello, world!");

    println!("=============================================");
    println!("Name:     {}", AUTHOR); // Constant variable usage
    println!("Goal:     Become a Rust Systems Engineer");
    println!("Country:  Bharat");
    println!("=============================================");

    variables();
    value_moving();
}

fn variables() {
    // let age = 33;
    // age = 34;
    // The above code results in error as we cannot assign again to mutable variable.

    let mut age = 33;
    println!("Age: {}", age);
    age = 34;
    println!("Age: {}", age);
    // The above code works because we have declared the variable as mutable using mut keyword.

    let mail_count = 0;
    println!("Mail Count: {}", mail_count);
    let mail_count = mail_count + 1;
    println!("Mail Count: {}", mail_count);
    // This is an example of shadowing. We have declared a new variable with the same name as the previous one. 
    // The previous variable is shadowed by the new one.

    let name: String = String::from("Alice");
    let age: i32 = 30;
    let salary: f64 = 50000.0;
    let is_active: bool = true;
    let grade: char = 'A';

    let employee = (name, age, salary, is_active, grade);
    let marks = [90, 85, 95];

    // println!("Name: {}", name);
    // Above code will not compile as name variable has been moved to employee tuple. We can access the name variable from the employee tuple.
    println!("Name: {}", employee.0);
    println!("Age: {}", employee.1);
    println!("Marks: {}", marks[1]);

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