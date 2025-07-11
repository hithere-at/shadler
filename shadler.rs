use std::io;
use std::io::Write;

static RESET: &'static str = "\x1B[0m";
static MAGENTA: &'static str = "\x1B[1;35m";
static YELLOW: &'static str = "\x1B[1;33m";
static GREEN: &'static str = "\x1B[1;32m";
static RED: &'static str ="\x1B[1;31m";
static BLUE: &'static str ="\x1B[1;34m";

static ANIME_QUERY_VARS: &'static str = "{%22search%22:{%22query%22:%22#QUERY#%22,%22allowAdult%22:false,%22allowUnknown%22:false},%22limit%22:26,%22page%22:1,%22translationType%22:%22sub%22,%22countryOrigin%22:%22ALL%22}";
static ANIME_STREAM_VARS: &'static str = "{%22showId%22:%22#ANIME_ID#%22,%22translationType%22:%22sub%22,%22episodeString%22:%22#EPISODE#%22}";
static ANIME_QUERY_HASH: &'static str = "06327bc10dd682e1ee7e07b6db9c16e9ad2fd56c1b769e47513128cd5c9fc77a";
static ANIME_STREAM_HASH: &'static str = "5f1a64b73793cc2234a389cf3a8f93ad82de7043017dd551f38f65b89daa65e0";
static ANIME_DETAIL_HASH: &'static str = "9d7439c90f203e534ca778c4901f9aa2d3ad42c06243ab2c5e6b79612af32028";

static MANGA_QUERY_VARS: &'static str = "{%22search%22:{%22query%22:%22#QUERY#%22,%22isManga%22:true},%22limit%22:26,%22page%22:1,%22translationType%22:%22sub%22,%22countryOrigin%22:%22ALL%22}";
static MANGA_READ_VARS: &'static str = "{%22mangaId%22:%22#MANGA_ID#%22,%22translationType%22:%22sub%22,%22chapterString%22:%22#CHAPTER#%22,%22limit%22:10,%22offset%22:0}";
static MANGA_QUERY_HASH: &'static str = "a27e57ef5de5bae714db701fb7b5cf57e13d57938fc6256f7d5c70a975d11f3d";
static MANGA_DETAIL_HASH: &'static str = "529b0770601c7e04c98566c7b7bb3f75178930ae18b3084592d8af2b591a009f";
static MANGA_READ_HASH: &'static str = "121996b57011b69386b65ca8fc9e202046fc20bf68b8c8128de0d0e92a681195";

static DETAIL_VARS: &'static str = "{%22_id%22:%22#HASH#%22}";
static API_EXT: &'static str = "{%22persistedQuery%22:{%22version%22:1,%22sha256Hash%22:%22#HASH#%22}}";

fn shadler_help() {
    let _help = "
Usage: shadler <subcommand> [options]

Example: shadler anime -v -r 8 -s -q 'oshi no ko'
         shadler manga -r 10 12 -s -q 'kaoru hana wa rin to saku'

Options:

    -q | --query <keywords>         Search anime or manga with specified keywords. Please quote the keywords
    -v | --vlc                      Stream using VLC media player
    -n | --nextplay                 Stream using NextPlayer. Available only on Android
    -s | --stream                   Stream anime episode or read manga chapters with online
    -d | --download                 Download anime episode or download manga chapters for offline reading
    -r | --range <lower> <upper>    Specify episode/chapter range
";

   println!("{}", _help);
}

fn shadler_get_query_url(query_type: &str, query: &str) -> String {
    let good_query = query.replace(" ", "%20");
    let mut query_var = String::new();
    let mut ext_var = String::new();

    if query_type == "anime" {
        query_var = ANIME_QUERY_VARS.replace("#QUERY#", good_query.as_str());
        ext_var = API_EXT.replace("#HASH#", ANIME_QUERY_HASH);

    } else if query_type == "manga" {
        query_var = MANGA_QUERY_VARS.replace("#QUERY#", good_query.as_str());
        ext_var = API_EXT.replace("#HASH#", MANGA_QUERY_HASH);

    }

    let mut query_url = String::from("https://api.allanime.day/api?variables=");
    query_url.push_str(query_var.as_str());
    query_url.push_str("&extensions=");
    query_url.push_str(ext_var.as_str());

    return query_url;
}

fn shadler_anime() {
    print!("{}Query:{} ", MAGENTA, RESET);
    
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let query = input.trim();

    let query_url = shadler_get_query_url("anime", query);
    println!("{}", query_url);

}

fn main() {
    
    shadler_anime();

}
