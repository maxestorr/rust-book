use std::io;

fn main() {
    let tup = (500, 6.4, 1);

    let (_, y, _) = tup;
    println!("The value of y is: {y}");

    let x = tup.0;
    println!("The value of x is: {x}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
