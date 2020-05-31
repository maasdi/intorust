fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("The value of s2 is {}", s2);
    // s1 already invalid

    let s3 = s2.clone();
    println!("Ts2: {}, s3: {}", s2, s3);

    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    take_owenership(s3);
    // s3 already invalid

    makes_copy(x);
    println!("still valid x: {}", x);
}

fn take_owenership(some_string: String) {
    println!("some_string: {}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("some_int: {}", some_int);
}