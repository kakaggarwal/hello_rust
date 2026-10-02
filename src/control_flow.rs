pub fn controlflow() {
    println!("Control Flow Demonstration:");
    println!("---------------------------");
    println!("For loop demonstration:");

    let marks = [95, 72, 45, 88, 61];

    let mut a_count = 0;
    let mut b_count = 0;
    let mut c_count = 0;
    let mut f_count = 0;

    for mark in marks {
        let grade = if mark > 90 {
            a_count += 1;
            "A"
        } else if mark > 75 {
            b_count += 1;
            "B"
        } else if mark > 60 {
            c_count += 1;
            "C"
        } else {
            f_count += 1;
            "F"
        };

        println!("Mark: {}, Grade: {}", mark, grade);
    }

    println!("A grades: {}", a_count);
    println!("B grades: {}", b_count);
    println!("C grades: {}", c_count);
    println!("F grades: {}", f_count);

    println!("For Loop Demonstration Completed.");
    println!("---------------------------");
    println!("Match statement demonstration:");
    
    match a_count {
        0 => println!("No Toppers"),
        1 => println!("One Topper"),
        _ => println!("Many Toppers"),
    }
    
    println!("Match statement demonstration completed.");
    println!("---------------------------");
    println!("If statement demonstration:");

    let overall_result = if f_count == 0 {
        "All passed"
    } else {
        "Some failed"
    };
    
    println!("Overall Result: {}", overall_result);

    println!("If statement demonstration completed.");
    println!("---------------------------");
    println!("While loop demonstration:");
    
    println!("Control Flow Demonstration Completed.");
}
