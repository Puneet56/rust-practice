#[derive(Debug)] //this helps us to print the Rectangle object using {:?} in println!
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn increase_height(&mut self, increase_in_length: u32) {
        self.height = self.height + increase_in_length
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 10,
        height: 10,
    };

    rect.increase_height(5);

    let square = Rectangle::square(5);

    println!("Rectangle is {:?} and its area is {}", rect, rect.area());

    println!("Square is {:?} and its area is {}", square, square.area());

    println!(
        "Rectangle {} hold square",
        if rect.can_hold(square) {
            "can"
        } else {
            "cannot"
        }
    );
}
