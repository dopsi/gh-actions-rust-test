pub fn bar(a: i32) -> i32 {
    a + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bar() {
        let a = 1;
        assert_eq!(a + 1, bar(a));
    }
}
