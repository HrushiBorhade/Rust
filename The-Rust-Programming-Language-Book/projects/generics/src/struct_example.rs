pub struct Point<T,U> {
   pub x: T,
   pub y: U,
}

impl<T,U> Point<T,U> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

