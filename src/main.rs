use std::{fs, env, process, path::Path};
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

    if cfg!(win32){ // NEEDS TO BE CHANGED
        clifi_dir = env::var("FOO").unwrap_or("none".to_string()); 
    } else if cfg!(unix){
        clifi_dir = env::var("HOME").unwrap_or("none".to_string()) + "/.clifi";
    }

    if matches.is_present("kill"){
        match fs::remove_file(&format!("{}/clifi.lck", clifi_dir)) {
            Ok(_) => (),
            Err(error) => panic!("error while attempting to delete lockfile. if error persists, try manually. (~/.clifi/clifi.lck)\n{}", error),
        };
        match Popen::create(&["killall", "VLC"], PopenConfig::default()) {
            Ok(_) => process::exit(0),
            Err(error) => panic!("error while attempting to kill VLC instances. if error persists, try manually.\n{}", error),
        };
    }

    if env::consts::OS == "linux"{
        vlc_path = "/usr/bin/vlc";
    } else if env::consts::OS == "macos"{
        vlc_path = "/Applications/VLC.app/Contents/MacOS/VLC";
    } else if env::consts::OS == "windows" {
        vlc_path = "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe";
    }

    let json_raw_string = fs::read_to_string(format!("{}/streams.json", clifi_dir)).unwrap();
    let json_data = json::parse(&format!(r#"{}"#, json_raw_string)).unwrap();

    println!("JSON_DATA = {}", json_data["streams"][0]);
    println!("CLIFI_DIR = {}", clifi_dir);
    println!("VLC = {}", vlc_path);

    if Path::new(&format!("{}/clifi.lck", clifi_dir)).exists() {
        process::exit(1);
    }

    match Popen::create(&[vlc_path, "-I", "dummy", "-q", "--no-video", "https://www.youtube.com/watch?v=5qap5aO4i9A"], PopenConfig {
        stdout: Redirection::Pipe,
        stderr: Redirection::Pipe,
        detached: true,
        ..Default::default()
    }) {
        Ok(_) => fs::File::create(&format!("{}/clifi.lck", clifi_dir))?,
        Err(error) => panic!("error opening stream: {:?}", error),
    };



    Ok(())
}
