use reqwest::header::{USER_AGENT, REFERER};
use serde_json::Value; 

use super::constants;
use super::structs;

pub fn shadler_get_query_url(query_type: &str, query: &str) -> String {

    let good_query = query.replace(" ", "%20");
    let mut query_var = String::new();
    let mut ext_var = String::new();

    if query_type == "shows" {
        query_var = constants::ANIME_QUERY_VARS.replace("#QUERY#", good_query.as_str());
        ext_var = constants::API_EXT.replace("#HASH#", constants::ANIME_QUERY_HASH);

    } else if query_type == "mangas" {
        query_var = constants::MANGA_QUERY_VARS.replace("#QUERY#", good_query.as_str());
        ext_var = constants::API_EXT.replace("#HASH#", constants::MANGA_QUERY_HASH);

    }

    let mut query_url = String::from("https://api.allanime.day/api?variables=");
    query_url.push_str(query_var.as_str());
    query_url.push_str("&extensions=");
    query_url.push_str(ext_var.as_str());

    return query_url;

}

pub fn shadler_get_detail_url(detail_type: &str, id: &str) -> String {

    let mut ext_var = String::new();
    let query_var = constants::DETAIL_VARS.replace("#ID#", id);

    if detail_type == "shows" {
        ext_var = constants::API_EXT.replace("#HASH#", constants::ANIME_DETAIL_HASH);

    } else if detail_type == "mangas" {
        ext_var = constants::API_EXT.replace("#HASH#", constants::MANGA_DETAIL_HASH);

    }

    let mut query_url = String::from("https://api.allanime.day/api?variables=");
    query_url.push_str(query_var.as_str());
    query_url.push_str("&extensions=");
    query_url.push_str(ext_var.as_str());

    return query_url;

}

pub fn shadler_get_api_response(uri: &String) -> String {

    let request = reqwest::blocking::Client::new();
    let response = request.get(uri)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0")
        .header(REFERER, "https://allmanga.to")
        .send()
        .unwrap();

    if response.status().is_success() {

        let body = response.text().unwrap();

        if body.contains("PERSISTED_QUERY_NOT_FOUND") {
            return String::from("CRITICAL: Hash expired")

        } else {
            return body;

        }

    } else {
        return String::from("ERROR: Failed to make an API request, please try again.");

    }

}

pub fn shadler_get_query_object(content_type: &str, resp: &String) -> Result<Vec<structs::QueryContent>, String> {

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
