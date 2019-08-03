struct User {
    username: String,
    email: String,
    sign_in_count: i32,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { height: size, width: size }
    }
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    {
        let user1 = User {
            username: String::from("someone@example.com"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
            active: true
        };
        println!("{}", user1.email);
    }

    {
        let mut user1 = User {
            username: String::from("someone@example.com"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
            active: true
        };
        user1.email = String::from("another_one@exmaple.com");
        println!("{}", user1.email);
    }

    {
        let user1 = build_user(
            String::from("someone@exmaple.com"),
            String::from("someone@exmaple.com")
        );
        let user2 = User {
            username: String::from("another_one@exmaple.com"),
            email: String::from("another_one@exmaple.com"),
            sign_in_count: user1.sign_in_count,
            active: user1.active
        };
        let user2 = User {
            username: String::from("another_one@exmaple.com"),
            //email: String::from("another_one@exmaple.com"),
            ..user1
        };
        println!("{}", user2.email);
    }

    // pattern 1
    // {
    //     let height = 30;
    //     let width = 50;
    //     println!(
    //         "The area of the rectangle is {} square pixel",
    //         area(height, width)
    //     );
    // }

    // pattern 2
    // {
    //     let rect = (30, 50);
    //     println!(
    //         "The area of the rectangle is {} suqare pixel",
    //         area(rect)
    //     );
    // }

    // pattern 3
    // {
    //     let rect = Rectangle {
    //         height: 30,
    //         width: 50
    //     };
    //     println!("{}", area(&rect));
    // }
    
    {
        let rect = Rectangle {
            height: 30,
            width: 50
        };
        let rect2 = Rectangle::square(3);
        println!("{:#?}", rect);
        println!("{}", rect.area());
        println!("rect can hold rect2: {}", rect.can_hold(&rect2));
    }
}

// pattern 1
// fn area(height: u32, width: u32) -> u32 {
//     height * width
// }

// pattern 2
// fn area(rect: (u32, u32)) -> u32 {
//     rect.0 * rect.1
// }

// pattern 3
// fn area(rect: &Rectangle) -> u32 {
//     rect.height * rect.width
// }

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}
