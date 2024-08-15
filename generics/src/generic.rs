pub struct Point1<T> {
    x: T,
    y: T,
}

impl<T> Point1<T> {
    pub fn get_point(&self) -> (&T, &T) {
        let (x, y) = (&self.x, &self.y);
        (x, y)
    }
    pub fn set_point(x: T, y: T) -> Self {
        Point1 {
            x,
            y,
        }
    }
}

pub struct Point2<T, U> {
    x: T,
    y: U,
}

impl<U> Point2<f64, U> {
    pub fn get_point_x_f64(&self) -> (&f64, &U) {
        let (x, y) = (&self.x, &self.y);
        (x, y)
    }
    pub fn set_point_x_f64(x: f64, y: U) -> Self {
        Point2 {
            x,
            y,
        }
    }
}

pub enum User<T> {
    Username(T),
    Nickname(T),
}
