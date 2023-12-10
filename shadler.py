from json import loads as json_loads
from argparse import ArgumentParser
import urllib.request
import requests

DETAIL_VARS = '{%22_id%22:%22#DEATH#%22}'
API_EXT = '{%22persistedQuery%22:{%22version%22:1,%22sha256Hash%22:%22#HASH#%22}}'

ANIME_QUERY_VARS = '{%22search%22:{%22query%22:%22#QUERY#%22,%22allowAdult%22:false,%22allowUnknown%22:false},%22limit%22:26,%22page%22:1,%22translationType%22:%22sub%22,%22countryOrigin%22:%22ALL%22}'
ANIME_STREAM_VARS = '{%22showId%22:%22#ANIME_ID#%22,%22translationType%22:%22sub%22,%22episodeString%22:%22#EPISODE#%22}'

MANGA_QUERY_VARS = '{%22search%22:{%22query%22:%22#QUERY#%22,%22isManga%22:true},%22limit%22:26,%22page%22:1,%22translationType%22:%22sub%22,%22countryOrigin%22:%22ALL%22}'
MANGA_READ_VARS = '{%22mangaId%22:%22#MANGA_ID#%22,%22translationType%22:%22sub%22,%22chapterString%22:%22#CHAPTER#%22,%22limit%22:10,%22offset%22:0}'

ANIME_QUERY_HASH = "06327bc10dd682e1ee7e07b6db9c16e9ad2fd56c1b769e47513128cd5c9fc77a"
ANIME_STREAM_HASH = "5f1a64b73793cc2234a389cf3a8f93ad82de7043017dd551f38f65b89daa65e0"
ANIME_DETAIL_HASH = "9d7439c90f203e534ca778c4901f9aa2d3ad42c06243ab2c5e6b79612af32028"

MANGA_QUERY_HASH="a27e57ef5de5bae714db701fb7b5cf57e13d57938fc6256f7d5c70a975d11f3d"
MANGA_DETAIL_HASH="a42e1106694628f5e4eaecd8d7ce0c73895a22a3c905c29836e2c220cf26e55f"
MANGA_READ_HASH="121996b57011b69386b65ca8fc9e202046fc20bf68b8c8128de0d0e92a681195"

opt_parser = ArgumentParser(prog="shadler", description="Yet another script to watch anime.")
opt_parser.add_argument("query", nargs="*", type=str, help="Search anime or manga with specified keywords")
opt_parser.add_argument("-c", dest="type", type=str, metavar="anime|manga", help="Specify which type of content to consume", required=False, default="anime")
opt_parser.add_argument("-r", metavar=("<a>", "<b>"), dest="range", type=int, nargs=2, help="Specify episode/chapter range", required=False)
opt_parser.add_argument("-s", dest="stream", metavar="True|False", type=bool, help="Stream anime or read manga online", required=False, default=True)
opt_parser.add_argument("-d", dest="download", metavar="True|False", type=bool, help="Download anime episode or read manga chpaters for offline use", required=False, default=False)
opt_parser.add_argument("-n", dest="nextplayer", metavar="True|False", type=bool, help="Use NextPlayerm to stream anime episode. Only available on Android", required=False, default=False)

opt_args = opt_parser.parse_args()

query = " ".join(opt_args.query)
please_let_me_name_my_var_to_type = opt_args.type
can_i_name_my_var_to_range = opt_args.range
stream = opt_args.stream
download = opt_args.download
nextplayer = opt_args.nextplayer

def get_query_url(content_type: str, search_query: str) -> str:
    search_query = search_query.replace(" ", "%20")

    if content_type == "anime":
        query_obj = ANIME_QUERY_VARS.replace("#QUERY#", search_query)
        ext_obj = API_EXT.replace("#HASH#", ANIME_QUERY_HASH)

    elif content_type == "manga":
        query_obj = MANGA_QUERY_VARS.replace("#QUERY", search_query)
        ext_obj = API_EXT.replace("#HASH", MANGA_QUERY_HASH)

    return 'https://api.allanime.day/api?variables=' + query_obj + '&extensions=' + ext_obj

def get_req_res(url: str): # i have no idea what is
    headers = {"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/112.0", "Referer": "https://allanime.to"}
    request_obj = urllib.request.Request(url, headers=headers)

    return urllib.request.urlopen(request_obj)

def anime_handler() -> None:

    if len(query) == 0:
        local_query = input("Search: ")

    else:
        local_query = query

    query_url = get_query_url("anime", local_query)
    api_resp = get_req_res(query_url).read().decode()
    print(api_resp)

if please_let_me_name_my_var_to_type == "anime":
    anime_handler()

else:
    manga_handler()
