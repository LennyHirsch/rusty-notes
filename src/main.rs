/// Command line tool to create a note in the notes directory and open this for editing in neovim
use std::process::Command;
use std::env;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize};

fn main() {
    // Collect args: expects a filename
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("ERROR: Not enough arguments. Please insert a filename") };
    let filename = &args[1];

    // Import config
    let config = import_config().expect("ERROR: Failed to import config");

    // Create the new note in the notes directory
    let directory = new_note(filename.to_string(), config.directory, config.default_format).expect("ERROR: Failed to create note.");

    // Open the created note in neovim for editing
    open_note(directory);
}

#[derive(Debug, Deserialize)]
struct Config {
    directory: String,
    default_format: String,
}

/// Imports config containing notes directory and default note format from config.json
fn import_config() -> Result<Config, ()> {
    // Getting the path of the executable, so it reads config.json in its own directory.
    let mut exec_path = env::current_exe()
        .expect("ERROR: Failed to get executable path.") // Type: PathBuf
        .to_str() // so we convert to str. This lets us use strip_suffix as well.
        .expect("ERROR: Failed to convert executable path.")
        .strip_suffix("\\rusty_notes.exe") // Remove .exe from the path, we want to target config
        .expect("ERROR: Failed to format executable path.")
        .to_string(); // Convert to String, so we can use push_str to add config file.

    exec_path.push_str("/config.json");

    let config_file = fs::read_to_string(&exec_path).expect("ERROR: Could not read config.");
    let config: Config = serde_json::from_str(config_file.as_str()).expect("ERROR: Could not read json.");

    Ok(config)
}

/// Checks if given filename has a file extension. Returns true if a file extension exists.
fn has_suffix(filename: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\.[A-Za-z]+").unwrap();
    }
    RE.is_match(filename)
}

/// Builds the full filename, adds a .md suffix if required, and checks if the file exists. If not,
/// creates the file. Returns the full address of the file as a String.
fn new_note(filename: String, mut directory: String, format: String) -> Result<String, ()> {
    // Check if slash is included at the end of the address. If not, add it.
    if !directory.ends_with("/") {
        directory.push_str("/")
    }

    // Build full filename, so notes can be created from anywhere.
    directory.push_str(&filename);

    // Check if filename with file extension is given. If not, make this a markdown file.
    if !has_suffix(&filename) {
        directory.push_str(format.as_str());
    }

    // Attempt to create a new note file. If a file with that name already exists, we get an Err
    // result. This is fine; we match the result and simply pass the filename on to open_note().
    let result = fs::OpenOptions::new().write(true)
        .create_new(true)
        .open(&directory);

    // If file didn't exist, we write a rustynote tag. This will help with e.g. logseq integration.
    match result {
        Ok(_) => { // file doesn't exist. We create it, then add the rustynotes tag
            fs::write(&directory, "#rustynote").expect("ERROR: Unable to write file")
        }
        Err(_) => {} // file exists. Don't need to write.
    }

    Ok(directory)
}

/// Opens the note in neovim for editing.
fn open_note(directory: String) {
    let _ = Command::new("cmd")
        .arg("/C")
        .arg("nvim")
        .arg(directory)
        .spawn()
        .expect("ERROR: Failed to run nvim")
        .wait()
        .expect("ERROR: Failed to open note");
}
