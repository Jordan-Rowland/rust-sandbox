// use isahc::prelude::*;
// fn main() {
//     // let mut response = isahc::get("https://jsonplaceholder.typicode.com/todos/1").unwrap();
//     let mut response = isahc::get("https://jsonplaceholder.typicode.com/todos/1").unwrap();
//     println!("{:?}", response.text().unwrap());
// }

// use isahc::prelude::*;
// use std::fs::File;

// Download a file
// fn main() {
//     let mut res = isahc::get("https://i.redd.it/kqj64iqen0r41.jpg").expect("request failed");
//     let mut out = File::create("kqj64iqen0r41.jpg").expect("failed to create file");
//     res.copy_to(&mut out).expect("failed to copy content");
// }

struct Tweet {
    username: String,
    body: String,
}

struct FBPost {
    username: String,
    body: String,
}

trait Post {
    // fn post(&self) -> String;
    fn post(&self) -> String {
        format!("{} - {}", self.body(), self.username())
    }

    fn username(&self) -> &str;
    fn body(&self) -> &str;
}

impl Post for Tweet {
    fn post(&self) -> String {
        format!("@{}: {}", self.username(), self.body())
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn body(&self) -> &str {
        &self.body
    }
}

impl Post for FBPost {
    fn post(&self) -> String {
        format!("{} - {}", self.body(), self.username())
    }

    fn body(&self) -> &str {
        &self.body
    }

    fn username(&self) -> &str {
        &self.username
    }
}

fn main() {
    let t = Tweet {
        username: "FatBoi420".to_string(),
        body: "I'm ao fat and I love weed".to_string(),
    };
    let f = FBPost {
        username: "lameguy".to_string(),
        body: "heall yeah dude".to_string(),
    };
    println!("{}", t.post());
    println!("{}", f.post());
}
