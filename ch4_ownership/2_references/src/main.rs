fn main() {
    fun_1();
    fun_2();
    fun_3();
}

fn fun_1() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn fun_2() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn fun_3() {
    let mut s = String::from("hello");

    // let r1 = &mut s; // Cannot have multiple mutable references like this
    // let r2 = &mut s; 

    // Use curly brace scopes to allow for this, which prevents simultaneous mutable references
    {
        let r1 = &mut s;
    }
    let r2 = &mut s; 
    
}
