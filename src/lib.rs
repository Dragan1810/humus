pub fn say_may_name() -> &'static str {
    "Humus!!!"
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_numero_uno() {
        assert_eq!(2 + 2, 4);
    }
}
