pub fn variables() {
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