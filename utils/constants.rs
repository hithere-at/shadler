pub static RESET: &'static str = "\x1B[0m";
pub static MAGENTA: &'static str = "\x1B[1;35m";
pub static YELLOW: &'static str = "\x1B[1;33m";
pub static GREEN: &'static str = "\x1B[1;32m";
pub static RED: &'static str ="\x1B[1;31m";
pub static BLUE: &'static str ="\x1B[1;34m";

pub static ANIME_QUERY_STRING: &'static str = "query%28%24search%3A%20SearchInput%29%20%7B%20shows%28search%3A%20%24search%29%20%7B%20edges%20%7B%20_id%20name%20%7D%20%7D%20%7D";
pub static ANIME_QUERY_VARS: &'static str = "%7B%22search%22%3A%7B%22query%22%3A%22#QUERY#%22%7D%7D";
pub static ANIME_STREAM_STRING: &'static str = "query%28%24showId%3A%20String%21%20%24episodeString%3A%20String%21%29%20%7B%20episode%28showId%3A%20%24showId%20translationType%3A%20sub%20episodeString%3A%20%24episodeString%29%20%7B%20sourceUrls%20%7D%20%7D";
pub static ANIME_STREAM_VARS: &'static str = "%7B%22showId%22%3A%22#ANIME_ID#%22%2C%22episodeString%22%3A%22#EPISODE#%22%7D";

pub static MANGA_QUERY_STRING: &'static str = "query%28%24search%3A%20SearchInput%29%20%7B%20mangas%28search%3A%20%24search%29%20%7B%20edges%20%7B%20_id%20name%20%7D%20%7D%20%7D";
pub static MANGA_QUERY_VARS: &'static str = "%7B%22search%22%3A%7B%22query%22%3A%22#QUERY#%22%2C%22isManga%22%3Atrue%7D%7D";
pub static MANGA_READ_STRING: &'static str = "query%28%24mangaId%3A%20String%21%20%24chapterString%3A%20String%21%29%20%7B%20chapterPages%28mangaId%3A%20%24mangaId%20translationType%3A%20sub%20chapterString%3A%20%24chapterString%29%20%7B%20edges%20%7B%20pictureUrls%20pictureUrlHead%20%7D%20%7D%20%7D";
pub static MANGA_READ_VARS: &'static str = "%7B%22mangaId%22%3A%22#MANGA_ID#%22%2C%22chapterString%22%3A%22#CHAPTER#%22%7D";

pub static ANIME_DETAIL_STRING: &'static str = "query%28%24_id%3A%20String%21%29%20%7B%20show%28_id%3A%20%24_id%29%20%7B%20availableEpisodesDetail%20%7D%20%7D";
pub static MANGA_DETAIL_STRING: &'static str = "query%28%24_id%3A%20String%21%29%20%7B%20manga%28_id%3A%20%24_id%29%20%7B%20availableChaptersDetail%20%7D%20%7D";
pub static DETAIL_VARS: &'static str = "%7B%22_id%22%3A%22#ID#%22%7D";

pub static MANGA_READER_BASE: &'static str = "<!DOCTYPE html>
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
kill -15 \"$(cat /data/data/com.termux/files/usr/tmp/shadler_server_lock 2> /dev/null)\" 2> /dev/null
python3 -m http.server -d '#MANGA_PATH#' 10100 > /dev/null 2>&1 &
echo \"$!\" > /data/data/com.termux/files/usr/tmp/shadler_server_lock
termux-open http://127.0.0.1:10100/#CHAPTER_START#-#CHAPTER_STOP#.html
";
