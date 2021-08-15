fn main() {
    variable_scope();
    string_type();

    println!("\n--Functions and Ownership--");
    let v = String::from("test");
    takes_ownership(v);
    // v.push_str("won't work"); // v has been moved
    let v = 5;
    makes_copy(v);
    println!("{}", v);

    println!("\n--Return Values and Scope--");
    let v = gives_ownership();
    let v = takes_and_gives_back(v);
    println!("{}", v);

    println!("\n--References and Borrowing--");
    let mut v = String::from("hello");
    let len = calculate_length(&v);
    println!("{}'s length is {}", v, len);
    change(&mut v);
    println!("{}", v);

    // Restriction on mutable refs is you can only have one borrow per particular scope
    let _r1 = &mut v;
    // Doesn't work:
    // let _r2 = &mut v;
    // println!("{} and {}", _r1, _r2);

    // Note: Scope of _r1 ends here because it is not used again

    // Resriction does not apply to immutable refs
    let r1 = &v;
    let r2 = &v;
    println!("{} and {}", r1, r2);


    println!("\n--Slice Type--");
    string_slice();
    array_slice_type();

}

fn variable_scope() {
    println!("\n--Variable Scope--");

    // When s comes into scope it is valid. When its scope ends, it is invalid.
    {
        let s = "hello";
        println!("The scope of s ends here: {}", s);
    }
    // println!("s does not exist here {}", s);
}

fn string_type() {
    println!("\n--String Type--");

    // String literals are hardcoded into our program. They're immutable.
    // String types are immutable, and allow us to extend for strings not known at compile-time.
    let mut s = String::from("hello");

    // In order to support string literals being mutable (and growing), we need to allocate an
    // amount of memory on the heap.
    s.push_str(" world!");
    println!("{}", s);

    // Note that the memory allocated to s on the heap is automatically returned when the variable
    // goes out of scope. The function used is called drop(). Similar to RAII.

    // This copies pointer, length, and capacity; not the underlying data.
    // s is considered move; it is not longer valid to use it. Ownership has been transferred.
    let s2 = s;
    println!("{}", s2);

    // If we want a deep-copy, we can clone the data.
    let s1 = s2.clone();
    // Now both s1 and s2 are valid and point to different allocated memory.
    println!("\"{}\" and \"{}\"", s1, s2);

    // Note: Types that implement the 'Copy' trait are stored on the stack.
    // There is no difference between shallow and deep copy
    let x = 5;
    let y = x;
    println!("{} and {}", x, y);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
    // At the end of this function, some_string goes out of scope and the backing memory is freed.
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // Since we do not have ownership of s, only ref, going out of scope does not free backing
    // memory
}

fn change(s: &mut String) {
    s.push_str(" world!");
}

fn string_slice() {
    let s = String::from("hello world!");

    let hello = &s[..5];
    let world = &s[6..11];
    let end = &s[11..];

    println!("{} {} {}", hello, world, end);

    let slice_entire_string = &s[..];
    println!("{}", slice_entire_string);

    // Note: String literals are slices! It's a slice pointing to a specific point in the binary,
    // and has the same type as any other slice: &str.
    let literal = "hello World";

    // Writing our first_word function to take string slice parameter allows us to use slices and
    // literals.
    println!("{}", first_word(literal));
    println!("{}", first_word(&s));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}

fn array_slice_type() {
    let a = [1, 2, 3, 4, 5];
    // Array slice type has type &[i32]
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
