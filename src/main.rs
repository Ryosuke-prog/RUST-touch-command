// BSD 3-Clause License
//
// Copyright (c) 2023, Ryosuke
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
//    this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

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
        .author("Unixtech-06")
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
