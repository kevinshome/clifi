use clap::{App, Arg};
use colored::*;
use std::{env, fs, panic, path::Path, process};
use subprocess::{Popen, PopenConfig, Redirection};
use toml_edit::{value, Document};

fn clifi_kill(clifi_dir: &str, switch: bool){
    match fs::remove_file(&format!("{}/clifi.lck", &clifi_dir)) {
        Ok(_) => (),
        Err(error) => {
            println!("{}\n[ {} ]", "error while attempting to delete lockfile.\nif error persists, try manually. ($ rm ~/.clifi/clifi.lck)".bright_red(), format!("{}", error).red());
            process::exit(1);
        }
    };
    match Popen::create(&["killall", "VLC"], PopenConfig::default()) {
        Ok(_) => {
            if !switch{
                process::exit(0)
            }
        },
        Err(error) => {
            println!("{}\n[ {} ]", "error while attempting to kill VLC instances. if error persists, try manually. ($ killall VLC)".bright_red(), format!("{}", error).red());
            process::exit(1);
        }
    };
}

fn check_for_default(data: &toml_edit::Document) {
    /*
    check for default stream in config file
    */

    let def = data["config"]["default_stream"].as_str().unwrap();

    let result = panic::catch_unwind(|| {
        data["streams"][def]["full_name"].as_str().unwrap();
    });
    if result.is_err() {
        println!(
            "{}{} ['{}'] {}\n{}",
            "ERROR: ".bold().bright_red(),
            "Your default stream".bright_red(),
            def,
            "cannot be found.".bright_red(),
            "Please check your configuration file at '~/.clifi/clifi.cfg'".bright_red()
        );
        process::exit(1);
    }
}

fn init_checks(vlc_path: &str, clifi_dir: &str) -> std::io::Result<()> {
    /*
    necessary pre-run checks and catches
    */

    // CHECK FOR VLC
    if !Path::new(&format!("{}", vlc_path)).exists() {
        println!(
            "{}{}\n{}\n{}",
            "ERROR: ".bold().bright_red(),
            "VLC not found on your system".bright_red(),
            "clifi requires VLC to run".bright_red(),
            "It can be found at https://www.videolan.org/vlc/".bright_red()
        );
        process::exit(1);
    }

    //CHECK FOR CLIFI DIRECTORY
    if !Path::new(&format!("{}", clifi_dir)).exists() {
        println!(
            "{}{}\n{}\n{}",
            "ERROR: ".bold().bright_red(), 
            "clifi directory not found".bright_red(), 
            "Please run command 'make' from clifi repository".bright_red(), 
            "If repository is not available, it can be cloned from https://github.com/kevinshome/clifi/tree/rustrw".bright_red()
        );
        process::exit(1);
    }

    //CHECK FOR CLIFI CONFIG FILE
    if !Path::new(&format!("{}/clifi.cfg", clifi_dir)).exists() {
        fs::copy(
            &format!("{}/defaults/clifi.cfg", clifi_dir),
            &format!("{}/clifi.cfg", clifi_dir),
        )?;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    /*
    main function
    */

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

    let _init = init_checks(&vlc_path, &clifi_dir);

    // READ CONFIG FILE TO MUTABLE AND IMMUTABLE VARS
    let config_raw_string = format!(
        r#"{}"#,
        fs::read_to_string(format!("{}/clifi.cfg", clifi_dir)).unwrap()
    );
    let immut_config_data = config_raw_string.parse::<Document>().expect("invalid doc"); // a workaround
    let mut config_data = config_raw_string.parse::<Document>().expect("invalid doc");
    let default_stream = immut_config_data["config"]["default_stream"]
        .as_str()
        .unwrap();
    check_for_default(&immut_config_data);

    // CLAP DEFINITIONS
    let matches = App::new("clifi")
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
            Arg::with_name("add-stream")
                .long("add-stream")
                .help("add a new stream to config file")
                .takes_value(true)
                .value_name("NAME"),
        )
        .arg(
            Arg::with_name("rm-stream")
                .long("rm-stream")
                .help("remove a stream from config file")
                .takes_value(true)
                .value_name("NAME"),
        )
        .arg(
            Arg::with_name("list-streams")
                .short("l")
                .long("list-streams")
                .help("return a list of all available streams in config file")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("be verbose")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("switch")
                .short("s")
                .long("switch")
                .help("switch to another stream")
                .takes_value(true)
                .value_name(""),
        )
        .get_matches();

    // STREAMS MANIPULATION
    if matches.is_present("add-stream") {
        let stream_name = matches.value_of("add-stream").unwrap();
        let mut stream_full_name = String::new();
        let mut stream_url = String::new();

        println!("Full name of stream (not required, but recommended): ");
        std::io::stdin().read_line(&mut stream_full_name).unwrap();
        println!("Stream URL: ");
        std::io::stdin().read_line(&mut stream_url).unwrap();

        config_data["streams"][&stream_name.trim()]["url"] = value(stream_url.trim());

        if stream_full_name.trim() == "" {
            config_data["streams"][&stream_name.trim()]["full_name"] = value("null");
        } else {
            config_data["streams"][&stream_name.trim()]["full_name"] =
                value(stream_full_name.trim());
        }

        let _nil = fs::write(
            format!("{}/clifi.cfg", &clifi_dir),
            &config_data.to_string(),
        );
        process::exit(0);
    }
    if matches.is_present("rm-stream") {
        config_data["streams"]
            .as_table_mut()
            .unwrap()
            .remove(matches.value_of("rm-stream").unwrap());
        let _nil = fs::write(
            format!("{}/clifi.cfg", &clifi_dir),
            &config_data.to_string(),
        );
        process::exit(0);
    }
    if matches.is_present("list-streams") {
        let _table_like = config_data["streams"].as_table().unwrap();
        for item in _table_like.iter() {
            println!(
                "{} ({})",
                item.1.as_inline_table()
                    .unwrap()
                    .get("full_name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .replace("\"", ""),
                item.0
            );
        }
        process::exit(0);
    }

    // IF VLC KILL IS REQUESTED WITH "-k" ARG
    if matches.is_present("kill") {
        clifi_kill(&clifi_dir, false);
    }

    // STREAM NAME AND STREAM URL
    let stream_name: String;

    if matches.is_present("switch"){
        clifi_kill(&clifi_dir, true);
        stream_name = format!("{}", matches.value_of("switch").unwrap()).replace('\"', "");
        println!("Switching to: {}", stream_name);
    } else {
        stream_name = format!("{}", matches.value_of("stream").unwrap()).replace('\"', "");
    }

    let stream_url = match config_data["streams"][&stream_name]["url"].as_str() {
        Some(value) => value.replace('\"', ""),
        None => {
            println!("stream '{}' not found", matches.value_of("stream").unwrap());
            process::exit(1);
        }
    };

    let stream_name_full = config_data["streams"][&stream_name]["full_name"]
        .as_str()
        .unwrap()
        .replace("\"", "");

    // CHECK FOR LOCKFILE. IF EXISTS, EXIT
    if Path::new(&format!("{}/clifi.lck", &clifi_dir)).exists() {
        process::exit(1);
    }

    // RUN STREAM IN HEADLESS VLC
    match Popen::create(
        &[vlc_path, "-I", "dummy", "-q", "--no-video", &stream_url],
        //&[vlc_path, "-vvv", &stream_url], //for diagnostics
        PopenConfig {
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            detached: true,
            ..Default::default()
        },
    ) {
        Ok(_) => {
            fs::File::create(&format!("{}/clifi.lck", &clifi_dir))?;
            println!("Running stream: {} ({})", &stream_name_full, &stream_name);
        }
        Err(error) => panic!("error opening stream: {:?}", error),
    };

    Ok(())
}
