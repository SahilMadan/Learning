use std::collections::HashMap;

fn main() {
    vectors();
    strings();
    hash_maps();
}

fn vectors() {
    println!("\n--Vectors--");

    // Infer type as Vec<i32>
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        println!("{:?}", v);
        // Like any other struct, v goes out of scope and is freed here
    }
    // println!("{:?}", v);

    let mut v = vec![1, 2, 3];

    // Indexing
    // Causes program to panic for invalid index
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // get method
    // Invalid index handles by None case
    match v.get(1) {
        Some(num) => println!("The second element is {}", num),
        None => (),
    }

    // Since third is still in scope, this won't work.
    // v.push(4);
    println!("{}", third);
    // But it will work here since third isn't used anymore and its scope ends
    v.push(4);

    // Iterating over immutable refs
    for i in &v {
        println!("Value of v: {}", i);
    }

    // Iterating over mutable refs
    for i in &mut v {
        // Use the dereference operator to modify...
        *i += 10;
        // But not here...
        println!("Value of v: {}", i);
    }

    // Storing different values in an array using enums
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Value of row: {:?}", row);
}

fn strings() {
    println!("\n--Strings--");

    // Growable, mutable, owned UTF-8 string

    let data = "Initial contents";
    // to_string() is available to any type that implements the Display trait
    let _s = data.to_string();
    let _s = "Initial contents".to_string();
    let mut s = String::from("Initial contents");

    s.push_str(", and more");
    s.push('!');
    let s2 = "!!".to_string();

    // Signature: fn add(self, s: &str) -> String
    // Note: &String is coerced into &str type
    let s3 = s + &s2;
    println!("{}", s3);

    // s has been moved and can no longer be moved
    // println!("{}", s);

    // Note: You cannot index into a string
    // Why? Because unicode characters aren't always one byte, this is 24 bytes not 12
    let hello = String::from("Здравствуйте");
    // println!("{}", &_hello[0]);

    // Slicing unicode strings
    // This will fail because we're not inside a char boundary, index 3 is in middle of char
    // let s = &hello[0..3];
    let s = &hello[0..4];
    println!("{}", s);
    // Conclusion: Use ranges to create string slices with CAUTION

    // Iterating
    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }
}

fn hash_maps() {
    println!("\n--Hash Maps--");

    // Constructing new hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 5);

    let teams = vec![String::from("blue"), String::from("yellow")];
    let initial_scores = vec![10, 5];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);

    let field_name = String::from("green");
    scores.insert(field_name, 7);

    // field_name is now invalid
    // println!("{}", field_name);

    // Accessing value
    match scores.get("blue") {
        Some(v) => println!("The value of the blue team is {}", v),
        None => (),
    };

    for (key, value) in &scores {
        println!("{} = {}", key, value);
    }

    // Replacing a value
    scores.insert("blue".to_string(), 25);

    // Only inserting a value if the key has no value
    scores.entry("blue".to_string()).or_insert(9);
    scores.entry("red".to_string()).or_insert(9);
    for (key, value) in &scores {
        println!("{} = {}", key, value);
    }

    // Inserting a value based on the old value
    let score = scores.entry("red".to_string()).or_insert(9);
    *score += 1;
    match scores.get("red") {
        Some(v) => println!("The value of the red team is {}", v),
        None => (),
    };
}
