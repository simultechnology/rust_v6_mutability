#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y:i32) -> Self {
        Point {
            x: x,
            y: y,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut x = 34;
    let y = x;
    x += 5;
    println!("y = {}, x = {}", y, x);

    let p = Person {
        name: "Taka".to_string(),
        age: 44,
    };

    let mut p2 = p.clone();
    p2.age = 45;
    p2.name.push_str(" ishikawa");
    println!("p = {:?}, p2 = {:?}", p, p2);

    let mut pnt = Point::new(3, 4);
    let mut pnt2 = pnt;
    pnt.x += 100;
    pnt2.y += 999;
    println!("pnt = {:?}, pnt2 = {:?}", pnt, pnt2);
}
