use std::io;

// consts (unlike like immutable variables) are compile-time constants.
// You cannot use mut with const
// You must ALWAYS annotate the type
// const can be declared in any scope (including global scope)
const MAX_POINTS: u32 = 100_000;

fn main() {
    variables_and_mutability();

    // Data Types
    floating_point_types();
    numeric_operations();
    boolean_type();
    character_type();
    tuple_type();
    array_type();

    // Functions
    another_function(-4, 3.3);

    // Control flow
    if_statement();
    loops();
    while_loops();
    for_loops();
}

fn variables_and_mutability() {
    println!("\n--Variables and Mutability--");
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    println!("Compile time const MAX_POINTS = {}", MAX_POINTS);

    // Shadowing creates a new (immutable) variable
    let shadow = 5;
    println!("The value of shadow is {}", shadow);
    let shadow = shadow + 1;
    println!("The value of shadow is {}", shadow);
    // It also allows us to change the value
    let shadow = "a string";
    println!("The value of shadow is {}", shadow);
}

fn floating_point_types() {
    println!("\n--Floating Point Types--");
    let f = 2.0; // f64
    println!("The value of f is {}", f);
    let f: f32 = 3.0; // f32
    println!("The value of f is {}", f);
}

fn numeric_operations() {
    println!("\n--Numeric Operations--");

    // Addition
    let operation = 5 + 10;
    println!("operation = {}", operation);

    // Subtraction
    let operation = 95.5 - 4.3;
    println!("operation = {}", operation);

    // Multiplication
    let operation = 4 * 30;
    println!("operation = {}", operation);

    // Division
    let operation = 56.7 / 32.2;
    println!("operation = {}", operation);

    // Remainder
    let operation = 43 % 5;
    println!("operation = {}", operation);
}

fn boolean_type() {
    println!("\n--Boolean Type--");
    let b = true;
    println!("bool = {}", b);
    let b: bool = false; // with explicit type annotation
    println!("bool = {}", b);
}

fn character_type() {
    println!("\n--Character Type--");
    let c = 'z';
    println!("char = {}", c);
    let c = '😻';
    println!("char = {}", c);
}

fn tuple_type() {
    println!("\n--Tuple Type--");
    let tup = (500, 6.4, 'z');

    // Destructuring
    let (x, y, z) = tup;

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);

    // Accessing directly
    println!("The value of tup.0 is {}", tup.0);
    println!("The value of tup.1 is {}", tup.1);
    println!("The value of tup.2 is {}", tup.2);
}

fn array_type() {
    println!("\n--Array Type--");

    // Arrays are useful when you want your data allocated on the stack not the heap
    let a: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Please enter an array index for a=[..days of month..].");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn another_function(param: i32, param2: f32) {
    println!("\n--Function--");

    println!("Another function parameter is {}", param);
    println!("Another function parameter 2 is {}", param2);

    // This expression is a block that evaluates to 4
    let y = {
        let x = 3;
        // Note: expressions do not end in semicolons
        x + 1
    };
    println!("The value of y is {}", y);

    println!("5 + 1 is {}", plus_one(5));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn if_statement() {
    println!("\n--If Statements--");

    let number = 6;

    if number < 5 {
        println!("{} is less than 5", number);
    } else if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is greater than 5", number);
    }

    // if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Value of condition is {}", number);
}

fn loops() {
    println!("\n--Loops--");

    let mut counter = 0;
    let result = loop {
        println!("Again");
        counter = counter + 1;
        if counter >= 5 {
            break counter * 2;
        }
    };

    println!("Loop result is {}", result);
}

fn while_loops() {
    println!("\n--While Loops--");

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");
}

fn for_loops() {
    println!("\n--For Loops");

    let a = [10, 20, 30, 40, 50];

    for elem in a.iter() {
        println!("The value is: {}", elem);
    }
}
