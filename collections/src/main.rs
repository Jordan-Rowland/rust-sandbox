#[allow(unused_variables)]

fn main() {
    // Vectors
    // Vectors can only store 1 type of value, but
    // can be added to or removed from.
    let v: Vec<i32> = Vec::new();

    // Useing a type annotation here because there are
    // no values yet, so the compiler cannot infer the
    // type. In more realistic code, Rust can infer the
    // type once you insert values, so you rarely need
    // to do this type annotation. Rust provides this
    // macro for convenience:
    let v = vec![1, 2, 3];

    // Updating a vector
    // As with any value, if we want to make it mutable
    // we need to use the `mut` keyword.
    let mut v = Vec::new();

    v.push(8);
    v.push(6);
    v.push(75);

    println!("{:?}", v);

    // Like any other struct a vector is freed when it
    // goes out of scope.

    // Reading elements of vectors
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // `.get` with index returns an Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    if let Some(third) = v.get(2) {
        println!("The third value from if let is {}", third)
    }

    // the above is saying,
    // if v.get(2) == Some(<value>), then println!(x)

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    // When using index notation(`[]`) to access a value
    // that does not exist, Rust will `panic`. This is
    // best used when you want the program to crash
    // if there's an attempt to access an element past
    // the end of the vector.

    // When using the get method outside the vector, it
    // returns None without panicking. You would use this
    // method if accessing an element beyond the range of
    // the vector happens occasionally under normal
    // circumstances.

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", &v);

    // Using an enum to store multiple types
    // Variants of an enum are defined under the same
    // type:

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.23),
    ];

    // Strings
    #[allow(unused_mut)]
    let mut s = String::new();
    // This line creates a new empty string called `s`
    // which we can then load data into.

    // Updating Strings
    let mut s = String::from("Hello ");
    s.push_str("World!");

    // the `.push()` method takes a single character
    // and adds it to the `String`.
    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = "World!".to_string();
    let s3 = s1 + &s2; // Note: s1 has been moved here

    // The reason s1 is moved and s2 is a reference has
    // to do with the signature method that gets called
    // when we use the `+` operator. The `+` operator
    // uses the add method, which looks like:
    // fn add(self, s: &str) -> String {};

    // Better method is with `format!` macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    // `format!` does not take ownership of any of it's
    // parameters.

    println!("{}, {},{},{}", s, s1, s2, s3);

    // Methods for iterating over strings
    // for c in "नमस्ते".chars() {
    //     println!("{}", c);
    // }

    // for b in "नमस्ते".bytes() {
    //     println!("{}", b);
    // }

    // Storing keys with associated values in Hash Maps
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Just like vectors, hash maps store their data on
    // the heap. Like vectors, hash maps are homogenous:
    // all of the keyes must have the same type, and all
    // of the values must have the same type.

    // Another way of constructing a hash map is by
    // using the `collect` method on a vector of
    // tuples, where each tuple consists of a key and
    // its value. The `collect` method gathers data into a
    // number of collection types. For example, if we had
    // the team names and scores in two separate vectors,
    // we could use the `zip` method to create a vector
    // of tuples:

    let teams = vec![String::from("Blue")];
    let initial_scores = vec![10];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // The type annotations HashMap<_, _> is needed here
    // because it's possible to collect into many different
    // data structures and Rust doesn't know which you want
    // unless you specify. For the parameters for the key and
    // value types, however, we can use underscores, and
    // Rust can infer the types that the hash map contains
    // based on the types of the data in the vectors.

    // For types that implement the Copy trait, like i32,
    // the values are copied into the hash map. For owned
    // values like String, the values will be moved and the
    // hash map will be the owner of those values.

    // Accessing values in a hash map
    // println!("{:?}", scores)
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    let team_name = String::from("Blue");

    let score = scores.get(&team_name);
    println!("{:?}", score);

    // Updating a hash map //

    // Overwriting a value
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Only Inserting a Value If the Key Has No Value
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // This code will print {"world": 2, "hello": 1,
    // "wonderful": 1}. The or_insert method actually
    // returns a mutable reference (&mut V) to the value
    // for this key. Here we store that mutable reference
    // in the count variable, so in order to assign to that
    // value, we must first dereference count using the
    // asterisk (*). The mutable reference goes out of
    // scope at the end of the for loop, so all of these
    // changes are safe and allowed by the borrowing rules.

    // use std::convert::TryInto;
    //
    fn sum(iter: &Vec<i32>) -> i32 {
        let mut n: i32 = 0;

        for i in iter {
            n += i;
        }

        n
    }

    fn mean(iter: &Vec<i32>) -> i32 {
        let iter_length = iter.len();
        let s = sum(iter);
        s / iter_length as i32
    }

    fn median(iter: &mut Vec<i32>) -> i32 {
        iter.sort();
        let iter_length: usize = iter.len();
        let ind: usize = iter_length / 2 as usize;
        iter[ind]
    }

    fn mode(iter: &Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for i in iter {
            let num = map.entry(i).or_insert(0);
            *num += 1
        }
        let mut most_freq_num = 0;
        let mut most_freq = 0;
        for (num, count) in map {
            println!("{}: {}", num, count);
            if count > most_freq {
                most_freq = count;
                most_freq_num = *num;
            }
        }
        most_freq_num
    }

    let v = vec![1, 2, 3, 4, 5, 5, 2, 8, 3, 6, 7, 8];
    let mut v2 = vec![9, 3, 5, 7, 2, 22, 1];
    let s = sum(&v);
    let m = mean(&v);
    let md = median(&mut v2);
    let mo = mode(&v);

    println!("sum: {}", s);
    println!("mean: {}", m);
    println!("median: {}", md);
    println!("mode: {:?}", mo);

    fn igpay(string: &String) -> String {
        let strings = string.split_whitespace();
        let mut new_string = String::new();
        let vowels = vec!["a", "e", "i", "o", "u"];
        let mut new_word: String;
        for s in strings {
            let first = s[..1].to_string();
            let rest = s[1..].to_string();
            if vowels.contains(&&first[..]) {
                new_word = format!("{}-hay ", s);
            } else {
                new_word = format!("{}{}-ay ", rest, first);
            }
            // new_strings_vec.push(new_word)
            new_string.push_str(&new_word);
        }
        new_string
    }

    let igpays = igpay(&String::from("Speaking pig latan is super fun"));
    println!("{}", igpays);

    use std::collections::HashMap;
    use std::io;

    #[allow(unused_mut)]
    let mut dept_emps_map = HashMap::<String, Vec<String>>::new();

    #[derive(Debug)]
    enum Action {
        Add,
        List,
        Exit,
    }

    let mut action_map = HashMap::new();
    action_map.insert(String::from("add"), Action::Add);
    action_map.insert(String::from("list"), Action::List);
    action_map.insert(String::from("exit"), Action::Exit);

    loop {
        let mut action = String::new();
        println!("\nWhat would you like to do? (add, list, exit)");
        io::stdin()
            .read_line(&mut action)
            .expect("failed to read line");

        let action_trim = action.trim();

        if action_trim == "add".to_string() {
            let mut dept = String::new();
            let mut empl = String::new();

            println!("\nWhat department would you like to add an employee to:");
            io::stdin()
                .read_line(&mut dept)
                .expect("failed to read line");

            println!("Employee name:");
            io::stdin()
                .read_line(&mut empl)
                .expect("failed to read line");

            let dept = dept.trim().to_string();
            let empl = empl.trim().to_string();

            dept_emps_map.entry(dept.clone()).or_insert(Vec::new());

            let dept_emps = match dept_emps_map.get_mut(&dept) {
                Some(v) => v.push(empl),
                None => println!("Could not add new employee"),
            };

            println!("{:?}", dept_emps_map);
        } else if action_trim == "list".to_string() {
            let mut dept_input = String::new();
            println!("\nWhat department like see:");
            io::stdin()
                .read_line(&mut dept_input)
                .expect("failed to read line");

            if let Some(v) = dept_emps_map.get_mut(dept_input.trim()) {
                v.sort();
                for i in v {
                    println!("\t{}", i);
                }
            }
        } else if action_trim == "exit".to_string() {
            use std::process;
            process::exit(0x0100);
        }
    }
}
