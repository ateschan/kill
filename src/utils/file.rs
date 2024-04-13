use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use crate::utils::user::User;
use crate::games::list::game_list;
use std::fs::create_dir;
use std::path::Path;
use glob::glob;

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
    let path = "killfs/";
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(path)?;
    }
    else {
        for entry in glob("killfs/*.txt").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => std::fs::remove_file(path).unwrap(),
            Err(e) => println!("{:?}", e),
            }
        }
    }

    for (fname, fcontent) in fs {
        let mut file = File::create(path.to_owned() + &fname)?;
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
