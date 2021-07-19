use clap::{Arg, App};

fn main() {
    println!("Hello, world!");

    let matches = App::new("eSports roster")
    .version("1.0")
    .author("Enzo <enzocuellar12@gmail.com>")
    .about("Build your eSports dream team")
    .arg(Arg::new("roster")
        .short('c')
        .long("config-file")
        .value_name("roster_style")
        .takes_value(true)
        .about("Which player do you want in your team?"))
    .get_matches();
}
