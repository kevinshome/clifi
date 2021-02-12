use clap::{App, Arg};
use std::{env, fs, path::Path, process};
use subprocess::{Popen, PopenConfig, Redirection};
use toml::Value;

fn main() -> std::io::Result<()> {
    // DEFINE VARIABLES FOR LATER
    let mut vlc_path = "";
    let mut clifi_dir: String = "".to_string();

    // DEFINE CLIFI DIRECTORY
    if cfg!(win32) {
        // NEEDS TO BE CHANGED
        clifi_dir = env::var("FOO").unwrap();
    } else if cfg!(unix) {
        clifi_dir = env::var("HOME").unwrap() + "/.clifi";
    }

    // DEFINE VLC PATH ON SYSTEM
    if env::consts::OS == "linux" {
        vlc_path = "/usr/bin/vlc";
    } else if env::consts::OS == "macos" {
        vlc_path = "/Applications/VLC.app/Contents/MacOS/VLC";
    } else if env::consts::OS == "windows" {
        vlc_path = "C:\\Program Files\\VideoLAN\\VLC\\vlc.exe";
    }

    // CHECK FOR REQUIRED DIRECTORIES/FILES
    if !Path::new(&format!("{}", vlc_path)).exists() {
        println!("VLC not found on your system\nclifi requires VLC to run\nIt can be found at https://www.videolan.org/vlc/");
        process::exit(1);
    }

    if !Path::new(&format!("{}", clifi_dir)).exists() {
        println!("clifi directory not found\nPlease run command 'make' from clifi repository\nIf repository is not available, it can be cloned from https://github.com/kevinshome/clifi/tree/rustrw");
        process::exit(1);
    }

    if !Path::new(&format!("{}/clifi.cfg", clifi_dir)).exists() {
        fs::copy(
            &format!("{}/defaults/clifi.cfg", clifi_dir),
            &format!("{}/clifi.cfg", clifi_dir),
        )?;
    }

    // READ CONFIG FILE AND SET DEFAULT STREAM
    let config_raw_string = fs::read_to_string(format!("{}/clifi.cfg", clifi_dir)).unwrap();
    let config_data = config_raw_string.parse::<Value>().unwrap();
    let default_stream = &config_data["config"]["default_stream"].to_string();

    // CLAP DEFINITIONS
    let matches = App::new("clifi")
        .version("221.1.0")
        .about("play your favorite streams straight from the command line")
        .arg(
            Arg::with_name("stream")
                .help("stream to play")
                .required(true)
                .default_value(default_stream),
        )
        .arg(
            Arg::with_name("kill")
                .short("k")
                .long("kill")
                .help("kill clifi instance")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("be verbose")
                .takes_value(false),
        )
        .get_matches();

    // IF VLC KILL IS REQUESTED WITH "-k" ARG
    if matches.is_present("kill") {
        match fs::remove_file(&format!("{}/clifi.lck", clifi_dir)) {
            Ok(_) => (),
            Err(error) => panic!("error while attempting to delete lockfile. if error persists, try manually. (~/.clifi/clifi.lck)\n{}", error),
        };
        match Popen::create(&["killall", "VLC"], PopenConfig::default()) {
            Ok(_) => process::exit(0),
            Err(error) => panic!("error while attempting to kill VLC instances. if error persists, try manually.\n{}", error),
        };
    }

    // STREAM NAME AND STREAM URL
    let stream_name = &format!("{}", matches.value_of("stream").unwrap()).replace('\"', "");
    let stream_url = config_data["streams"][stream_name]["url"]
        .to_string()
        .replace('\"', "");

    // VERBOSE PRINTS
    if matches.is_present("verbose") {
        println!("RUNNING_STREAM = {}", stream_name);
        println!("STREAM_URL = {}", stream_url);
        //println!("JSON_DATA = {}", json_data["streams"]);
        println!("CONFIG_DATA = {}", config_data["config"]["default_stream"]);
        println!("CLIFI_DIR = {}", clifi_dir);
        println!("VLC = {}", vlc_path);
    }

    // CHECK FOR LOCKFILE. IF EXISTS, EXIT
    if Path::new(&format!("{}/clifi.lck", clifi_dir)).exists() {
        process::exit(1);
    }

    // RUN STREAM IN HEADLESS VLC
    match Popen::create(
        &[vlc_path, "-I", "dummy", "-q", "--no-video", &stream_url],
        PopenConfig {
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            detached: true,
            ..Default::default()
        },
    ) {
        Ok(_) => fs::File::create(&format!("{}/clifi.lck", clifi_dir))?,
        Err(error) => panic!("error opening stream: {:?}", error),
    };

    Ok(())
}
