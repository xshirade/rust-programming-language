fn main() {
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    let x = x + 1;
    let x = x * 2;
    println!("The value of x is : {}", x);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    println!("{}, {}, {}", five_hundred, x.0, x.2);

    let a: [i32; 5] = [1,2,3,4,5];
    let first_value = a[0];
    println!("First value is: {}", first_value);
    let a = [3; 5];
    let second_value = a[1];
    println!("First value is: {}", second_value);

    let x: i32 = 1024;
    another_function(x, a[3]);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);

    let x = 5;
    let x = plus_one(x);
    println!("The value of x: {}", x);

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    };

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is : {}", a[index]);
        index += 1;
    };

    for element in a.iter() {
        println!("The value is : {}", element);
    };

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32, y: i32) {
    println!("Another functions is called.");
    println!("The value of x: {}", x);
    println!("The value of y: {}", y);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
