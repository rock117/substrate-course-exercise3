static PI:f64 = 3.1415926;
fn area<T: Shape>(shape:&T)-> f64 {
    shape.area()
}
trait Shape{
    fn area(&self) -> f64;
}
struct Circle(f64);
struct Rectangle(f64, f64);
struct Triangle(f64, f64, f64);

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.0 * self.0 * PI
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.0 * self.1
    }
}
impl Shape for Triangle{
    fn area(&self) -> f64 {
        let p:f64 = (self.0 + self.1 + self.2) / 2;
        let result = p * (p - self.0) * (p - self.1) * (p - self.2);
        result.sqrt()
    }
}



