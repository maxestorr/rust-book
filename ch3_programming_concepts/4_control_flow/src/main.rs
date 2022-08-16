fn main() {
    if_else();

    inf_loop();

    while_loop();

    for_loop();

    final_loop();
}

fn final_loop() {
    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!!")
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is {element}");
    }
}

fn while_loop() {
            let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn inf_loop() {
        let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn if_else() {
        let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    };
}