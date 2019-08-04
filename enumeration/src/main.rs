enum IpAddrKind {
    V4,
    V6
}

fn main() {
    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        route(four);
        route(six);
    }

    {
        struct IpAddr {
            kind: IpAddrKind,
            address: String
        }
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1")
        };
        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1")
        };
    }

    {
        enum IpAddr {
            V4(String),
            V6(String)
        }
        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        #[derive(Debug)]
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String)
        }
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
        println!("v4 = {:#?}, v6 = {:#?}", home, loopback);
    }

    {
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32)
        }
        impl Message {
            fn call(&self) {
                // method body would be defined here
                println!("{:#?}", self)
            }
        }
        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
    }

    {
        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        // let sum = x + y; // error, Rust cannot add i8 to Option<i8>.
        // needs to extract value of T inside Option<T>
    }

    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    println!("Lucky Penny!");
                    1
                },
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25
            }
        }
    }

    {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska
        }
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState)
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quater from {:?}!", state);
                    25
                }
            }
        }
        value_in_cents(Coin::Quarter(UsState::Alaska));
    }

    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1)
            }
        }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        // println!("{}, {}, {}", five, six, none);
    }
}

fn route(ip_kind: IpAddrKind) {

}
