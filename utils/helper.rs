use std::io;
use std::process::{Command, Stdio};
use std::io::Write;
use serde_json::Value; 

use super::constants;
use super::structs;

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

pub fn shadler_get_query_url(query_type: &str, query: &str) -> String {

    let good_query = query.replace(" ", "%20");
    let mut query_var = String::new();
    let mut ext_var = String::new();

    if query_type == "shows" {
        query_var = constants::ANIME_QUERY_VARS.replace("#QUERY#", &good_query);
        ext_var = constants::API_EXT.replace("#HASH#", constants::ANIME_QUERY_HASH);

    } else if query_type == "mangas" {
        query_var = constants::MANGA_QUERY_VARS.replace("#QUERY#", &good_query);
        ext_var = constants::API_EXT.replace("#HASH#", constants::MANGA_QUERY_HASH);

    }

    let mut query_url = String::from("https://api.allanime.day/api?variables=");
    query_url.push_str(&query_var);
    query_url.push_str("&extensions=");
    query_url.push_str(&ext_var);

    return query_url;

}

pub fn shadler_get_detail_url(detail_type: &str, id: &str) -> String {

    let mut ext_var = String::new();
    let detail_var = constants::DETAIL_VARS.replace("#ID#", id);

    if detail_type == "shows" {
        ext_var = constants::API_EXT.replace("#HASH#", constants::ANIME_DETAIL_HASH);

    } else if detail_type == "mangas" {
        ext_var = constants::API_EXT.replace("#HASH#", constants::MANGA_DETAIL_HASH);

    }

    let mut detail_url = String::from("https://api.allanime.day/api?variables=");
    detail_url.push_str(&detail_var);
    detail_url.push_str("&extensions=");
    detail_url.push_str(&ext_var);

    return detail_url;

}

pub fn shadler_get_stream_url(detail_type: &str, id: &str, episode: &str) -> String {

    let mut ext_var = String::new();
    let mut stream_var = String::new(); 

    if detail_type == "shows" {
        stream_var = constants::ANIME_STREAM_VARS.replace("#ANIME_ID#", id).replace("#EPISODE#", episode);
        ext_var = constants::API_EXT.replace("#HASH#", constants::ANIME_STREAM_HASH);

    } else if detail_type == "mangas" {
        stream_var = constants::MANGA_READ_VARS.replace("#MANGA_ID#", id).replace("#CHAPTER#", episode);
        ext_var = constants::API_EXT.replace("#HASH#", constants::MANGA_READ_HASH);

    }

    let mut stream_url = String::from("https://api.allanime.day/api?variables=");
    stream_url.push_str(&stream_var);
    stream_url.push_str("&extensions=");
    stream_url.push_str(&ext_var);

    return stream_url;

}

pub fn shadler_get_api_response(uri: &str) -> String {

    let mut response = ureq::get(uri)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0")
        .header("Referer", "https://allmanga.to")
        .call()
        .unwrap();

    if response.status().is_success() {

        let body = response
            .body_mut()
            .read_to_string()
            .unwrap();

        if body.contains("PERSISTED_QUERY_NOT_FOUND") {
            return String::from("CRITICAL: Hash expired")

        } else {
            return body;

        }

    } else {
        return String::from("ERROR: Failed to make an API request, please try again.");

    }

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
                detail_url: shadler_get_detail_url(content_type, x["_id"].as_str().unwrap())
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
