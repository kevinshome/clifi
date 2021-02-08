use std::{env, process};
use subprocess::{Popen, PopenConfig, Redirection};
use clap::{Arg, App};

fn main() -> std::io::Result<()> {

    let matches = App::new("clifi")
                          .version("221.1.0")
                          .about("play your favorite streams straight from the command line")
                          .arg(Arg::with_name("kill")
                               .short("k")
                               .long("kill")
                               .help("kill clifi instance")
                               .takes_value(false))
                          .get_matches();

    let mut vlc_path = "";
    let mut clifi_dir: String = "".to_string();

    if matches.is_present("kill"){
        process::Command::new("killall")
            .args(&["VLC"])
            .output()
            .expect("error while attempting to kill VLC instances. if error persists, try manually.");
        process::exit(0);
    }

    if env::consts::OS == "linux"{
        vlc_path = "/usr/bin/vlc";
    } else if env::consts::OS == "macos"{
        vlc_path = "/Applications/VLC.app/Contents/MacOS/VLC";
    } else if env::consts::OS == "windows" {
        vlc_path = "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe";
    }

    if cfg!(win32){ // NEEDS TO BE CHANGED
        clifi_dir = env::var("FOO").unwrap_or("none".to_string()); 
    } else if cfg!(unix){
        clifi_dir = env::var("HOME").unwrap_or("none".to_string()) + "/.clifi/";
    }

    println!("CLIFI_DIR = {}", clifi_dir);
    println!("VLC = {}", vlc_path);

    match Popen::create(&[vlc_path, "-I", "dummy", "-q", "--no-video", "https://www.youtube.com/watch?v=5qap5aO4i9A"], PopenConfig {
        stdout: Redirection::Pipe,
        stderr: Redirection::Pipe,
        detached: true,
        ..Default::default()
    }) {
        Ok(_) => (),
        Err(error) => panic!("error opening stream: {:?}", error),
    };



    Ok(())
}
