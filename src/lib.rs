#[macro_export]
macro_rules! augh {
    () => {
        panic!()
    };
    ($($arg:tt)+) => {
        panic!($($arg)+)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "explicit panic")]
    fn test_augh() {
        augh!()
    }

    #[test]
    #[should_panic(expected = "sple")]
    fn test_augh_with_message() {
        augh!("sple")
    }
}
