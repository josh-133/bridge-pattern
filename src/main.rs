trait Color {
    fn fill(&self) -> String;
}

struct Red;
struct Blue;

impl Color for Red {
    fn fill(&self) -> String {
        "red".to_string()
    }
}

impl Color for Blue {
    fn fill(&self) -> String {
        "blue".to_string()
    }
}

trait Shape {
    fn draw(&self) -> String;
}

struct Circle<'a> {
    color: &'a dyn Color,
}

impl<'a> Shape for Circle<'a> {
    fn draw(&self) -> String {
        format!("Drawing a {} circle", self.color.fill())
    }
}

struct Square<'a> {
    color: &'a dyn Color,
}

impl<'a> Shape for Square<'a> {
    fn draw(&self) -> String {
        format!("Drawing a {} square", self.color.fill())
    }
}

fn main() {
    let red = Red;
    let blue = Blue;

    let red_circle = Circle { color: &red };
    let blue_circle = Circle { color: &blue };
    let red_square = Square { color: &red };
    let blue_square = Square { color: &blue };

    println!("{}", red_circle.draw());
    println!("{}", blue_circle.draw());

    println!("{}", red_square.draw());
    println!("{}", blue_square.draw());

}
