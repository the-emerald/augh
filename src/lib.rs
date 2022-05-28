#[macro_export]
macro_rules! augh {
    () => {
        panic!("augh!")
    };
    ($($arg:tt)+) => {
        panic!("augh: {}", core::format_args!($($arg)+))
    };
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "augh")]
    fn test_augh() {
        augh!()
    }

    #[test]
    #[should_panic(expected = "augh: sple")]
    fn test_augh_with_message() {
        augh!("sple")
    }
}
