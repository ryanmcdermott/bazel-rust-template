pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_positive() {
        assert_eq!(add(2, 3), 5);
    }
}