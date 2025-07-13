mod utils;

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

    let query = utils::helper::shadler_string_input("Query: ");
    let query_url = utils::helper::shadler_get_query_url("shows", &query);
    let query_response = utils::helper::shadler_get_api_response(&query_url);

    let query_contents_wrap = utils::helper::shadler_get_query_object("shows", &query_response);

    if let Err(e) = query_contents_wrap {
        eprintln!("\n{}{}{}", utils::constants::RED, e, utils::constants::RESET);
        return;

    }

    let query_contents_vec = query_contents_wrap.unwrap();
    let mut query_contents_len = 0;

    print!("\n");
    for x in &query_contents_vec {
        query_contents_len += 1;
        println!("{}[{}] {} {} {} ", utils::constants::MAGENTA, query_contents_len, utils::constants::BLUE, x.title, utils::constants::RESET);

    }

    let mut range;
    loop {

        range = utils::helper::shadler_range_input(&format!("Select anime [1-{}]: ", query_contents_len), 1, query_contents_len);

        if let Err(e) = range {
            eprintln!("\n{}{}{}\n", utils::constants::RED, e, utils::constants::RESET);
            continue;

        } else {
            break;

        }

    }

    let selected = range.unwrap()[0] as usize;
    let selected_id = &query_contents_vec[selected-1].id;
    let detail_url = &query_contents_vec[selected-1].detail_url;
    let detail_response = utils::helper::shadler_get_api_response(&detail_url);

    println!("{}", detail_response);
    return;

    let detail_contents = utils::helper::shadler_get_available_episodes("show", &detail_response);

}

fn main() {
    
    shadler_anime();

}
