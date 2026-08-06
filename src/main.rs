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
}