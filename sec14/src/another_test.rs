// #2
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let r1 = Rectangle {
            width: 20,
            height: 10,
        };
        let r2 = Rectangle {
            width: 10,
            height: 5,
        };
        assert!(r1.can_hold(&r2))
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let r1 = Rectangle {
            width: 20,
            height: 10,
        };
        let r2 = Rectangle {
            width: 10,
            height: 5,
        };
        assert!(!r2.can_hold(&r1))
    }
}
