use clap::{Arg, App};

fn main() {
    println!("Hello, world!");

    let matches = App::new("eSports roster")
    .version("1.0")
    .author("Enzo <enzocuellar12@gmail.com>")
    .about("Build your eSports dream team")
    .arg(Arg::new("select_player")
        .short('a')
        .long("add-player")
        .value_name("player_name")
        .takes_value(true)
        .multiple_occurrences(true)
        .about("Which player do you want in your team?"))
        .arg(Arg::new("roster")
            .short('o')
            .long("show-roster")
            .value_name("roster_option")
            .takes_value(true)
            .about("Which player do you want in your team?"))
    .get_matches();

    let vals: Vec<&str> = matches.values_of("select_player").unwrap().collect();

    println!("{:#?}", vals);

    if let Some(i) = matches.value_of("select_player") {

        
    
        match i {
            "faker" => println!("The best player in the world {} was added to your team!", i),
            "jankos" => println!("{} was added to your team", i),
            "caps" => println!("{} was added to your team", i),
            "rekkles" => println!("{} was added to your team", i),
            "promisq" => println!("{} was added to your team", i),
            "khan" => println!("{} was added to your team",  i),
            _ => println!("This player isn´t available to enter to the team :(")
        };
        
        println!("Your actual players: {}", i);
    } 
    
    if let Some(i) = matches.value_of("roster") {

        match i {
           "s" => println!("The best player in the world {} was added to your team!", i),
            _ => println!("This player isn´t available to enter to the team :(")
        };

    } 

}
