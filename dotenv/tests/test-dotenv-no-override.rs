mod common;

use dotenvy::*;
use std::{env, error::Error, result::Result};

use crate::common::*;

#[test]
fn test_dotenv_no_override() -> Result<(), Box<dyn Error>> {
    let dir = make_test_dotenv()?;

    let iter = dotenv_iter()?;

    env::set_var("TESTKEY", "initial_val");

    iter.load()?;
    assert_eq!(env::var("TESTKEY")?, "initial_val");

    env::set_current_dir(dir.path().parent().unwrap())?;
    dir.close()?;
    Ok(())
}
