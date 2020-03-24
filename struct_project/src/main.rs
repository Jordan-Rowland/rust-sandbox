// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     )
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

///////////////////
// Makes more sense with a tuple:

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     )
// }

// fn area(dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }

///////////////////
// Using derive debug

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );

//     println!("{:#?}", rect1)
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

///////////////////
// area as method

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width > rect2.width && self.height > rect2.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 100,
        height: 400,
    };

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// struct Dog {
//     name: String,
//     age: u32,
//     breed: String,
// }

// impl Dog {
//     fn bark(&self) -> String {
//         format!(
//             "My name is {name} and I'm a {age} year old {breed}!",
//             name = self.name,
//             age = self.age,
//             breed = self.breed,
//         )
//     }

//     fn birfday(&mut self) -> String {
//         self.age += 1;
//         format!("I just had a birfday! Now I'm {}!", self.age)
//        }
// fn new(name: String, age: u32, breed: String) -> Dog {
//     Dog { name, age, breed }
// }
// }

// fn new_dog(name: String, age: u32, breed: String) -> Dog {
//     Dog { name, age, breed }
// }

// fn main() {
//     let mut rex = new_dog("Rex".to_string(), 4, "Pup!".to_string());

//     println!("{}", rex.bark());
//     println!("{}", rex.birfday());
//     println!("{}", rex.birfday());
//     println!("{}", rex.birfday());
// }
