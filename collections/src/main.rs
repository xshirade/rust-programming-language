use std::collections::HashMap;

fn main() {
    {
        let v: Vec::<i32> = Vec::new();
        let v = vec![1, 2, 3];
    }

    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        let v = vec![1,2,3,4,5];
        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element")
        }
    }

    {
        let v = vec![1,2,3,4,5];
        // let does_not_exist = &v[100]; // error, index out of bounds
        let does_not_exist = v.get(100); // ok, this returns Optional<T>
    }

    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
            println!("{}", i);
        }
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String)
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12)
        ];
    }

    {
        let mut s = String::new();
        let data = "initial contents";
        let s = data.to_string();
        let s = "initial contents".to_string();
        println!("{}", s);
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{}", s);
        let s2 = "bar";
        s.push_str(s2);
        println!("{}", s2);
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2;
        println!("? + {} = {}", s2, s3);
    }

    {
        for c in "日本".chars() {
            println!("{}", c);
        }

        for b in "日本".bytes() {
            println!("{}", b);
        }
    }

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        for (k, v) in &scores {
            println!("{}, {}", k, v);
        }
    }

    {
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![100, 500];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        for (k, v) in &scores {
            println!("{}, {}", k, v);
        }
        let key = String::from("Blue");
        println!("{:?}", scores.get(&key));
    }

    {
        let field_name = String::from("Faborite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // println!("{}, {}", field_name, field_value); // error, namd and value vars are moved in the above insert method.
    }

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", &scores)
    }

    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1
        }
        println!("{:?}", &map)
    }
}
