#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // "Method" AKA Member Method
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, compare_to: &Rectangle) -> bool {
        self.height > compare_to.height && self.width > compare_to.width
    }

    // "Associated Function" "::" AKA Static
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
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
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(10);

    println!("The area of our new square rectangle: {}", sq.area());
}
