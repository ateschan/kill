use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use crate::utils::user::User;
use crate::games::meta::Game;
use std::vec::Vec;


//return true for now...
//
//

//function that writes a user struct to a toml
//


//function that checks for data in the toml
//
//


pub fn check_for_name(name: &str) -> bool{
    return true;
}

pub fn return_dummy_id() -> i32{
    return 0;
}

pub fn return_dummy_user() -> User{
    User{
        name : "Addison".to_string(),
        id : 34
    }
}

pub fn fs_create(fs : HashMap<String,String>) -> std::io::Result<()> {
    for (fname, fcontent) in fs {
        let mut file = File::create(fname)?;
        file.write(fcontent.as_bytes())?;
    }
    Ok(())
}

pub fn fs_delete(fs : HashMap<String,String>) -> std::io::Result<()>{
    for (fname, _fcontent) in fs {
        std::fs::remove_file(fname)?;
    }
    Ok(())
}
