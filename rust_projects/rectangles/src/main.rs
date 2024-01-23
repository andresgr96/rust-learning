struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }
}

fn main() {

    // Params only version
    let widht1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} sq pixels.",
        area(widht1, height1)
    );

    // Tuple version
    let rect1 = (30, 50);
    print_area(rect1);


    // Struct version
    let rect2 = Rectangle {
        width:30,
        height:50,
    };

    println!(
        "The area of the rectangle is {} sq pixels.",
        area_struct(&rect2)
    );

    // Method version
    println!(
        "The area of the rectangle is {} sq pixels.",
        rect2.area()
    );

    let rect3 = Rectangle {
        width:20,
        height:40,
    };

    println!("Can rec2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rec3 hold rect2? {}", rect3.can_hold(&rect2));
}

fn area(widht: u32, height: u32) -> u32 {
    widht * height
}

fn area_tuple(dims: (u32, u32)) -> u32 {
    dims.0 * dims.1
}

fn print_area(rect: (u32, u32)) {
    println!(
        "The area of the rectangle is {} sq pixels.",
        area_tuple(rect)
    );
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}