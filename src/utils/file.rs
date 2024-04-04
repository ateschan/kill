use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use crate::utils::user::User;
use crate::games::list::game_list;


//Bits and pieces used for debug purposes
pub fn check_for_name(name: &str) -> bool{
    return true;
}

pub fn return_dummy_id() -> i32{
    return 0;
}

pub fn return_dummy_user() -> User{
    User{
        name : "Addison".to_string(),
        id : 34,
        games : game_list()
    }
}



//USED UTILITIES DO NOT DELETE
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



//return true for now...
//
//

//function that writes a user struct to a toml
//


//function that checks for data in the toml
//
//
