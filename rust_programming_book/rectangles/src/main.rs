// Struct
// Note the derive annotation trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Tuple Struct
struct Vector(u32, u32, u32);

struct User {
    username: String,
    email: String,
    // address: &str, // Must have ownership of data (cannot be slice)
}

// Store the IP Address (string) inside the enum itself (instead of using a struct with both enum
// and String.
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Struct differs from a tuple in that each field is named.
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Note that _user has ownership of strings
    let user = User {
        username: String::from("name"),
        email: String::from("email@gmail.com"),
    };
    println!("User name is {}, email is {}", user.username, user.email);

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );

    // Tuple structs are like structs except no need for named data
    let vec1 = Vector(1, 3, 4);
    let vec2 = Vector(5, 1, 2);
    println!("The dot-product of vec1 and vec2 is {}", dot(vec1, vec2));

    // Printing with traits
    // Putting the :? specifier tells println! we want to use the output format called Debug
    println!("rect1 is {:?}", rect1);
    // We can also use :#? for prettier output for larger structs
    println!("rect1 is {:#?}", rect1);

    // Using struct method instead
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );

    println!("Square example: {:?}", Rectangle::square(5));

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    // Option
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // To add x and y, we need to convert Option to a value first
    // Note: matches must be exhaustive, but you can use _ as a default case.
    let sum = x + match y {
        Some(num) => num,
        None => 0,
    };
    println!("sum = {}", sum);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        // This pattern matches any value
        _ => (),
    }
    println!("some_u8_value = {}", some_u8_value);

    // if let: What if we want to handle only when Some = 5 and no other cases?
    // Wordy method
    match y {
        Some(5) => println!("y equals 5"),
        _ => println!("y does not equal 5"),
    }

    // Less-wordy if/let method
    if let Some(5) = y {
        println!("y equals 5");
    } else {
        println!("y does not equal 5");
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn dot(vec1: Vector, vec2: Vector) -> u32 {
    vec1.0 * vec2.0 + vec1.1 * vec2.1 + vec1.2 * vec2.2
}

// Method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// It is valid to have multiple impl blocks
impl Rectangle {
    // Associated function does not take self and don't have an instance of the object to work on.
    // Often used by constructors.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
