use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn init_log(filename: &str) -> Result<std::fs::File, std::io::Error>{
    match File::create(filename){
        Ok(f) => {return Ok(f);}
        Err(err) => {return Err(err);}
    }
}

pub fn print_log(file: &mut File, tag: &str, msg: &str){
    let timestamp = match SystemTime::now().duration_since(UNIX_EPOCH){
        Ok(n) => n.as_secs(),
        Err(_err)=> 0u64,
    };
    let msg_to_write = format!("[{timestamp}][{tag}] {msg}");
    match writeln!(file, "{}", msg_to_write){
        Ok(_) => {}
        Err(err) => { println!("Failed to write file {}", err); }
    }
}