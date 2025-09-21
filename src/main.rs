struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0*(self.width + self.height)
    }

    fn is_square(&self) -> bool {
        if self.width == self.height{
            return true;
        } else{
            return false;
        }
    }
}

struct Circle{
    radius : f64
}

impl Circle{
    fn new(radius: f64) -> Circle{
        Circle{radius}
    }
    fn area(&self) -> f64{
        3.14*self.radius*self.radius
    }
    fn circumfrence(&self) -> f64{
        2.0*3.14*self.radius
    }
}

fn main() {
    let rect = Rectangle::new(10.0, 5.0);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
    println!("Is square? {}", rect.is_square());

    assert!(Rectangle::new(5.0, 5.0).is_square());
    assert!(!Rectangle::new(5.0, 6.0).is_square());

    let circ = Circle::new(4.0);
    println!("Area: {}", circ.area());
    println!("Circumfrence: {}", circ.circumfrence());
}
