fn main() {
    // // Generic types, traits, and lifetimes

    // /*

    // Generics are abstract stand-ins for concrete types or other
    // properties. When writing code, we can express the behaviour
    // of generics or how they relate to other generics without
    // knowing what will be in thier place when compiling and
    // running the code.

    // e.g.
    // Result<T, E>

    // Let’s first look at how to remove duplication that
    // doesn’t involve generic types by extracting a function.

    // */
    // let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {}", largest);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {}", largest);

    // // To eliminate duplication,
    // // we can extract to a function

    // fn largest(list: &[i32]) -> i32 {
    //     let mut largest = list[0];

    //     for &number in list {
    //         if number > largest {
    //             largest = number;
    //         }
    //     }
    //     largest
    // }
    // /**
    //  * We use the same steps with generics to reduce code
    //  * duplication in different ways. For example, if we
    //  * had 2 functions that finds the largest item in a
    //  * slice of `i32` values, and the largest in a slice of
    //  * `char` values.
    //  *
    //  * When defining a function that uses generics, we
    //  * place the generics in the signature of the function
    //  * where we would usually specify the data types of the
    //  * parameters and return value. Doing so makes our code
    //  * more flexible and provides more functionality to
    //  * callers of our function while preventing code
    //  * duplication.
    //  */
    // // the two version of this function for different
    // // types:

    // fn largest_i32(list: &[i32]) -> i32 {
    //     let mut largest = list[0];

    //     for &item in list.iter() {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }

    //     largest
    // }

    // fn largest_char(list: &[char]) -> char {
    //     let mut largest = list[0];

    //     for &item in list.iter() {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }

    //     largest
    // }

    // /**
    // * To parameterize the types in the new function
    // * we’ll define, we need to name the type parameter,
    // * just as we do for the value parameters to a
    // * function. You can use any identifier as a type
    // * parameter name. But we’ll use T because, by
    // * convention, parameter names in Rust are short,
    // * often just a letter, and Rust’s type-naming
    // * convention is CamelCase. Short for “type,” T
    // * is the default choice of most Rust programmers.

    //   When we use a parameter in the body of the
    //   function, we have to declare the parameter
    //   name in the signature so the compiler knows
    //   what that name means.

    //   To define the generic largest function, place
    //   type name declarations inside angle brackets,
    //   <>, between the name of the function and the
    //   parameter list, like this:
    // */
    // // fn largest<T>(list: &[T]) -> T {
    // //     let mut largest = list[0];

    // //     for &item in list.iter() {
    // //         if item > largest {
    // //             largest = item;
    // //         }
    // //     }
    // //     largest
    // // }

    // /**
    //  * The above will not compile however because
    //  * std::cmp::PartialOrd is not implemented on generic
    //  * <T> traits.
    //  */
    // // In Struct Definitions

    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }

    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };

    // // You can also define 2 a struct with 2 different
    // // generic types:
    // struct Point<T, U> {
    //     x: T,
    //     y: U,
    // }

    // fn main() {
    //     let both_integer = Point { x: 5, y: 10 };
    //     let both_float = Point { x: 1.0, y: 4.0 };
    //     let integer_and_float = Point { x: 5, y: 4.0 };
    // }

    // // In method definitions
    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }

    // impl<T> Point<T> {
    //     fn x(&self) -> &T {
    //         &self.x
    //     }
    // }

    // let p = Point { x: 5, y: 10 };
    // println!("p.x = {}", p.x());

    // /**
    //  * Note we have to decalre T just after `impl` so
    //  * we can use it to specify that we're implementing
    //  * methods on the type Point<T>. By declaring T as a
    //  * generic type after `impl`, Rust can identify that
    //  * the type in the angle brackets in Point is a
    //  * generic type rather than a concrete type.
    //  *
    //  * We could, for example, implement methods only on
    //  * Point<f32> instances rather than on Point<T> instances
    //  * with any generic type.
    //  */
    // impl Point<f32> {
    //     fn distance_from_origin(&self) -> f32 {
    //         (self.x.powi(2) + self.y.powi(2)).sqrt()
    //     }
    // }

    // /**
    //  * This code means the type Point<f32> will have a
    //  * method named `distance_from_origin` and other
    //  * instances of Point<T> where T is not of type f32
    //  * will not have this method defined. The method
    //  * measured how far our point is from the point at
    //  * coordinated(0.0, 0.0) and uses mathematical
    //  * operations that are available only for floating
    //  * point types.
    //  */
    // // Traits: Defining shared behaviour
    // /**
    //  * A `trait` tells the rust compiler about
    //  * functionality a particular type has and can share
    //  * with other types. We can use traits to define
    //  * shared behaviour in an abstract way. We can use
    //  * trait bounds to specify that a generic can be any
    //  * type that has a certain behaviour.
    //  *
    //  * Traits are similar to a feature called `interfaces`
    //  * in other languages, although with some differences.
    //  *
    //  * A type's behaviour consists of the methods we can
    //  * call on that type. Different types share the same
    //  * behaviour if we can call the same methods on all of
    //  * those types. Trait definitions are a way to group
    //  * method signatures together to define a set of
    //  * behaviours necessary to accomplish some purpose.
    //  *
    //  * We want to make a media aggregator library
    //  * that can display summaries of data that might
    //  * be stored in a NewsArticle or Tweet instance.
    //  * To do this, we need a summary from each type,
    //  * and we need to request that summary by calling
    //  * a summarize method on an instance.
    //  */
    // pub trait Summary {
    //     fn summarize(&self) -> String;
    // }

    // /**
    //  * After the method signature, instead of providing
    //  * an implementation with curly brackets, we use a
    //  * semicolon. Each type implementing this trait must
    //  * provide its own custom behaviour for the body of
    //  * the method. The compiler will enfore that any type
    //  * that has the `Summary` trait will have the method
    //  * `summarize` defined with this signature exactly.
    //  * A trait can have multiple methods in its body: the
    //  * method signatures are listed one per line and each
    //  * line ends in a semicolon.
    //  */
    // // Implementing a Trait on a Type

    // pub struct NewsArticle {
    //     pub headline: String,
    //     pub location: String,
    //     pub author: String,
    //     pub content: String,
    // }

    // impl Summary for NewsArticle {
    //     fn summarize(&self) -> String {
    //         format!("{}, by {} ({})", self.headline, self.author, self.location)
    //     }
    // }

    // pub struct Tweet {
    //     pub username: String,
    //     pub content: String,
    //     pub reply: bool,
    //     pub retweet: bool,
    // }

    // impl Summary for Tweet {
    //     fn summarize(&self) -> String {
    //         format!("{}: {}", self.username, self.content)
    //     }
    // }

    // /**
    //  * After implementing the trait, we can call the method
    //  * on instances of NewArticle and Tweet in the same way
    //  * we call regular methods:
    //  */
    // let tweet = Tweet {
    //     username: String::from("John_Candy"),
    //     content: String::from("Have you ever had a dream you were a hotdog"),
    //     reply: false,
    //     retweet: false,
    // };

    // println!("1 new tweet: {}", tweet.summarize());

    // /**
    //  * To implement a trait on another library's
    //  * scope, you will namespace:
    //  * use aggregator::Summary;
    //  */
    // // Default Implementations
    // /**
    //  * Sometimes it's useful to have default behaviour
    //  * for some or all of the methods in a trait instead
    //  * of requiring implementations for all methods on
    //  * every type. Then, as we implement the trai on a
    //  * particular type, we can keep or override each
    //  * method's default behaviour.
    //  */
    // pub trait Summary {
    //     fn summarize(&self) -> String {
    //         String::from("(Read More...)")
    //     }
    // }

    // /**
    //  * To use a default implementation to summarize
    //  * instances of `NewsArticle` instead of defining
    //  * a custom implementation, we specify an empty
    //  * impl block with `impl Summary for NewsArticle` {}
    //  * Even though we're no longer defining the
    //  * summarize method on `NewsArticle` directly, we're
    //  * providing default implementation.
    //  *
    //  * The syntax for overriding a default implementation
    //  * is the same as the syntax for implementing a
    //  * trait method that doesn't have a defualt
    //  * implementation. `Tweet.summarize()` will still
    //  * work as implemented.
    //  *
    //  * Default implementations can call other methods
    //  * in the same trait, even if those other methods
    //  * don't have a default implementation. In this way,
    //  * a trait can provide a lot of useful functionality
    //  * and only require implementors to specify a small
    //  * part of it. For example, we could define the
    //  * `Summary` trait to have a `summarize_author`
    //  * method whose implementation is required, and then
    //  * define a summarize method that has a default
    //  * implementation that calls the summarize_author
    //  * method:
    //  */
    // pub trait Summary {
    //     fn summarize_author(&self) -> String;

    //     fn summarize(&self) -> String {
    //         format!("(Read more from {}...)", self.summarize_author)
    //     }
    // }

    // /**
    //  * To use this version of Summary, we only need
    //  * to define summarize_author when we implement
    //  * the trait on a type:
    //  */
    // impl Summary for Tweet {
    //     fn summarize_author(&self) -> String {
    //         format!("@{}", self.username)
    //     }
    // }

    // // Traits as Parameters
    // /**
    //  * We can also explore how to use traits to define
    //  * function that accepts many different types. We
    //  * can define a `notify` function that calls the
    //  * `summarize` method on its `item` parameter, which
    //  * is of some time that implements the `summary` trait.
    //  */
    // pub fn notify(item: impl Summary) {
    //     println!("Breaking News! {}", item.summarize());
    // }

    // /**
    //  * Instead of a concrete type for the `item` parameter,
    //  * we specify the `impl` keyword and the trait name. This
    //  * parameter accepts any type that implements that specific
    //  * trait.
    //  */
    // // Specifying Multiple Trait Bounds with the + Syntax

    // fn notify(item: impl Summary + Display) {
    //     // code block
    // }

    // // Clearer Trait Bounds with `where` Clauses
    // /**
    //  * Using too many trait bounds has its downsides.
    //  * Each generic has its own trait bounds, so functions
    //  * with multiple generic type parameters can contain
    //  * lots of trait bound information between the functions
    //  * name and its parameter list, making the signature hard
    //  * to read. Rust has an alternate syntax for specifying
    //  * trait bounds inside a `where` clause after the function
    //  * signature:
    //  */
    // fn some_function<T, U>(t: T, u: U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // {
    //     // code block
    // }

    // // Returning Types that Implement Traits
    // fn return_summarizable() -> impl Summary {
    //     Tweet {
    //         username: String::from("GuitarMan"),
    //         content: String::from("Tele is best"),
    //         reply: false,
    //         retweet: false,
    //     }
    // }

    // // Fixing the largest Function with Trait Bounds
    // fn largest<T: PartialOrd>(list: &[T]) -> &T {
    //     let mut largest = &list[0];

    //     for item in list.iter() {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }
    //     &largest
    // }

    // // println!("{}", largest(&vec![1, 4, 6, 8, 0]));

    // // let v = vec![1, 2, 3, 4];
    // fn reffer(v: Vec<i32>) {
    //     for &item in &v {
    //         println!("{}", item);
    //     }
    // }
    // // Using trait bounds to conditionally implement methods
    // /**
    //  * By using a trait bound with an `impl` block that
    //  * uses generic type parameters, we can implement
    //  * methods conditionally for types that implement the
    //  * specific traits. For example, Pain<T> always
    //  * implements the `new` method, but only implements
    //  * the `cmp_display` method if it's inner type T
    //  * implements the `PartialOrd` traut that enables
    //  * comparison and the display trait that enables
    //  * printing:
    //  */
    // use std::fmt::Display;

    // struct Pair<T> {
    //     x: T,
    //     y: T,
    // }

    // impl<T> Pait<T> {
    //     fn new(x: T, y: T) -> Self {
    //         Self { x, y }
    //     }
    // }

    // impl<T: Display + PartialOrd> Pair<T> {
    //     fn cmp_display(&self) {
    //         if self.x >= self.y {
    //             println!("Largest is x = {}", self.x);
    //         } else {
    //             println!("Largest is y = {}", self.y);
    //         }
    //     }
    // }

    // /**
    //  * We can also conditionally implement a trait for any
    //  * type that implements another trait. Implementations
    //  * of a trait on any type that satisfies the trait
    //  * bounds are called `blanket implementations` and are
    //  * extensively used in the Rust standard library. For
    //  * example, the standard library implements the
    //  * `ToString` trait on any type that implements the
    //  * `Display` trait. The `impl` block in the standard
    //  * library looks similar to this:
    //  */
    // impl<T: Display> ToString for T {
    //     // code block
    // }

    // /**
    //  * Because the standard library has this blanket
    //  * implementation, we can call the `to_string` method
    //  * defined by the `ToString` trait on any type that
    //  * implements the `Display` trait. For example, we
    //  * can turn integers into their corresponding `String`
    //  * values like this because integers implement
    //  * `Display`:
    //  */
    // let s = 3.to_string();

    // // Validating references with Lifetimes

    // /**
    //  * Lifetime annotations don't change how long any of
    //  * the references live, and just as functions can accept
    //  * any type with the generic type parameter, functions
    //  * can accept references with any lifetime by specifying
    //  * a generic lifetime parameter. Lifetime annotationa
    //  * describe the relationships of the lifetimes of
    //  * multiple references to eachother with affecting the
    //  * lifetimes.
    //  *
    //  * One lifetime annotation by itself doesn't have
    //  * much meaning because the annotations are meant
    //  * to tell Rust how generic lifetime parameters of
    //  * multiple references relate to eachother.
    //  */
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    /**
     * This function signature tells Rust that for some
     * lifetime `a, the function takes two parameters,
     * both of which are string slices that live at least
     * as long as lifetime `a. The function signature also
     * tells Rust that the string slice returned from the
     * function will live at least as long as lifetime `a.
     * In practice, it means that the lifetime of the
     * reference returned by the `longest` function is the
     * same as the smaller of the lifetimes of the
     * references passed in.
     *
     * What we are saying is that the borrow checker should
     * reject any values that don't adhere to these
     * constraints.
     */
    // This will compile:
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // This will not:
    let string1 = String::from("long string is long");
    let result;
    {
        // Moving `let string2(...)` to the outer scope
        // will allow it to compile:
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    // Lifetime Annotations in Struct Definitions
    /**
     * It is possible for structs to hold references,
     * but tin that case we would need to add a lifetime
     * annotation on every reference in the structs
     * definition.
     */

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("blah blah blah...");
    let first_sentnece = novel.split(".")
        .next()
        .expect("Could not fine a '.'");
    let i = ImportantExcerpt { part: first_sentnece }

    /**
     * This lifetime annotation means and instance of
     * `ImportantExcerpt` can't outlive the reference
     * it holds in its `part` field.
     */
}
