fn main() {
    // `panic!` macro
    // panic!("Halt and catch fire");

    // other ways to cause panic
    // let v = vec![1, 2, 3];

    // v[99];

    // `Result`

    use std::fs::File;
    let f = File::open("hello.txt");

    use std::io::ErrorKind;

    // With nested `match`
    // Fig 9.4
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // with `if - else` (cleaner implementation)
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("Problem creating file: {}", error))
        } else {
            panic!("Problem opening file: {}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect

    // Using `match` works well, but it can be verbose and
    // doesn't always communicate intent well. The Result<T, E>
    // type has many helper methods defined on it to do various
    // tasks. One of those methods, `unwrap` is a shortcut
    // method that is implemented just like the match expression
    // written in fig 9.4. If the result is the Ok valiant, `unwrap`
    // will return the value inside, and if it is the Err
    // varient, it will call the `panic!` macro for us.

    // use std::fs::File;
    let f = File::open("hello.txt").unwrap();

    // Another method, `expect()` is similar to unwrap, but
    // lets us choose the `panic!` message. Using `expect`
    // instad of `unwrap` and providing good error messages
    // can convey your intent and make tracking down the source
    // of panic easier.

    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // We use `expect` in ther same was a `unwrap`: to return
    // the file handle or call the `panic!` macro. The error
    // message used by `expect` will be the message `panic!`
    // gets called with, rather than the default `panic!` message
    // unwrap uses.

    // Propagating errors

    // When you're writing a function whose implementation calls
    // something that might fail, instad of handling the error
    // within this function, you can return the error to the
    // calling code so that is can decide what to do. This is
    // known as propagating the error and gives more control to
    // the calling code, where there might be more information
    // or logic that dictates hoe the error should be handled
    // than what you have available in the context of your code.

    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    // The above function can be written in a much shorter
    // way:
    // A Shortcut for Propagating Errors: the ? Operator
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    /*
    The `?` placed after `Result` value is defined to work in
    almost the same way as the match expressions defined to
    handle the Result values above. If the value of the Result
    is an Ok, the value inside the Ok will get returned from this
    expression, and the program will continue. If the value is
    an Err, the Err will be returned from the whole function as
    if we had used the return keyword so the error value gets
    propagated to the calling code.

    This code is made even shorter with chaining
    */

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }
    /*
    Even shorter:
    */

    // use std::io;
    // use std::fs;

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}
