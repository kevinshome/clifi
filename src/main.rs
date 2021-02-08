use std::{fs, env, process, path::Path};
use subprocess::{Popen, PopenConfig, Redirection};
use clap::{Arg, App};

fn main() -> std::io::Result<()> {

    // CLAP DEFINITIONS
    let matches = App::new("clifi")
                          .version("221.1.0")
                          .about("play your favorite streams straight from the command line")
                          .arg(Arg::with_name("stream")
                               .help("stream to play")
                               .required(true)
                               .default_value("lofi"))
                          .arg(Arg::with_name("kill")
                               .short("k")
                               .long("kill")
                               .help("kill clifi instance")
                               .takes_value(false))
                          .arg(Arg::with_name("verbose")
                               .short("v")
                               .long("verbose")
                               .help("be verbose")
                               .takes_value(false))
                          .get_matches();


    // DEFINE VARIABLES FOR LATER
    let stream_name = matches.value_of("stream").unwrap();                    
    let mut vlc_path = "";
    let mut stream_url: String = "".to_string();
    let mut clifi_dir: String = "".to_string();

    // DEFINE CLIFI DIRECTORY
    if cfg!(win32){ // NEEDS TO BE CHANGED
        clifi_dir = env::var("FOO").unwrap(); 
    } else if cfg!(unix){
        clifi_dir = env::var("HOME").unwrap() + "/.clifi";
    }

    // IF VLC KILL IS REQUESTED WITH "-k" ARG
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

    // DEFINE VLC PATH ON SYSTEM
    if env::consts::OS == "linux"{
        vlc_path = "/usr/bin/vlc";
    } else if env::consts::OS == "macos"{
        vlc_path = "/Applications/VLC.app/Contents/MacOS/VLC";
    } else if env::consts::OS == "windows" {
        vlc_path = "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe";
    }

    // READ STREAMFILE
    let json_raw_string = fs::read_to_string(format!("{}/streams.json", clifi_dir)).unwrap();
    let json_data = json::parse(&format!(r#"{}"#, json_raw_string)).unwrap();

    // GET STREAM URL FROM STREAMFILE
    for i in 0..json_data["streams"].len(){
        if json_data["streams"][i]["name"] == stream_name{
            stream_url = json_data["streams"][i]["url"].to_string();
        }
    };

    // VERBOSE PRINTS
    if matches.is_present("verbose"){
        println!("RUNNING_STREAM = {}", stream_name);
        println!("STREAM_URL = {}", stream_url);
        println!("JSON_DATA = {}", json_data["streams"]);
        println!("CLIFI_DIR = {}", clifi_dir);
        println!("VLC = {}", vlc_path);
    }

    // CHECK FOR LOCKFILE. IF EXISTS, EXIT
    if Path::new(&format!("{}/clifi.lck", clifi_dir)).exists() {
        process::exit(1);
    }

    // RUN STREAM IN HEADLESS VLC
    match Popen::create(&[vlc_path, "-I", "dummy", "-q", "--no-video", &stream_url], PopenConfig {
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
