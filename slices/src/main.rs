fn main() {
    let s = String::from("Hello world");

    let world = first_word(&s);

    let second = second_word(&s);
    
    println!("The value {} {} {}", s, world, second);

    let a = [1, 2, 3, 4, 5];

    let slices = &a[0..3];
    for i in slices {
        println!("The slices content {}", i);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..s.len()];
        }
    }
    &s[..]
}