use std::env;
use std::process::Command;
use nix::unistd::{sethostname};

type Result<T> = std::result::Result<T, Box<std::error::Error>>;


fn run(string: &str, string1: &str) {
    println!("{}", string);
    Command::new("/proc/self/exe")
        .arg("child")
        .arg(string1)
        .spawn()
        .expect("blah");
    
    // let mut cf = CloneFlags::empty();
    //sethostname("gargi")?;
}

fn child(string: &str) -> Result<()> {
    println!("{}", string);
    sethostname("gawwrgi")?; 
    Command::new("ls")
        .env("PATH", string)
        .spawn()
        .expect("ls command failed to start");
    Ok(())
}

fn main() -> Result<()>{
    let argv: Vec<_> = env::args().collect();
    println!("{}", argv[1]);
    if argv[1] == "run" {
        run(&argv[2], &argv[3]);
    } else if argv[1] == "child" {
        println!("HOWOOWOWO");
        child(&argv[2]);
    } else {
        panic!("No run specified")
    }
    Ok(())
}
