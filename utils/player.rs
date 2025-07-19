use std::process::{Command, Stdio};

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
