#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    
    // Testing fn that returns a Result<T, E> (? is allowed)
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("two plus two does not equal four".to_owned())
        }
    }
}
