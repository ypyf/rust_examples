// 打印命令行参数，不包括程序名
fn main() {
    for arg in std::env::args().skip(1) {
        println!("{}", arg);
    }
    // let x: i32 = -10;
    // println!("{:02x}, {a:}", x, a=x.abs());
    // let mut lists = Vec::new();
    // lists.push("aaaa");
    // println!("{}", lists[0]);

    let circle1 = Circle {
        x: 10.0,
        y: 6.0,
        radius: 5.0,
    };
    println!("x: {}, y: {}, radius: {}, area: {}",
             circle1.x,
             circle1.y,
             circle1.radius,
             circle1.area());
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

#[allow(dead_code)]
impl Circle {
    pub fn get_x(&self) -> f64 {
        self.x
    }
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.1415926 * self.radius * self.radius
    }
}