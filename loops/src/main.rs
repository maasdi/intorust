fn main() {
    let mut counter = 0;

    // loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    // while
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Done!");

    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is {}", array[index]);
        index += 1;
    }

    // for
    for element in array.iter() {
        println!("The value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("Done");
}
