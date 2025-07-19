use std::io;
use std::process::{Command, Stdio};
use std::io::Write;
use std::fs;
use std::path::Path;
use serde_json::Value;

use super::constants;
use super::structs;
use super::api;

pub fn shadler_string_input(prompt: &str) -> String {

    print!("{}{}{}", constants::MAGENTA, prompt, constants::RESET);

    let mut input = String::new();
    io::stdout().flush().unwrap(); // flush manually because stdout flush on newlines and we dont want that
    io::stdin().read_line(&mut input).unwrap();

    return input.trim().to_owned();

}

pub fn shadler_range_input(prompt: &str, lower: i32, upper: i32) -> Vec<i32> {

    loop {

        print!("{}{}{}", constants::MAGENTA, prompt, constants::RESET);

        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        
        let ranges: Vec<i32> = input
            .trim()
            .split(" ")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        if ranges.len() == 0 {
            eprintln!("{}ERROR: Invalid input{}\n", constants::RED, constants::RESET);
            continue;

        } else if ranges.len() == 1 {

            if ranges[0] < lower || ranges[0] > upper {
                eprintln!("{}ERROR: Invalid range{}\n", constants::RED, constants::RESET);
                continue;

            } else {
                return ranges;

            }

        } else {

            if ranges[0] < lower || ranges[1] > upper || ranges[0] > ranges[1] || ranges[1] < ranges[0] {
                eprintln!("{}ERROR: Invalid range{}\n", constants::RED, constants::RESET);
                continue;

            } else {
                return ranges;

            }

        }

    }

}

pub fn shadler_create_file(content_type: &str, title: &str, file_name: &str) -> (fs::File, String) {

    let content_string = if content_type == "shows" { "anime" } else { "manga" };

    let home_buf = std::env::home_dir().unwrap();
    let home_dir = home_buf.to_str().unwrap();

    let mut content_data_dir = String::from(home_dir);
    content_data_dir.push_str(&format!("/.local/share/shadler/{content_string}/"));
    content_data_dir.push_str(title);

    let mut content_file_dir = String::from(&content_data_dir);
    content_file_dir.push_str(&format!("/{file_name}"));

    let content_data_path = Path::new(&content_data_dir);
    let content_file_path = Path::new(&content_file_dir);

    if content_data_path.exists() == false {
        fs::create_dir_all(content_data_dir).unwrap();

    }

    let content_file = fs::File::create(content_file_path).unwrap();

    return (content_file, content_file_path.to_str().unwrap().to_owned());

}

pub fn shadler_get_query_object(content_type: &str, resp: &str) -> Result<Vec<structs::QueryContent>, String> {

    let response_json: Value = serde_json::from_str(resp).unwrap();
    let results = response_json["data"][content_type]["edges"].as_array().unwrap();

    // check if there is no query results
    if results.len() == 0 {
        return Err(format!("No results.."));
    }

    let mut contents: Vec<structs::QueryContent> = Vec::new();

    for x in results {
        contents.push(
            structs::QueryContent {
                id: x["_id"].as_str().unwrap().to_owned(),
                title: x["name"].as_str().unwrap().to_owned(),
                detail_url: api::shadler_get_detail_url(content_type, x["_id"].as_str().unwrap())
            }
        )
    }

    return Ok(contents);

}

pub fn shadler_get_available_episodes(content_type: &str, resp: &str) -> Vec<String> {

    // another stupid hack because the key is called "show" / "manga", not "shows" / "mangas"
    let mut key_type = String::from(content_type);
    key_type.pop();

    let episodes_key = if content_type == "shows" { "availableEpisodesDetail" } else { "availableChaptersDetail" };

    let response_json: Value = serde_json::from_str(resp).unwrap();
    let result = response_json["data"][&key_type][episodes_key]["sub"].as_array().unwrap();
    let episodes: Vec<String> = result
        .into_iter()
        .map(|x| x.as_str().unwrap().to_owned())
        .collect();

    return episodes;

}

pub fn shadler_stream_video(platform: &str, title: &str, link: &str) {

    if platform == "linux" {
    Command::new("mpv")
        .args([format!("--force-media-title={title}"), format!("{link}")])
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .unwrap();

    } else if platform == "android" {
    Command::new("am")
        .args(["start", "--user", "0", "-a", "android.intent.action.VIEW", "-n", "live.mehiz.mpvkt/.ui.player.PlayerActivity", "-d", link])
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .unwrap();

    }

}
