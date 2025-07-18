use super::constants;
use super::structs;

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
