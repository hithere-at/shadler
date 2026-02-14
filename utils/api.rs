use std::process::exit;

use super::constants;

pub fn shadler_get_query_url(query_type: &str, query: &str) -> String {

    let good_query = query.replace(" ", "%20");
    let mut query_var = String::new();

    let mut query_url = String::from("https://api.allanime.day/api?query=");

    if query_type == "shows" {
        query_url.push_str(constants::ANIME_QUERY_STRING);
        query_var = constants::ANIME_QUERY_VARS.replace("#QUERY#", &good_query);

    } else if query_type == "mangas" {
        query_url.push_str(constants::MANGA_QUERY_STRING);
        query_var = constants::MANGA_QUERY_VARS.replace("#QUERY#", &good_query);

    }

    query_url.push_str("&variables=");
    query_url.push_str(&query_var);

    return query_url;

}

pub fn shadler_get_detail_url(query_type: &str, id: &str) -> String {

    let detail_var = constants::DETAIL_VARS.replace("#ID#", id);

    let mut detail_url = String::from("https://api.allanime.day/api?query=");

    if query_type == "shows" {
        detail_url.push_str(constants::ANIME_DETAIL_STRING);

    } else if query_type == "mangas" {
        detail_url.push_str(constants::MANGA_DETAIL_STRING);

    }

    detail_url.push_str("&variables=");
    detail_url.push_str(&detail_var);

    return detail_url;

}

pub fn shadler_get_stream_url(detail_type: &str, id: &str, episode: &str) -> String {

    let mut stream_var = String::new();
    println!("{}", episode);

    let mut stream_url = String::from("https://api.allanime.day/api?query=");

    if detail_type == "shows" {
        stream_url.push_str(constants::ANIME_STREAM_STRING);
        stream_var = constants::ANIME_STREAM_VARS.replace("#ANIME_ID#", id).replace("#EPISODE#", episode);

    } else if detail_type == "mangas" {
        stream_url.push_str(constants::MANGA_READ_STRING);
        stream_var = constants::MANGA_READ_VARS.replace("#MANGA_ID#", id).replace("#CHAPTER#", episode);

    }


    stream_url.push_str("&variables=");
    stream_url.push_str(&stream_var);

    return stream_url;

}

pub fn shadler_get_api_response(uri: &str) -> String {

    let response_result = ureq::get(uri)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0")
        .header("Referer", "https://allmanga.to/")
        .call();

    println!("{}", uri);

    let mut response;

    match response_result {
        Ok(val) => { response = val },
        Err(_) => { eprintln!("\n{}ERROR: Failed to send an API request, please try again later.{}", constants::RED, constants::RESET); exit(1) }

    }

    if response.status().is_success() {

        let body = response
            .body_mut()
            .read_to_string()
            .unwrap();

        if body.contains("PERSISTED_QUERY_NOT_FOUND") {
            eprintln!("\n{}CRITICAL: Hash is invalid{}", constants::RED, constants::RESET);
            exit(1);

        } else {
            return body;

        }

    } else {
        eprintln!("\n{}CRITICAL: API request sent but returned non-200 status code. Not good{}", constants::RED, constants::RESET);
        exit(1);

    }

}
