use clap::{App, Arg, ArgMatches};

fn get_role(matches: ArgMatches) {
    if let Some(i) = matches.value_of("role") {
        println!("Role: {}", i);
    }
}

fn main() {
    println!("Hello, world!");

    let matches = App::new("eSports roster")
        .version("1.0")
        .author("Enzo <enzocuellar12@gmail.com>")
        .about("Build your eSports dream team")
        .arg(
            Arg::new("select_player")
                .short('a')
                .long("add-player")
                .value_name("player_name")
                .takes_value(true)
                .multiple_occurrences(true)
                .about("Which player do you want in your team?"),
        )
        .arg(
            Arg::new("roster")
                .short('o')
                .long("show-roster")
                .value_name("roster_option")
                .takes_value(true)
                .about("Show the players that you have in your team"),
        )
        .arg(
            Arg::new("assign_role")
                .required(true)
                .short('w')
                .long("assign_a_role")
                .value_name("assign_option")
                .takes_value(true)
                .about("Select in which role the player will play"),
        )
        .arg(
            Arg::new("role")
                .short('r')
                .long("role")
                .required_if_eq("assign_role", "role")
                .value_name("role_option")
                .takes_value(true)
                .about("Select in which role the player will play"),
        )
        .get_matches();

    if let Some(i) = matches.values_of("select_player") {
        let vals: Vec<&str> = i.collect();
        for val in vals {
            match val {
                "faker" => println!(
                    "The best player in the world {} was added to your team!",
                    val
                ),
                "jankos" => println!("{} was added to your team", val),
                "caps" => println!("{} was added to your team", val),
                "rekkles" => println!("{} was added to your team", val),
                "promisq" => println!("{} was added to your team", val),
                "khan" => println!("{} was added to your team", val),
                _ => println!("This player isn´t available to enter to the team :("),
            };
        }
        //println!("Your actual players: {:#?}", val);
    }

    if let Some(i) = matches.value_of("roster") {
        match i {
            "s" => println!("The best player in the world {} was added to your team!", i),
            _ => println!("This player isn´t available to enter to the team :("),
        };
    }
    if let Some(i) = matches.value_of("assign_role") {
        match i {
            "role" => get_role(matches),
            _ => println!("This player isn´t available to enter to the team :("),
        };
    }
}
