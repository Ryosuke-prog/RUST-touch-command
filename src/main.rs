use clap::{App, Arg};
use std::fs::OpenOptions;

fn main() {
    let mut app = App::new("touch")
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
        .arg(Arg::with_name("m")
             .short("m")
             .help("Change only the modification time"));

    // 引数のマッチした結果を取得
    let matches_result = app.clone().get_matches_safe();

    // エラーが存在する場合、USAGEを出力する
    if let Err(e) = matches_result {
        if e.kind == clap::ErrorKind::MissingRequiredArgument && e.message.contains("<FILE>") {
            app.print_help().unwrap();  // USAGEを出力する
            println!();  
            std::process::exit(1);
        } else {
            e.exit();
        }
    }

    let matches = matches_result.unwrap();

    if let Some(file_name) = matches.value_of("FILE") {
        // ファイルを作成する
        match OpenOptions::new().create(true).write(true).open(file_name) {
            Ok(_) => println!("{} created successfully.", file_name),
            Err(e) => eprintln!("Error creating file: {}", e),
        }
    }
}

