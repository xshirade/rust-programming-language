use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;
use std::error::Error;
use std::net::IpAddr;

fn main() -> Result<(), Box<dyn Error>> {
    {
        // panic!("crush and burn");
    }

    {
        // let v = vec![1,2,3];
        // v[100];
    }

    {
        // let f = File::open("hello.txt");
        // let f = match f {
        //     Ok(file) => file,
        //     Err(err) => panic!("Problem opening the file: {:?}", err)
        // };
    }

    {
        // let f = File::open("hello.txt");
        // let f = match f {
        //     Ok(file) => file,
        //     Err(err) => match err.kind() {
        //         ErrorKind::NotFound => match File::create("hello.txt") {
        //             Ok(file) => file,
        //             Err(err) => panic!("Problem opening the file: {:?}", err)
        //         },
        //         other_error => panic!("Problem opening the file: {:?}", other_error)
        //     }
        // };
    }

    {
        // let f = File::open("hello.txt").unwrap();
        // let f = File::open("hello.txt").expect("Failed to open hello.txt");
    }

    {
        read_username_from_file();
        shortcut_read_username_from_file();
        more_shortcut_read_username_from_file();
        most_shortcut_read_username_from_file();
    }

    {
        // Absolutely succeed unwrap() function in this case.
        let ip: IpAddr = "127.0.0.1".parse().unwrap();
    }

    {
        let f = File::open("hello.txt")?;
        Ok(())
    }
}

fn most_shortcut_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn more_shortcut_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn shortcut_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    // "?" operator replace the following three line`s behavior
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e)
    // };
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}


