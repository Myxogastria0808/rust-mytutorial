#[derive(Debug)]

//***impl***//
struct Triangle {
    height: f64,
    bottom: f64,
}

//imple 構造体名　{}
impl Triangle {
    fn equ_triangle (bottom: f64) -> Self {
        Self {
            height: (bottom / 2.0) * 3.0f64.sqrt(),
            bottom: bottom,
        }
    }

    //***不変な参照***//
    fn area(&self) -> f64 {
        self.height * self.bottom / 2.0
    }

    //***可変な参照***//
    //setterの作成
    fn set_bottom(&mut self, bottom: f64) {
        self.bottom = bottom;
    }
}


fn main() {
    let mut triangle1: Triangle = Triangle {
        height: 10.0,
        bottom: 10.0,
    };

    let equ_triangle1: Triangle = Triangle::equ_triangle(6.0);

    println!("{:?}", triangle1);
    println!("{:?}", equ_triangle1);
    println!("triangle1: {}", triangle1.area());
    println!("equ_triangle1: {}", equ_triangle1.area());

    triangle1.set_bottom(40.0);
    println!("{:?}", triangle1);
}
