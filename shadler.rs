use std::{process::exit, io::Write};
use regex::Regex;
use serde_json::Value;
use utils::constants::{MAGENTA, BLUE, RED, RESET, YELLOW, GREEN};

mod utils;

fn shadler_help() {
    let help = "
Usage: shadler <subcommand> [options]

Example: shadler anime -v -r 8 -s -q 'oshi no ko'
         shadler manga -r 10 12 -s -q 'kaoru hana wa rin to saku'

Options:

    -q | --query <keywords>         Search anime or manga with specified keywords. Please enclose the keywords with quotes
    -v | --vlc                      Stream using VLC media player
    -n | --nextplay                 Stream using NextPlayer. Available only on Android
    -k | --mpvkt                    Stream using mpvKt. Available only on Android
    -s | --stream                   Stream anime episode or read manga chapters with online
    -d | --download                 Download anime episode or download manga chapters for offline reading
    -r | --range <lower> <upper>    Specify episode/chapter range
    -h | --help                     Show this help message
";

   println!("{help}");
}

fn shadler_prep(content_type: &str, args: utils::structs::CommandArguments) -> utils::structs::StreamContent {

    // variables that are taken from command line arguments
    let query_arg = args.query;
    let action_arg = args.action;
    let player_arg = args.player;
    let range_arg = args.range;

    let content_type_string = if content_type == "shows" { "anime" } else { "manga" };
    let content_part_string = if content_type == "shows" { "episodes" } else { "chapters" };

    // this one is handled internally, for making requests
    let query;
    let action;
    let player;
    let mut selected_episodes;

    if query_arg.is_empty() {
        query = utils::helper::shadler_string_input("Query: ");

    } else {
        query = query_arg;

    }

    let query_url = utils::api::shadler_get_query_url(content_type, &query);
    let query_response = utils::api::shadler_get_api_response(&query_url);
    let mut query_contents_vec = utils::helper::shadler_get_query_object(content_type, &query_response);

    let mut query_contents_len = 0;

    print!("\n");
    for x in &query_contents_vec {
        query_contents_len += 1;
        println!("{MAGENTA}[{query_contents_len}] {BLUE} {} {RESET} ", x.title);

    }

    let range = utils::helper::shadler_range_input(&format!("Select {content_type_string} [1-{}]: ", query_contents_len), 1, query_contents_len);

    let selected_index = range[0] as usize;
    let selected = query_contents_vec.swap_remove(selected_index-1);

    let selected_id = selected.id;
    let selected_title = selected.title;

    let detail_url = selected.detail_url;
    let detail_response = utils::api::shadler_get_api_response(&detail_url);

    let available_episodes = utils::helper::shadler_get_available_episodes(content_type, &detail_response);
    let available_episodes_len = available_episodes.len() as i32;

    if range_arg.is_empty() {
        selected_episodes = utils::helper::shadler_range_input(&format!("Select {content_part_string} [1-{}]: ", available_episodes_len), 1, available_episodes_len);

    } else {
        selected_episodes = range_arg;
        let range_is_valid = utils::helper::shadler_validate_range(&selected_episodes, 1, available_episodes_len);

        if let Err(e) = range_is_valid {
            eprintln!("{RED}{e}{RESET}\n");
            exit(1);

        }

    }

    // very hacky way to handle a single range episode input (this will be fed to a for loop)
    if selected_episodes.len() == 1 {
        selected_episodes.push(selected_episodes[0]);

    }

    if action_arg == 0 {
        println!("\n{MAGENTA}[1] {BLUE}Stream\n{MAGENTA}[2] {BLUE}Download{RESET}");
        action = utils::helper::shadler_range_input(&format!("Select action [1-2]: "), 1, 2)[0];

    } else {
        action = action_arg;

    }

    if player_arg.is_empty() {
        match std::env::consts::OS {
            "linux" => player = "mpv",
            "android" => player = "android_mpv",
            &_ => player = "mpv"

        }

    } else {
        player = &player_arg;

    }

    let stream_content = utils::structs::StreamContent {
        id: selected_id,
        title: selected_title,
        selected: selected_episodes,
        available: available_episodes,
        action: action,
        player: player.to_string()

    };

    return stream_content;

}

fn shadler_anime(stream_content: utils::structs::StreamContent) {

    let selected_id = stream_content.id;
    let selected_turtle = stream_content.title;
    let selected_episode = stream_content.selected;
    let action = stream_content.action;
    let selected_player = stream_content.player;
    let mut available_episodes_rev = stream_content.available;

    // reverse because API returns episodes in descending order instead of ascending
    available_episodes_rev.reverse();

    for x in selected_episode[0]..selected_episode[1]+1 {

        let current_selected = (x-1) as usize;
        let stream_url = utils::api::shadler_get_stream_url("shows", &selected_id, &available_episodes_rev[current_selected]);
        let stream_response = utils::api::shadler_get_api_response(&stream_url);

        let re = Regex::new("apivtwo/[^\"]*").unwrap();
        let matched = re.captures(&stream_response)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str();

        let mut video_source = String::from("https://blog.allanime.day/");
        video_source.push_str(
            &matched
            .replace("clock", "clock.json")
            .replace("/download", "")
        );

        let vid_source_response = utils::api::shadler_get_api_response(&video_source);
        let vid_src_json: Value = serde_json::from_str(&vid_source_response).unwrap();
        let video_link = vid_src_json
            ["links"][0]["link"]
            .as_str()
            .unwrap();

        if action == 1 {

            utils::player::shadler_stream_video(std::env::consts::OS, &selected_player, &selected_turtle, &video_link);

            if x < selected_episode[1] {
                println!("\n{MAGENTA}[1] {BLUE}Next episode\n{MAGENTA}[2] {BLUE}Quit{RESET}");

                let next_action = utils::helper::shadler_range_input("Select action [1-2]: ", 1, 2);
                let selected_action = next_action[0]; // ignore range input

                // quit application
                if selected_action == 2 {
                    exit(0);

                } else {
                    continue;
                }

            }

        } else if action == 2 {

            println!("\n{YELLOW}Downloading Episode {}..{}", x, RESET);
            let download_result = utils::downloader::shadler_download_file("shows", &video_link, &selected_turtle, &format!("Episode {x}.mp4"));

            match download_result {
                Err(e) => eprintln!("\n{RED}{e}{RESET}"),
                Ok(path) => println!("{GREEN}Episode {x} downloaded at {YELLOW}'{path}'!{RESET}")

            }

        }

    }

}

fn shadler_manga(stream_content: utils::structs::StreamContent) {

    let selected_id = stream_content.id;
    let selected_turtle = stream_content.title;
    let chapter_start = stream_content.selected[0];
    let chapter_end = stream_content.selected[1];
    let action = stream_content.action;
    let mut available_chapters_rev = stream_content.available;

    // reverse because API returns episodes in descending order instead of ascending
    available_chapters_rev.reverse();

    let chapters_file_info = utils::helper::shadler_create_file("mangas", &selected_turtle, &format!("chp{chapter_start}-{chapter_end}.html"));
    let chapters_file_path = chapters_file_info.1;
    let mut chapters_file = chapters_file_info.0;
    let mut page_collection = String::new();

    for x in chapter_start..chapter_end+1 {

        println!("\n{YELLOW}Loading chapter {}..{}", x, RESET);

        let current_selected = (x-1) as usize;
        let stream_url = utils::api::shadler_get_stream_url("mangas", &selected_id, &available_chapters_rev[current_selected]);
        let stream_response = utils::api::shadler_get_api_response(&stream_url);

        let page_source: Value = serde_json::from_str(&stream_response).unwrap();
        let page_url_head = page_source["data"]["chapterPages"]["edges"][0]["pictureUrlHead"].as_str().unwrap();
        let chapter_pages = page_source["data"]["chapterPages"]["edges"][0]["pictureUrls"].as_array().unwrap();
        let mut page_counter = 0;

        for current_page in chapter_pages {
            let page_path = current_page["url"].as_str().unwrap();
            let page_url = page_url_head.to_owned() + page_path;

            if action == 1 {
                let page_img_tag = format!("<img src='{page_url}' alt='Failed to load image'>\n");
                page_collection.push_str(&page_img_tag);

            } else if action == 2 {
                page_counter += 1;
                let download_result = utils::downloader::shadler_download_file("mangas", &page_url, &selected_turtle, &format!("chp{x}_{page_counter}"));

                if let Err(e) = download_result {
                    eprintln!("\n{RED}{e}{RESET}");
                    exit(1);

                } else if let Ok(path) = download_result {
                    let page_img_tag = format!("<img src='{path}' alt='Failed to load image'>\n");
                    page_collection.push_str(&page_img_tag);

                }
            }
        }
    }

    let reader_base = String::from(utils::constants::MANGA_READER_BASE);
    let reader = reader_base
        .replace("#TITLE#", &selected_turtle)
        .replace("#IMG_TAGS#", &page_collection);

    chapters_file.write_all(reader.as_bytes()).unwrap();

    if std::env::consts::OS == "android" {
        let termux_reader_file_info = utils::helper::shadler_create_file("mangas", &selected_turtle, &format!("read_{chapter_start}-{chapter_end}"));

        let termux_reader_file_path = termux_reader_file_info.1;
        let termux_reader_data_path = termux_reader_file_info.2;
        let mut termux_reader_file = termux_reader_file_info.0;

        let termux_http_server_base = String::from(utils::constants::TERMUX_HTTP_SERVER_BASE);
        let termux_http_server = termux_http_server_base
            .replace("#MANGA_PATH#", &termux_reader_data_path)
            .replace("#CHAPTER_START#", &format!("{chapter_start}"))
            .replace("#CHAPTER_STOP#", &format!("{chapter_end}"));

        termux_reader_file.write_all(termux_http_server.as_bytes()).unwrap();

        println!("\n{GREEN}HTML file generated. Start reading by running {YELLOW}'{termux_reader_file_path}'{RESET}");

    } else {
        println!("\n{GREEN}HTML file generated. Start reading by running {YELLOW}xdg-open '{chapters_file_path}'{RESET}");

    }

}

pub fn shadler_is_option(arg: &str) -> bool {

    if arg.starts_with("-") {
        return true;

    } else {
        return false;

    }

}

fn main() {

    let mut command_args = std::env::args_os();
    let subcommand_arg = command_args.nth(1);

    let mut query = String::new();
    let mut action = 0;
    let mut player = String::new();
    let mut range: Vec<i32> = Vec::new();

    while let Some(x) = command_args.next() {

        let option = x.into_string().unwrap();

        if option == "-q" || option == "--query"{
            let temp = command_args
                .next()
                .unwrap()
                .into_string()
                .unwrap();

            if !shadler_is_option(&temp) { query = temp }

        } else if option == "-s" || option == "--stream" {
            action = 1;

        } else if option == "-d" || option == "--download" {
            action = 2;

        } else if option == "-v" || option == "--vlc" {
            match std::env::consts::OS {
                "linux" => player = String::from("vlc"),
                "android" => player = String::from("android_vlc"),
                &_ => player = String::from("vlc")

            }

        } else if option == "-n" || option == "--nextplayer" {
            if std::env::consts::OS == "android" { player = String::from("android_nextplayer") }

        } else if option == "-k" || option == "--mpvkt" {
            if std::env::consts::OS == "android" { player = String::from("android_mpvkt") }

        } else if option == "-r" || option == "--range" {

            let range_lower = command_args
                .next()
                .unwrap_or("0".into())
                .into_string()
                .unwrap();

            if !shadler_is_option(&range_lower) && range_lower != "0" {
                if let Some(val) = range_lower.parse::<i32>().ok() {
                    range.push(val);

                }

            } else {
                continue;

            }

            let range_upper = command_args
                .next()
                .unwrap_or("0".into())
                .into_string()
                .unwrap();

            if !shadler_is_option(&range_upper) && range_upper != "0" {
                if let Some(val) = range_upper.parse::<i32>().ok() {
                    range.push(val);

                }

            } else {
                continue;

            }

        }

    }

    let shadler_args = utils::structs::CommandArguments {
        query: query,
        action: action,
        player: player,
        range: range

    };

    if let Some(val) = subcommand_arg {
        let subcommand = val.into_string().unwrap();

        if subcommand == "anime" {
            let streaming_info = shadler_prep("shows", shadler_args);
            shadler_anime(streaming_info);

        } else if subcommand == "manga" {
            let streaming_info = shadler_prep("mangas", shadler_args);
            shadler_manga(streaming_info);

        } else if subcommand == "help" || subcommand == "--help" || subcommand == "-h" {
            shadler_help();

        } else {
            eprintln!("{RED}ERROR: Unknown subcommand. Available subcommand is 'anime' and 'manga', and 'help'{RESET}");

        }

    } else {
        eprintln!("{RED}ERROR: No subcommand passed. Available subcommand is 'anime' and 'manga', and 'help'{RESET}");

    }

}

