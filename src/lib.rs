pub use std::panic as augh;

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "explicit panic")]
    fn test_augh() {
        crate::augh!()
    }

    #[test]
    #[should_panic(expected = "sple")]
    fn test_augh_with_message() {
        crate::augh!("sple")
    }
}
