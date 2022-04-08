// use basics_rust;

#[test]
fn test_structs_fn() {
    let result: bool = basics_rust::structs_example();
    assert!(result);
}

#[test]
fn test_collections_fn() {
    let result: bool = basics_rust::collections_example();
    assert!(result);
}

#[test]
fn test_errors_fn() {
    let result: bool = basics_rust::errors_example();
    assert!(result);
}

#[test]
fn test_traits_fn() {
    let result: bool = basics_rust::traits_example();
    assert!(result);
}