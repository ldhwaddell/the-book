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

//     println!("rect1 is {:#?}", rect1);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );

//     test_dbg();
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// // Trying the dbg! macro
// fn test_dbg() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

// Implementing a method:
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
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        Self {
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

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let sq = Rectangle::square(7);
    println!("New square: {:#?}", sq)
}
