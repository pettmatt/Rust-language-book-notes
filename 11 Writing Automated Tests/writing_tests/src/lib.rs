#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn can_hold_liquid(&self, other: &Rectangle) -> bool {
        (self.width * self.height) > (other.width * other.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn can_hold_the_liquid() {
        let glass1 = Rectangle {
            width: 10,
            height: 3
        };

        let glass2 = Rectangle {
            width: 3,
            height: 5
        };

        assert!(glass1.can_hold_liquid(&glass2));
    }

    // #[test]
    // fn exploration() {
    //     assert_eq!(2 + 2, 4);
    // }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
}