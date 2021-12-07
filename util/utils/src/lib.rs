// use reqwest::blocking::Client;
// use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
// use shellexpand::tilde;
// use std::path::Path;
// use cookie_store::CookieStore;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// DEPENDENCIES:
// reqwest = { version = "*", features = ["blocking", "cookies"] }
// tokio = { version = "*", features = ["full"] }
// hyper = "*"
// shellexpand = "*"
// cookie_store = "*"

// pub fn get_input(day: String) {
    // let mut headers = HeaderMap::new();

    // let filepath = tilde("~/.session").to_string();
    // let contents = fs::read_to_string(filepath)
    //     .expect("Unable to open session file");

    // let session = format!("session={}", contents.trim());
    // headers.insert(COOKIE, HeaderValue::from_str(&session).unwrap());
    // let cookie_store = CookieStore::new();
    // cookie_store.insert()

    // let url = format!("http://adventofcode.com/2021/day/{}/input", day);
    // let client = Client::builder().cookie_provider(&cookie_store).build().unwrap();
    // let response = client.get(url).headers(headers);
    // println!("{:?}", response);
    // println!("{:?}", response.send().unwrap().text());
// }

pub fn get_input(day: u8, real: bool) -> Vec<String> {
    let filename = match real {
        false => "example",
        true => "input"
    };
    if let Ok(lines) = read_lines(format!("{}/{}.txt", day, filename)) {
        lines.map(|line| line.unwrap()).collect()
    }
    else {
        panic!("Failed to read input")
    }
}

// file reading courtesy of https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        get_input(String::from("6"));
        assert!(false);
    }
}
