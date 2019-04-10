pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_one() {
        assert_eq!( add_one(2), 3);
    }
}
