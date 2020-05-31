fn main() {
    let mut s1 = String::from("Hello");

    let length = calculate_length(&s1);

    println!("The length of string {} is {}", s1, length);

    change(&mut s1);
    println!("The value is {}", s1);

    {
        let t1 = &mut s1;
        println!("The value {}", t1);
    }
    let t2 = &mut s1;
    println!("The value {}", t2);

    let dangled = dangled();
    println!("The value {}", dangled);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangled() -> String {
    let s = String::from("hello");

    s
}