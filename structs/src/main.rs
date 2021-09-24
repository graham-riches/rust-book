struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}


fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let user = build_user(String::from("test@123.com"), String::from("John Doe"));
    println!("{}", user.email);

    let user2 = User {
        email: String::from("fake@123.com"),
        username: String::from("Jane Doe"),
        ..user
    };
    println!("{}", user2.email);

    let rect1 = Rectangle {
        width: 15,
        height: 30
    };
    println!("Rectangle {:?}", rect1);
    let area = rect1.area();
    println!("Rectangle {}", area);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


