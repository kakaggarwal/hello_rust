pub fn controlflow() {
    println!("Control Flow Demonstration:");
    println!("================================");
    println!("For loop demonstration:");

    let marks = [95, 72, 45, 88, 61];

    let mut a_count = 0;
    let mut b_count = 0;
    let mut c_count = 0;
    let mut f_count = 0;

    for mark in marks {
        let grade = match mark {
            91.. => {
                a_count += 1;
                "A"
            }
            76..=90 => {
                b_count += 1;
                "B"
            }
            61..=75 => {
                c_count += 1;
                "C"
            }
            _ => {
                f_count += 1;
                "F"
            }
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

    let mut count = 3;
    while count > 0 {
        println!("Count: {}", count);
        count -= 1;
    }

    println!("While loop demonstration completed.");
    println!("---------------------------");
    println!("Loop demonstration:");
    let bonus = loop {
        break 5;
    };

    println!("Bonus Marks: {}", bonus);
    println!("Loop demonstration completed.");
    println!("================================");
    println!("Control Flow Demonstration Completed.");
}
