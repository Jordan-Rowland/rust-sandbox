fn main() {
    #![allow(unused_variables)]
    #![allow(dead_code)]

    enum IpAddrKind {
        V4,
        V6,
    }

    fn route(ip_kind: IpAddrKind) {}

    // Instead of Struct, store data directly into enum
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // Standard lib implementation
    // struct Ipv4Addr {
    //     // --snip--
    // }

    // struct Ipv6Addr {
    //     // --snip--
    // }

    // enum IpAddr {
    //     V4(Ipv4Addr),
    //     V6(Ipv6Addr),
    // }

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangColor(i32, i32, i32),
    }

    // enums can have implementations too
    impl Message {
        fn call(&self) {
            // method body here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // `Option` enum
    // Defined in std lib as:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let z = absent_number.is_none();
    println!("{}", z); // true

    // match control flow
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter,
    // }

    // fn value_in_cents(coin: Coin) -> u8 {
    //     match coin {
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    // }

    // `match` is different that `if` because `if` needs to return
    // a bool value but with `match` it can be any type.
    // The type of coin in this example is the Coin enum defined
    // above.

    // The `match` arms has 2 parts: a pattern and come code. The
    // first arm here has a pattern that is the value `Coin::Penny`,
    // and then the `=>` operator that separates the pattern and the
    // code to run. (This is different from JS where the values to the
    // left of the `=>` operator are arguments). The code on the right
    // of the fat arrow is the value returned.

    // let c = Coin::Dime;
    // println!("{}", value_in_cents(c));

    // When the match expression executes, it compares the resulting value
    // agains the pattern of each arm, in order. If a pattern matches the value,
    // the code associated with that pattern is executed. If the pattern
    // doesn't match the value, execution continues to the next arm, just
    // like in a coin-sorting machine. We can have as many arms as we need.

    // The code associated with each arm is an expression, and the resulting value
    // of the expression in the matching arm is the value that gets returned for thread::spawn(move || {
    // entire `match` expression.

    // Patterns that Bind to Values
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Matching with Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            // variable / argument for fat arrow functions, (i)
            // "match against an enum, bind a variable to the data
            // inside, and then execute code based on it"
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // The _ Placeholder
    // Rust also has a pattern to use when we don't want to list
    // all possible values. Because `match` is exhaustive, if we only want
    // to check for a subset of values and don't want to list them all,
    // we can use `_` insteadthread::spawn(move || {

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // if let - (like Python's walrus operator :=)

    // not this
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // this
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // The syntax `if let` takes a pattern and an expression
    // separated by an equal sign. It works the same
    // way as a `match`, where the expression is given
    // to the `match` and the pattern is it's first arm.

    // Using `if let` means less typing but loses the
    // exhaustive checking that `match` enforces. Choosing
    // between `match` and `if let` depends on what you're
    // doing in your particular situation and whether
    // gaining conciseness is an appropriate trade-off
    // for losing exhaustive checking.

    // `if let` is syntactic sugar for a `match` that runs
    // code when the value matches one pattern and ignores
    // all others.

    // We can also include an `else` block with an
    // `if let`. The block of code that goes with
    // the `else` is the same as the block that would
    // go with the _ case in the match expression.

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("{:?}", state),
        _ => count += 1,
    }

    // or we could use `if let` and `else`:
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("{:?}", state);
    } else {
        count += 1;
    }
}
