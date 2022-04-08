## Test Info
# Macros
* assert!()
* assert_eq!()
* assert_ne!()

# Attributes
* #[cfg(test)] - Defines a test module (unit tests).
* #[test] - Put this before every test function.
* #[should_panic(expected="")] - This attribute makes a test pass if the code inside the function panics; the test will fail if the code inside the function doesn’t panic.
* #[ignore] - Ignoring Some Tests Unless Specifically Requested.

# Returns
* Result<T, E> -> Useful for fn that returns a Result<T, E>

# Commands
* cargo test --help                 - displays the options you can use with cargo test.
* cargo test -- --help              - displays the options you can use after the separator --.
* cargo test -- --test-threads=1    - If you don’t want to run the tests in parallel or if you want more fine-grained control over the number of threads used.
* cargo test -- --show-output       - If we want to see printed values for passing tests as well.
* cargo test test_name              - Running a single test by name.
* cargo test part_of_the_test_name  - Filtering to run multiple tests. We can specify part of a test name, and any test whose name matches that value will be run.
* cargo test -- --ignored           - If we want to run only the ignored tests.
* cargo test -- --include-ignored   - If you want to run all tests whether they’re ignored or not.
