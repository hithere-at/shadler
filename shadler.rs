use std::io;
use std::io::Write;

mod utils;

static RESET: &'static str = "\x1B[0m";
static MAGENTA: &'static str = "\x1B[1;35m";
static YELLOW: &'static str = "\x1B[1;33m";
static GREEN: &'static str = "\x1B[1;32m";
static RED: &'static str ="\x1B[1;31m";
static BLUE: &'static str ="\x1B[1;34m";

fn shadler_help() {
    let help = "
Usage: shadler <subcommand> [options]

Example: shadler anime -v -r 8 -s -q 'oshi no ko'
         shadler manga -r 10 12 -s -q 'kaoru hana wa rin to saku'

Options:

    -q | --query <keywords>         Search anime or manga with specified keywords. Please quote the keywords
    -v | --vlc                      Stream using VLC media player
    -n | --nextplay                 Stream using NextPlayer. Available only on Android
    -s | --stream                   Stream anime episode or read manga chapters with online
    -d | --download                 Download anime episode or download manga chapters for offline reading
    -r | --range <lower> <upper>    Specify episode/chapter range
";

   println!("{}", help);
}

fn shadler_anime() {
    print!("{}Query:{} ", MAGENTA, RESET);
    
    let mut input = String::new();
    io::stdout().flush().unwrap(); // flush because stdout flush on newlines and we dont want that
    io::stdin().read_line(&mut input).unwrap();
    let query = input.trim();

    let query_url = utils::helper::shadler_get_query_url("shows", query);
    let query_response = utils::helper::shadler_get_api_response(&query_url);

    let query_contents = utils::helper::shadler_get_query_object("shows", &query_response);

    if let Err(e) = query_contents {
        eprintln!("\n{}{}{}", RED, e, RESET);
        return;

    }

    print!("\n");
    for (x, val) in query_contents.unwrap().iter().enumerate() {
        println!("{}[{}] {} {} {} ", MAGENTA, x+1, BLUE, val.title, RESET);

    }

}

fn main() {
    
    shadler_anime();

}
