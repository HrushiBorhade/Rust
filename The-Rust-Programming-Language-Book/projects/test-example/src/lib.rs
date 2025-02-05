pub fn adder(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}

impl Rectangle {
    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = adder(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn cause_panic() {
        panic!("aaaaaaahhhhhhhhhhh!!")
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 30,
            height: 50,
        };

        let smaller = Rectangle {
            width: 10,
            height: 30,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 30,
            height: 50,
        };

        let smaller = Rectangle {
            width: 10,
            height: 30,
        };

        assert!(smaller.can_hold(&larger));
    }
}
