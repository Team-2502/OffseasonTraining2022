#[cfg(test)]
mod tests {
    use crate::add::add;

    #[test]
    fn add_one_plus_one() {
        assert_eq!(
            add(1, 1),
            2
        )
    }

    #[test]
    fn add_over_u16_max() {
        assert_eq!(
            add(65535, 1),
            0
        )
    }
}