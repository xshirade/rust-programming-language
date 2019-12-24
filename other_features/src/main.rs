fn main() {
    {
        fn largest_number(list: &[i32]) -> i32 {
            let mut largest: i32 = list[0];
            for &item in list.iter() {
                if largest < item {
                    largest = item;
                }
            }
            largest
        }

        fn largest_char(list: &[char]) -> char {
            let mut largest: char = list[0];
            for &item in list.iter() {
                if largest < item {
                    largest = item;
                }
            }
            largest
        }

        let v = vec![0,2,4,6,8,1,3,5,7,9];
        let largest = largest_number(&v);
        println!("The largest number is {}", largest);

        let v = vec!['a', 's', 'd', 'f', 'g', 'h', 'j'];
        let largest = largest_char(&v);
        println!("The largest char is {}", largest);
    }

    {
        // fn largest<T>(list: &[T]) -> T {
        //     let mut largest = list[0];
        //     for &item in list.iter() {
        //         if item > largest {
        //             largest = item;
        //         }
        //     }
        //     largest
        // }

        // let v = vec![0,2,4,6,8,1,3,5,7,9];
        // let result = largest(&v);
        // println!("The largest number is {}", result);

        // let v = vec!['a', 's', 'd', 'f', 'g', 'h', 'j'];
        // let result = largest(&v);
        // println!("The largest char is {}", result);
    }

    {
        struct Point<T> {
            x: T,
            y: T
        }
        let integer = Point { x: 1, y: 2 };
        let float = Point { x: 2.0, y: 4.0 };
        // let mix = Point { x: 3, y: 5.0 }; // error, x and y is not the same type
    }

    {
        struct Point<T, U> {
            x: T,
            y: U
        }
        let integer = Point { x: 1, y: 2 };
        let float = Point { x: 2.0, y: 4.0 };
        let mix = Point { x: 3, y: 5.0 };
    }

    {
        struct Point<T> {
            x: T,
            y: T
        }
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        let integer = Point { x: 1, y: 2 };
        println!("The method is called and the value is {}", integer.x());

        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        let float = Point { x: 4.4, y: 5.5 };
        println!("The distance from origin is {}", float.distance_from_origin());
    }

    {
        struct Point<T, U> {
            x: T,
            y: U
        }
        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y
                }
            }
        }
        let p1 = Point { x: 1.0, y: 2 };
        let p2 = Point { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);
        println!("p3.x: {}, p3.y: {}", p3.x, p3.y);
    }

    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String
        }
        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool
        }
        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        println!("1 new tweet: {}", tweet.summarize());
    }

    {
        pub trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more ...)")
            }
        }

        pub struct NewsArticle {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String
        }
        impl Summary for NewsArticle {}

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best
            hockey team in the NHL."),
        };
        println!("New article available! {}", article.summarize());
    }

    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        // notify 1 and 2 are the same
        pub fn notify1(item: impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
        pub fn notify2<T: Summary>(item: T) {}
        
        // notify 3 and 4 is the same
        pub fn notify3(item1: impl Summary, item2: impl Summary) {}
        pub fn notify4<T: Summary>(item1: T, item2: T) {}

        // multi traits
        pub trait Display {}
        pub fn notify5(item: impl Summary + Display) {}
        pub fn notify6<T: Summary + Display>(item: T) {}

        fn some_function1<T: Summary + Display, U: Summary>(t: T, u: U) {}
        fn some_function2<T, U>(t: T, u: U) where T: Summary + Display, U: Summary {}

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool
        }
        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }
        fn return_summarizable() -> impl Summary {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false
            }
        }
    }

    {
        use std::cmp::PartialOrd;
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];
            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let v = vec![0,2,4,6,8,1,3,5,7,9];
        let result = largest(&v);
        println!("The largest number is {}", result);

        let v = vec!['a', 's', 'd', 'f', 'g', 'h', 'j'];
        let result = largest(&v);
        println!("The largest char is {}", result);
    }

    {
        use std::fmt::Display;
        use std::cmp::PartialOrd;
        struct Pair<T> {
            x: T,
            y: T
        }
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }
        let p = Pair { x: 10, y: 20 };
        p.cmp_display();

        // impl<T: Display> ToString for T {}
    }

    {
        // fn longest(x: &str, y: &str) -> &str {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    {
        // fn longest<'a>(x: &str, y: &str) -> &'a str {
        //     let result = String::from("result");
        //     result.as_str()
        // }
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
        println!("{}", i.part);
    }

    {
        struct ImportantExcerpt<'a> {
            part: &'a str
        }
        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
        println!("{}", i.announce_and_return_part(first_sentence));
    }

    {
        // static lifetime: which is a reference living for entire time of the program
        let s: &'static str = "I have a static lifetime.";
    }

    {
        use std::fmt::Display;
        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
            println!("Annotation: {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        let longest = longest_with_an_announcement("abc", "efghij", "This is a test annotation");
        println!("The longest str: {}", longest);
    }
}
