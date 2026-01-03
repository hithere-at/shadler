pub static RESET: &'static str = "\x1B[0m";
pub static MAGENTA: &'static str = "\x1B[1;35m";
pub static YELLOW: &'static str = "\x1B[1;33m";
pub static GREEN: &'static str = "\x1B[1;32m";
pub static RED: &'static str ="\x1B[1;31m";
pub static BLUE: &'static str ="\x1B[1;34m";

pub static ANIME_QUERY_VARS: &'static str = "{%22search%22:{%22query%22:%22#QUERY#%22,%22allowAdult%22:false,%22allowUnknown%22:false},%22limit%22:26,%22page%22:1,%22translationType%22:%22sub%22,%22countryOrigin%22:%22ALL%22}";
pub static ANIME_STREAM_VARS: &'static str = "{%22showId%22:%22#ANIME_ID#%22,%22translationType%22:%22sub%22,%22episodeString%22:%22#EPISODE#%22}";
pub static ANIME_QUERY_HASH: &'static str = "06327bc10dd682e1ee7e07b6db9c16e9ad2fd56c1b769e47513128cd5c9fc77a";
pub static ANIME_STREAM_HASH: &'static str = "5f1a64b73793cc2234a389cf3a8f93ad82de7043017dd551f38f65b89daa65e0";
pub static ANIME_DETAIL_HASH: &'static str = "9d7439c90f203e534ca778c4901f9aa2d3ad42c06243ab2c5e6b79612af32028";

pub static MANGA_QUERY_VARS: &'static str = "{%22search%22:{%22query%22:%22#QUERY#%22,%22isManga%22:true},%22limit%22:26,%22page%22:1,%22translationType%22:%22sub%22,%22countryOrigin%22:%22ALL%22}";
pub static MANGA_READ_VARS: &'static str = "{%22mangaId%22:%22#MANGA_ID#%22,%22translationType%22:%22sub%22,%22chapterString%22:%22#CHAPTER#%22,%22limit%22:10,%22offset%22:0}";
pub static MANGA_QUERY_HASH: &'static str = "3a4b7e9ef62953484a05dd40f35b35b118ad2ff3d5e72d2add79bcaa663271e7";
pub static MANGA_DETAIL_HASH: &'static str = "90024aeae9c1a4d3ace0473871dd1902e47fbcb8781ccbcd8ad81f8bb1f313ee";
pub static MANGA_READ_HASH: &'static str = "4a048654fbac31f11e201ac8bd34d748b514c28d2781b674d057d064282e620e";

pub static DETAIL_VARS: &'static str = "{%22_id%22:%22#ID#%22}";
pub static API_EXT: &'static str = "{%22persistedQuery%22:{%22version%22:1,%22sha256Hash%22:%22#HASH#%22}}";

pub static MANGA_READER_BASE: &'static str = "!<DOCTYPE html>
<html>

    <head>
        <title>#TITLE#</title>
        <meta name='viewport' content='width=device-width, initial-scale=1.0'>
        <style type='text/css'>body {{ background-color: #3B3A39; }}</style>
    </head>

    <body>
        #IMG_TAGS#
    </body>

</html>

";

pub static TERMUX_HTTP_SERVER_BASE: &'static str = "#!/bin/sh
python3 -m http.server -d '#MANGA_PATH#' 10100 > /dev/null 2>&1 &
termux-open http://127.0.0.1:10100/#CHAPTER_START#-#CHAPTER_STOP#.html
";
