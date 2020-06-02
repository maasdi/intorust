#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

fn main() {
    // v1
    let width = 30;
    let height = 50;

    println!("The area of rectangle is {} square pixels.", area_v1(width, height));

    // v2
    let rect1 = (30, 50);
    println!("The area of rectangle is {} square pixels.", area_v2(rect1));

    // v3
    let rect2 = Rectangle{
        width: 30,
        height: 50
    };
    // debug
    println!("rect2 is {:?}", rect2);
    // pretty debug
    println!("rect2 is {:#?}", rect2);
    println!("The area of rectangle is {} square pixels.", area_v3(&rect2));
}

fn area_v1(width: i32, height: i32) -> i32 {
    width * height
}

fn area_v2(dimension: (i32, i32)) -> i32 {
    dimension.0 * dimension.1
}

fn area_v3(rect: &Rectangle) -> i32 {
    rect.width * rect.height
}
