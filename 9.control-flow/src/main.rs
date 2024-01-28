fn main() {
    let num = 4;
    let mut msg = "";

    // Normal IF
    if num == 5 {
        msg = "five";
    } else if num == 4 {
        msg = "four";
    } else {
        msg = "Cannot determine";
    }

    msg = if num == 5 {
        "five"
    } else if num == 4 {
        "four"
    } else {
        "Cannot determine"
    };

    // Unconditional (or infinite) loops
    loop {
        println!("Test");

        break;
    }

    // Named unconditional loops
    'du: loop {
        'dudu: loop {
            'edu: loop {
                break 'dudu;
            }

            println!("Never reaches here");
        }

        break;
    }

    // While loops
    let mut number = 1;
    while number < 5 {
        println!("{}", number);
        number = number + 1;
    }

    // For loops
    let numbers = [1, 2, 3];
    for num in numbers {
        println!("{}", num);
    }

    // Ranged for loop
    for num in 0..50 {
        // 0 to 49
        println!("{}", num);
    }

    for num in 0..=50 {
        // 0 to 50
        println!("{}", num);
    }

    println!("End reached");
}
