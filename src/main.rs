use clap::{App, Arg};
use filetime::{FileTime, set_file_times};
use chrono::NaiveDateTime;
use std::fs::{OpenOptions, metadata};
use std::path::Path;
use std::io;

// Main function: Parses command line arguments and processes the file.
fn main() -> io::Result<()> {
    // Setup and parse command line arguments using clap.
    let matches = App::new("touch")
        .version("1.0")
        .author("Your Name")
        .about("Emulates the touch command")
        .arg(Arg::with_name("FILE")
             .help("Sets the input file to use")
             .required(true)
             .index(1))
        .arg(Arg::with_name("a")
             .short("a")
             .help("Change only the access time"))
        .arg(Arg::with_name("c")
             .short("c")
             .help("Do not create any files"))
        .arg(Arg::with_name("d")
             .short("d")
             .takes_value(true)
             .help("Use specified time instead of current time"))
        .arg(Arg::with_name("m")
             .short("m")
             .help("Change only the modification time"))
        .arg(Arg::with_name("r")
             .short("r")
             .takes_value(true)
             .help("Use this file's times instead of current time"))
        .arg(Arg::with_name("t")
             .short("t")
             .takes_value(true)
             .help("Use specified time instead of current time"))
        .get_matches();

    // Extract file name from command line arguments.
    let file_name = matches.value_of("FILE").unwrap();
    let path = Path::new(file_name);

    // Determine flags set by the user.
    let aflag = matches.is_present("a");
    let cflag = matches.is_present("c");
    let mflag = matches.is_present("m");

    // Initialize access and modification times to current time.
    let mut atime = FileTime::now();
    let mut mtime = FileTime::now();

    // Parse and set custom time if -d or -t option is provided.
    if let Some(time_str) = matches.value_of("d").or(matches.value_of("t")) {
        let datetime = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%dT%H:%M:%S")
            .expect("Failed to parse date time");
        let timestamp = datetime.timestamp();
        atime = FileTime::from_unix_time(timestamp, 0);
        mtime = FileTime::from_unix_time(timestamp, 0);
    }

    // Use file's times if -r option is provided.
    if let Some(ref_file) = matches.value_of("r") {
        let metadata = metadata(ref_file)?;
        atime = FileTime::from_last_access_time(&metadata);
        mtime = FileTime::from_last_modification_time(&metadata);
    }

    // Check if the file exists.
    let file_exists = path.exists();

    // Do not create the file if -c is specified and file does not exist.
    if cflag && !file_exists {
        return Ok(());
    }

    // Create or open the file based on the provided flags.
    let file = OpenOptions::new().create(!cflag).write(true).open(&path);
    match file {
        Ok(_) => {
            // Set file times based on the flags.
            if !aflag {
                set_file_times(&path, atime, mtime)?;
            }
            if !mflag {
                set_file_times(&path, atime, mtime)?;
            }
        }
        Err(e) => {
            // Handle file processing errors.
            eprintln!("Error processing file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
