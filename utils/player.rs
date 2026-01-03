use std::process::{Command, Stdio, Child};
use std::io::{Error, ErrorKind};

use super::constants;

fn player_die() -> Result<Child, Error> {
    let err_obj = Error::from(ErrorKind::NotFound);
    return Err(err_obj);

}

pub fn shadler_stream_video(platform: &str, player: &str, title: &str, link: &str) {

    if platform == "linux" {

	    let player_process;

        if player == "mpv" {
            player_process = Command::new("mpv")
	                        .args([format!("--force-media-title={title}"), format!("{link}")])
             	    	    .stdout(Stdio::null())
 	                        .stdin(Stdio::null())
            		        .spawn();

        } else if player == "vlc" {
            player_process = Command::new("vlc")
	                        .args([format!("--play-and-exit"), format!("--meta-title={title}"), format!("{link}")])
                 		    .stdout(Stdio::null())
	                        .stdin(Stdio::null())
                		    .spawn();

        } else {
            player_process = player_die();

        }

    	match player_process {
            Ok(_) => println!("\n{}Playing video...{}", constants::GREEN, constants::RESET),
            Err(_) => eprintln!("\n{}No player found on system. Have you installed the player?{}", constants::RED, constants::RESET)

	}

    } else if platform == "android" {

        if player == "android_mpv" {
            Command::new("am")
                .args(["start", "--user", "0", "-a", "android.intent.action.VIEW", "-n", "is.xyz.mpv/.MPVActivity", "-d", link])
                .stdout(Stdio::null())
                .stdin(Stdio::null())
                .spawn()
                .unwrap();

        } else if player == "android_vlc" {
            Command::new("am")
                .args(["start", "--user", "0", "-a", "android.intent.action.VIEW", "-n", "org.videolan.vlc/.gui.video.VideoPlayerActivity", "-d", link, "-e", "'title'", title])
                .stdout(Stdio::null())
                .stdin(Stdio::null())
                .spawn()
                .unwrap();

        } else if player == "android_nextplayer" {
            Command::new("am")
                .args(["start", "--user", "0", "-a", "android.intent.action.VIEW", "-n", "dev.anilbeesetti.nextplayer/.feature.player.PlayerActivity", "-d", link])
                .stdout(Stdio::null())
                .stdin(Stdio::null())
                .spawn()
                .unwrap();

        } else if player == "android_mpvkt" {
            Command::new("am")
                .args(["start", "--user", "0", "-a", "android.intent.action.VIEW", "-n", "live.mehiz.mpvkt/.ui.player.PlayerActivity", "-d", link])
                .stdout(Stdio::null())
                .stdin(Stdio::null())
                .spawn()
                .unwrap();

        }

    }

}
