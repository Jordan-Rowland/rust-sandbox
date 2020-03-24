struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: "jorban".to_string(),
        email: String::from("jordan@email.com"),
        active: true,
        sign_in_count: 1,
    };

    let mut user1 = User {
        username: "jorban".to_string(),
        email: String::from("jordan@email.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("jorban@gmail.com");

    let user2 = User {
        username: "jorban".to_string(),
        email: String::from("jordan@email.com"),
        // Similar unpacking as JS
        ..user1
    };

    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        // You can use `JS style` destructuring to
        // assign a value where the k: v pair
        // have the same name:
        email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// let user1 = User {
//     username: "jorban".to_string(),
//     email: String::from("jordan@email.com"),
//     active: true,
//     sign_in_count: 1,
// };

// let user2 = User {
//     username: "jorban".to_string(),
//     email: String::from("jordan@email.com"),
//     // Similar unpacking as JS
//     ..user1
// };

// // Tuple Structs with Named Fields
// scruct Color(i32, i32, i32);

// let black = Color(0, 0, 0)

struct User {
    // This requires `Lifetimes`
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}
