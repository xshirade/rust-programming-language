fn main() {
    {
        let _s = "hello";
        // The following s is on the heap but immutable.
        let _s = String::from("Hello");
        // s.push_str(", world!"); // this is not working because s is not declared as muttable in the above 
        let mut s = String::from("Hello");
        s.push_str(", world!");
        println!("{}", s);
    }

    {
        let x = 5;
        let y = x;
        println!("x: {}, y: {}", x, y);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}!", s1); // this cause error because s1 is already invalid when move s1 to s2
        println!("{}", s2);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let s = String::from("hello");
        takes_ownership(s);
        // println!("{}", s); // String has no Copy trait

        let x = 5;
        makes_copy(x);
    }

    {
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        // println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3); // s2 is already moved
    }

    // {
    //     let s1 = String::from("Calculate String Length");
    //     let (s2, len) = calculate_length(s1);
    //     println!("The length of '{}' is {}.", s2, len);
    // }

    {
        let s1 = String::from("Thw ownership is not moved.");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let mut s = String::from("hello");
        change(&mut s);
    }

    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            // let r2 = &mut s; // why this does not output error ? errors when r1 or r2 used
            // println!("{}, {}", r1, r2);
        }
        let r2 = &mut s;
    }

    {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{}, {}", r1, r2);
        // this is okay, r3 is declared after r1 and r2 used
        let r3 = &mut s;
        println!("{}", r3);
    }

    {
        let refernce_to_nothing = danlge();
    }

    {
        let mut s = String::from("hello  world");
        let word = first_word(&s);
        // s.clear(); // error, word is immutable ref.
        println!("{}", word);
    }

    {
        let my_string = String::from("hello world");
        let word = first_word(&my_string[..]);

        let my_string_literal = "Hello, World!";
        let word = first_word(&my_string_literal[..]);
        // my_string_literal is already in type of &str
        let word = first_word(my_string_literal);
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

fn danlge() -> String {
    let s = String::from("hello");
    // &s // this cause error, &s refers to s which is dropped after return &s
    s
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
