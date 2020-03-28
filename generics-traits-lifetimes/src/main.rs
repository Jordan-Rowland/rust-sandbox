fn main() {
    // Generic types, traits, and lifetimes

    /*

    Generics are abstract stand-ins for concrete types or other
    properties. When writing code, we can express the behaviour
    of generics or how they relate to other generics without
    knowing what will be in thier place when compiling and
    running the code.

    e.g.
    Result<T, E>

    Let’s first look at how to remove duplication that
    doesn’t involve generic types by extracting a function.

    */
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    // To eliminate duplication,
    // we can extract to a function

    fn largest(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &number in list {
            if number > largest {
                largest = number;
            }
        }
        largest
    }
    /**
     * We use the same steps with generics to reduce code
     * duplication in different ways. For example, if we
     * had 2 functions that finds the largest item in a
     * slice of `i32` values, and the largest in a slice of
     * `char` values.
     *
     * When defining a function that uses generics, we
     * place the generics in the signature of the function
     * where we would usually specify the data types of the
     * parameters and return value. Doing so makes our code
     * more flexible and provides more functionality to
     * callers of our function while preventing code
     * duplication.
     */
    // the two version of this function for different
    // types:

    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    /**
    * To parameterize the types in the new function
    * we’ll define, we need to name the type parameter,
    * just as we do for the value parameters to a
    * function. You can use any identifier as a type
    * parameter name. But we’ll use T because, by
    * convention, parameter names in Rust are short,
    * often just a letter, and Rust’s type-naming
    * convention is CamelCase. Short for “type,” T
    * is the default choice of most Rust programmers.

      When we use a parameter in the body of the
      function, we have to declare the parameter
      name in the signature so the compiler knows
      what that name means.

      To define the generic largest function, place
      type name declarations inside angle brackets,
      <>, between the name of the function and the
      parameter list, like this:
    */
    // fn largest<T>(list: &[T]) -> T {
    //     let mut largest = list[0];

    //     for &item in list.iter() {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //     largest
    // }

    /**
     * The above will not compile however because
     * std::cmp::PartialOrd is not implemented on generic
     * <T> traits.
     */
    // In Struct Definitions

    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // You can also define 2 a struct with 2 different
    // generic types:
    struct Point<T, U> {
        x: T,
        y: U,
    }

    fn main() {
        let both_integer = Point { x: 5, y: 10 };
        let both_float = Point { x: 1.0, y: 4.0 };
        let integer_and_float = Point { x: 5, y: 4.0 };
    }

    // In method definitions
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    /**
     * Note we have to decalre T just after `impl` so
     * we can use it to specify that we're implementing
     * methods on the type Point<T>. By declaring T as a
     * generic type after `impl`, Rust can identify that
     * the type in the angle brackets in Point is a
     * generic type rather than a concrete type.
     *
     * We could, for example, implement methods only on
     * Point<f32> instances rather than on Point<T> instances
     * with any generic type.
     */

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    /**
     * This code means the type Point<f32> will have a
     * method named `distance_from_origin` and other
     * instances of Point<T> where T is not of type f32
     * will not have this method defined. The method
     * measured how far our point is from the point at
     * coordinated(0.0, 0.0) and uses mathematical
     * operations that are available only for floating
     * point types.
     */

    // Traits: Defining shared behaviour
    /**
     * A `trait` tells the rust compiler about
     * functionality a particular type has and can share
     * with other types. We can use traits to define
     * shared behaviour in an abstract way. We can use
     * trait bounds to specify that a generic can be any
     * type that has a certain behaviour.
     *
     * Traits are similar to a feature called `interfaces`
     * in other languages, although with some differences.
     *
     * A type's behaviour consists of the methods we can
     * call on that type. Different types share the same
     * behaviour if we can call the same methods on all of
     * those types. Trait definitions are a way to group
     * method signatures together to define a set of
     * behaviours necessary to accomplish some purpose.
     *
     * We want to make a media aggregator library
     * that can display summaries of data that might
     * be stored in a NewsArticle or Tweet instance.
     * To do this, we need a summary from each type,
     * and we need to request that summary by calling
     * a summarize method on an instance.
     */

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    /**
     * After the method signature, instead of providing
     * an implementation with curly brackets, we use a
     * semicolon. Each type implementing this trait must
     * provide its own custom behaviour for the body of
     * the method. The compiler will enfore that any type
     * that has the `Summary` trait will have the method
     * `summarize` defined with this signature exactly.
     * A trait can have multiple methods in its body: the
     * method signatures are listed one per line and each
     * line ends in a semicolon.
     */

    // Implementing a Trait on a Type

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
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
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    /**
     * After implementing the trait, we can call the method
     * on instances of NewArticle and Tweet in the same way
     * we call regular methods:
     */
    let tweet = Tweet {
        username: String::from("John_Candy"),
        content: String::from("Have you ever had a dream you were a hotdog"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    /**
     * To implement a trait on another library's
     * scope, you will namespace:
     * use aggregator::Summary;
     */

    // Default Implementations
    /**
     * Sometimes it's useful to have default behaviour
     * for some or all of the methods in a trait instead
     * of requiring implementations for all methods on
     * every type. Then, as we implement the trai on a
     * particular type, we can keep or override each
     * method's default behaviour.
     */

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read More...)")
        }
    }

    /**
     * To use a default implementation to summarize
     * instances of `NewsArticle` instead of defining
     * a custom implementation, we specify an empty
     * impl block with `impl Summary for NewsArticle` {}
     * Even though we're no longer defining the
     * summarize method on `NewsArticle` directly, we're
     * providing default implementation.
     *
     * The syntax for overriding a default implementation
     * is the same as the syntax for implementing a
     * trait method that doesn't have a defualt
     * implementation. `Tweet.summarize()` will still
     * work as implemented.
     *
     * Default implementations can call other methods
     * in the same trait, even if those other methods
     * don't have a default implementation. In this way,
     * a trait can provide a lot of useful functionality
     * and only require implementors to specify a small
     * part of it. For example, we could define the
     * `Summary` trait to have a `summarize_author`
     * method whose implementation is required, and then
     * define a summarize method that has a default
     * implementation that calls the summarize_author
     * method:
     */

    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author)
        }
    }

    /**
     * To use this version of Summary, we only need
     * to define summarize_author when we implement
     * the trait on a type:
     */

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    // Traits as Parameters
}
