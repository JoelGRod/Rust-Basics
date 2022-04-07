## Test Info
# Macros
* assert!()
* assert_eq!()
* assert_ne!()

# Attributes
* #[cfg(test)] - Defines a test module
* #[test] - Put this before every test function
* #[should_panic(expected="")] - This attribute makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesn’t panic.