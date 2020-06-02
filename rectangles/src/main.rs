#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
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

    // v4: use struct impl
    println!("The area of rectangle is {} square pixels.", rect2.area());

    let rect3 = Rectangle {
        width:20,
        height:40
    };

    let rect4 = Rectangle {
        width: 40,
        height: 50
    };

    println!("rect2 can hold rect3: {}", rect2.can_hold(&rect3));
    println!("rect2 can hold rect4: {}", rect2.can_hold(&rect4));

    // associate function
    let rect5 = Rectangle::square(30);
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
