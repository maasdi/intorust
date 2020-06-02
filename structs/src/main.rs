struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let user1 = build_user(
        String::from("maas.dianto@test.com"),
        String::from("maas.dianto@test.com"),
    );
    println!(
        "Username: {}, email: {}, sign_in_count: {}, active: {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );

    let mut user2 = build_user(
        String::from("maas.dianto@test.com"),
        String::from("maas.dianto@test.com"),
    );

    // mutable
    user2.email = String::from("override.email@test.com");
    println!(
        "Username: {}, email: {}, sign_in_count: {}, active: {}",
        user2.username, user2.email, user2.sign_in_count, user2.active
    );

    // struct update syntax
    let user3 = User {
        username: String::from("user3@test.com"),
        email: String::from("user3@test.com"),
        ..user1
    };
    println!(
        "Username: {}, email: {}, sign_in_count: {}, active: {}",
        user3.username, user3.email, user3.sign_in_count, user3.active
    );

    // tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color: {} {} {}", black.0, black.1, black.2);

    println!("Point: {} {} {}", origin.0, origin.1, origin.2);

}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
