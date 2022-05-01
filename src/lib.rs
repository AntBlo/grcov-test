fn returns_one() -> i32 {
    1
}

fn returns_two() -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use crate::{returns_one, returns_two};
    #[test]
    fn test_one() {
        assert!(returns_one() == 2);
    }

    #[test]
    fn test_two() {
        assert!(returns_two() == 2);
    }
}
