use std::io::{Read, Write};
use super::helper::shadler_create_file;

// i could have used threads to have faster download but its too complex for me. as the time of writing this, i just started learning rust a week ago
pub fn shadler_download_file(content_type: &str, content_link: &str, title: &str, filename: &str) -> Result<String, String> {

    let episode_file_info = shadler_create_file(content_type, title, filename);
    let episode_file_path = episode_file_info.1;
    let mut episode_file = episode_file_info.0;

    let response = ureq::head(content_link)
        .call()
        .unwrap();

    let mut content_length: usize = 0;
    let mut download_buffer: Vec<u8> = Vec::new();
    let mut downloaded_bytes = 0; // remember: HTTP 'bytes' is 0 based, the first byte is 0

    if response.status().is_success() {
        content_length = response
            .headers()
            .get("content-length")
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<usize>()
            .unwrap();

    }

    while downloaded_bytes < content_length {

        let next_chunk = downloaded_bytes + (10 * 1024 * 1024) - 1; // this variable is 0 based

        let mut video_response = ureq::get(content_link)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0")
        .header("Range", &format!("bytes={downloaded_bytes}-{next_chunk}"))
        .call()
        .unwrap();

        if video_response.status().is_success() {
            let bytes_read: usize = video_response.body_mut().as_reader().read_to_end(&mut download_buffer).unwrap() as usize; // this is just stupid.

            // if only 0 bytes is read, it means download is completed
            if bytes_read == 0 {
                break;

            } else {
                downloaded_bytes += bytes_read;
                episode_file.write_all(&download_buffer[..bytes_read]).unwrap();

            }

        } else {
            return Err(format!("ERROR: Download failed"));

        }

    }

    return Ok(format!("{episode_file_path}"));

}
