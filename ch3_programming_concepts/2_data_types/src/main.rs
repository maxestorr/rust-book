fn main() {
    let tup = (500, 6.4, 1);

    let (_, y, _) = tup;
    
    println!("The value of y is: {y}");

    let x = tup.0;

    println!("The value of x is: {x}");
}
