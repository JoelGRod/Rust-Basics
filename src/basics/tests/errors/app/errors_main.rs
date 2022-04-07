use std::io;

use crate::basics::main::errors::app::errors_main;

#[test]
fn should_return_result_ok() -> Result<(), io::Error> {
    errors_main::read_username_from_file()?;
    Ok(())
}