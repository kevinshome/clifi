use std::{env, process};
use subprocess::{Popen, PopenConfig};
use clap::{Arg, App};

fn main() -> std::io::Result<()> {

    let matches = App::new("clifi")
                          .version("221.1.0")
                          .author("kevinshome <noah.tanner7@gmail.com>")
                          .about("vlc condom")
                          .arg(Arg::with_name("kill")
                               .short("k")
                               .long("kill")
                               .help("kill clifi instance")
                               .takes_value(false))
                          .get_matches();

    let mut vlc_path = "";
    let mut clifi_dir: String = "".to_string();

    if matches.is_present("kill"){
        Popen::create(&["killall", "VLC"], PopenConfig::default());
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

    match Popen::create(&[vlc_path, "-I", "dummy", "-q", "--no-video", "https://www.youtube.com/watch?v=5qap5aO4i9A", "&"], PopenConfig::default()) {
        Ok(mut popen) => Popen::detach(&mut popen),
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };


    Ok(())
}
