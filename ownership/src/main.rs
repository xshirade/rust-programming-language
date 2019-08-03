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

    {
        let s1 = String::from("Calculate String Length");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
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

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
