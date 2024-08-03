use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
    .version("0.1.0")
    .author("GYU")
    .about("Rust version of 'echo'")
    .arg(
        Arg::new("text")
        .value_name("TEXT")
        .help("Input test")
        .required(true)
        .num_args(1..),
    )
    .arg(
        Arg::new("omit_newline")
        .short('n')
        .action(ArgAction::SetTrue)
        .help("Do not print newline")
    )
    .get_matches();

    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();
    let omit_newline = matches.get_flag("omit_newline");

    if omit_newline == true {
        //        println!("[TRUE] : OMIT_NEWLINE");
        print!("{}\n", text.join(" "));
    } else {
//        println!("[FALSE] : OMIT_NEWLINE");
        print!("{}", text.join(" "));
    }

}
