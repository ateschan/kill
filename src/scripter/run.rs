//TODO: Calculate execution time and append to out
//
use std::process::Command;

pub fn execute(input : String) -> String{

    let head : String = "#!/bin/bash\n".to_string();
    let script = head + &input; 
    let output = Command::new("bash")
        .arg("-c")
        .arg(script)
        .output()
        .expect("Failed to execute command");

    
    String::from_utf8_lossy(&output.stdout).to_string()
}
