use std::io::Write;
use std::process::exit;
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
";

   println!("{}", help);
}

fn shadler_prep(content_type: &str) -> (i32, utils::structs::StreamContent) {

    let content_type_string = if content_type == "shows" { "anime" } else { "manga" };
    let content_part_string = if content_type == "shows" { "episodes" } else { "chapters" };

    let query = utils::helper::shadler_string_input("Query: ");
    let query_url = utils::api::shadler_get_query_url(content_type, &query);
    let query_response = utils::api::shadler_get_api_response(&query_url);

    let query_contents_wrap = utils::helper::shadler_get_query_object(content_type, &query_response);

    if let Err(e) = query_contents_wrap {
        eprintln!("\n{}{}{}", RED, e, RESET);
        exit(1);

    }

    let query_contents_vec = query_contents_wrap.unwrap();
    let mut query_contents_len = 0;

    print!("\n");
    for x in &query_contents_vec {
        query_contents_len += 1;
        println!("{}[{}] {} {} {} ", MAGENTA, query_contents_len, BLUE, x.title, RESET);

    }

    let range = utils::helper::shadler_range_input(&format!("Select {} [1-{}]: ", content_type_string, query_contents_len), 1, query_contents_len);

    let selected = range[0] as usize;
    let selected_id = &query_contents_vec[selected-1].id;
    let selected_title = &query_contents_vec[selected-1].title;

    let detail_url = &query_contents_vec[selected-1].detail_url;
    let detail_response = utils::api::shadler_get_api_response(&detail_url);

    let available_episodes = utils::helper::shadler_get_available_episodes(content_type, &detail_response);
    let available_episodes_len = available_episodes.len();

    let mut selected_episodes = utils::helper::shadler_range_input(&format!("Select {} [1-{}]: ", content_part_string, available_episodes_len), 1, available_episodes_len as i32);

    // very hacky way to handle a single range episode input (this will be fed to a for loop)
    if selected_episodes.len() == 1 {
        selected_episodes.push(selected_episodes[0]);
    
    }

    println!("\n{}[1] {}Stream\n{}[2] {}Download{}", MAGENTA, BLUE, MAGENTA, BLUE, RESET);

    let action = utils::helper::shadler_range_input(&format!("Select action [1-2]: "), 1, 2);

    let stream_content = utils::structs::StreamContent {
        id: selected_id.clone(),
        title: selected_title.clone(),
        selected: selected_episodes,
        available: available_episodes

    };

    let stream_info = (action[0], stream_content);
    return stream_info;

}

fn shadler_anime(info: (i32, utils::structs::StreamContent)) {

    let action = info.0;
    let stream_content  = info.1;

    let selected_id = stream_content.id;
    let selected_turtle = stream_content.title;
    let selected_episode = stream_content.selected;
    let mut available_episodes_rev = stream_content.available;

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

            utils::helper::shadler_stream_video(std::env::consts::OS, &selected_turtle, &video_link);

            if x < selected_episode[1] {
                println!("\n{}[1] {}Next episode\n{}[2] {}Quit{}", MAGENTA, BLUE, MAGENTA, BLUE, RESET);

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

            println!("\n{}Downloading Episode {}..{}", YELLOW, x, RESET);
            let download_result = utils::downloader::shadler_download_anime(&video_link, &selected_turtle, &x.to_string());

            match download_result { 
                Err(e) => eprintln!("\n{}{}{}", RED, e, RESET),
                Ok(path) => println!("{}Episode {} downloaded at {}'{}'!{}", GREEN, x, YELLOW, path, RESET)

            }

        }

    }

}

fn shadler_manga(info: (i32, utils::structs::StreamContent)) {

    let _action = info.0;
    let stream_content  = info.1;

    let selected_id = stream_content.id;
    let selected_turtle = stream_content.title;
    let chapter_start = stream_content.selected[0];
    let chapter_end = stream_content.selected[1];
    let mut available_chapters_rev = stream_content.available;

    available_chapters_rev.reverse();

    let chapters_file_info = utils::helper::shadler_create_file("mangas", &selected_turtle, &format!("chp{chapter_start}-{chapter_end}.html"));
    let chapters_file_path = chapters_file_info.1;
    let mut chapters_file = chapters_file_info.0;
    let mut page_collection = String::new();

    for x in chapter_start..chapter_end+1 {

        println!("\n{}Loading chapter {}..{}", YELLOW, x, RESET);

        let current_selected = (x-1) as usize;
        let stream_url = utils::api::shadler_get_stream_url("mangas", &selected_id, &available_chapters_rev[current_selected]);
        let stream_response = utils::api::shadler_get_api_response(&stream_url);

        let page_source: Value = serde_json::from_str(&stream_response).unwrap();
        let page_url_head = page_source["data"]["chapterPages"]["edges"][0]["pictureUrlHead"].as_str().unwrap();
        let chapter_pages = page_source["data"]["chapterPages"]["edges"][0]["pictureUrls"].as_array().unwrap();

        for current_page in chapter_pages {
            let page_path = current_page["url"].as_str().unwrap();
            let page_url = page_url_head.to_owned() + page_path;

            let page_img_tag = format!("<img src='{page_url}' alt='Failed to load image'>\n");
            page_collection.push_str(&page_img_tag);

        }

    }

    let reader_base = String::from(utils::constants::MANGA_READER_BASE);
    let reader = reader_base.replace("#TITLE#", &selected_turtle).replace("#IMG_TAGS#", &page_collection);

    chapters_file.write_all(reader.as_bytes()).unwrap();

    println!("\n{}HTML file generated. Start reading by running {}xdg-open '{}'{}", GREEN, YELLOW, chapters_file_path, RESET);

}

fn main() {

    let subcommand_arg = std::env::args_os().nth(1);

    if let Some(val) = subcommand_arg {
        let subcommand = val.into_string().unwrap();

        if subcommand == "anime" {
            let streaming_info = shadler_prep("shows");
            shadler_anime(streaming_info);

        } else if subcommand == "manga" {
            let streaming_info = shadler_prep("mangas");
            shadler_manga(streaming_info);

        } else if subcommand == "help" {
            shadler_help();

        } else { 
            eprintln!("{}ERROR: Unknown subcommand. Available subcommand is 'anime' and 'manga', and 'help'{}", RED, RESET);

        }

    } else {
        eprintln!("{}ERROR: No subcommand passed. Available subcommand is 'anime' and 'manga', and 'help'{}", RED, RESET);

    }

}
    
