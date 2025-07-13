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

    let available_episodes = utils::helper::shadler_get_available_episodes("show", &detail_response);
    let available_episodes_len = available_episodes.len();

    let mut episode_range;
    loop {

        episode_range = utils::helper::shadler_range_input(&format!("Select episode [1-{}]: ", available_episodes_len), 1, available_episodes_len as i32);

        if let Err(e) = episode_range {
            eprintln!("\n{}{}{}\n", utils::constants::RED, e, utils::constants::RESET);
            continue;

        } else {
            break;

        }

    }

    let mut selected_episode = episode_range.unwrap();

    // very hacky way to handle a single range
    if selected_episode.len() == 1 {
        selected_episode.push(selected_episode[0]);
    
    }

    let mut action;
    loop {

        action = utils::helper::shadler_range_input(&format!("Select action [1-2]: "), 1, 2);

        if let Err(e) = action {
            eprintln!("\n{}{}{}\n", utils::constants::RED, e, utils::constants::RESET);
            continue;

        } else {
            break;

        }

    }

    for x in selected_episode[0]..selected_episode[1] {

    }

}

fn main() {
    
    shadler_anime();

}
