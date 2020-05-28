fn main() {
    let x = five();
    println!("The value of x is {}", x);

    let x = plus_one(2);
    println!("The value of x is {}", x);

    let y = {
        let x =3;
        x + 1
    };

    println!("The value of y is {}", y);

    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}