# AllAnime GraphQL API Documentation
This documentation discuss about how the API works for `anime` and `music` part of the AllAnime API. `manga` part soon.

## API endpoint
AllAnime API only have a single endpoint, which is `/allanimeapi` with base url of `https://api.allanime.to`. This will be used to fetch stuff from the website.

## API persisted queries
AllAnime API use GraphQL with APQ (Automatic Persisted Queries), which means query string has a unique identifier encoded with SHA-256. Fortunately, the API have their own hashes to fetch different resource type and it wont regenerate. Read more about APQ [here](https://www.apollographql.com/docs/apollo-server/performance/apq/).

> To persist a query string, Apollo Server must first receive it from a requesting client.
> Consequently, each unique query string must be sent to Apollo Server at least once.
> After any client sends a query string to persist, every client that executes that query can then benefit from APQ.

With that said, here are the list of known hashes:

1. Search anime and get their ID, name, episode, etc.
`b645a686b1988327795e1203867ed24f27c6338b41e5e3412fc1478a8ab6774e`

2. Get streaming sources for the specified anime episode
`0ac09728ee9d556967c1a60bbcf55a9f58b4112006d09a258356aeafe1c33889`

## Fetch data from the API
To fetch data, the endpoint takes two query string:

1. `variables`
This is where you put your URL encoded JSON to fetch data like search results and stream sources.

2. `extensions`
This is where you put your APQ query string to specify what type of operation to do.

  * APQ format
    * `{"persistedQuery":{"version":1,"sha256Hash":"HASH"}}`
    Replace `HASH` with the hash you want to use and URL encode it

Put it all together, for example:
`https://api.allanime.to/allanimeapi?variables={%22search%22:{%22query%22:%22link%20click%22,%22allowAdult%22:false,%22allowUnknown%22:false},%22limit%22:26,%22page%22:1,%22translationType%22:%22sub%22,%22countryOrigin%22:%22ALL%22}&extensions={%22persistedQuery%22:{%22version%22:1,%22sha256Hash%22:%22b645a686b1988327795e1203867ed24f27c6338b41e5e3412fc1478a8ab6774e%22}}`

## Anime

### Fetch search results
Create a JSON object with the key listed below

* `search`
  contain JSON object with the key listed below.

  | Key | Type | Description |
  | ----- | ---- | -----------
  | `query` | `string` | Kewyords to search |
  | `allowAdult` | `boolean` | Allow adult content to be displayed in the search results |
  | `allowUnknown` | `boolean` | sorry idk |

* `limit`
  * Data type: `int`
  * Description: The limit of displayed search result.

* `page`
  * Data type: `int`
  * Description: Display the results only on the spcified page.

* `translationType`
  * Data type: `string`
  * Values: `sub`, `dub`
  * Description: Specify which type of translation to display the search result.

* `countryOrigin`
  * Data type: `string`
  * Values: `ALL` (could be more than this but idk what is the values)
  * Description: Display search results from the specified country.

Return JSON object containing full anime metadata (e.g ID, available episodes, duration, the time the anime aired). Here is the data structure:

```json
{
  "data": {
    "shows": {
      "pageinfo": {
        "total": N
      },
      "edges": [
         {
           "_id": "ID",
           "name": "ROMAJI_NAME"
           "englishName": "EN_NAME"
           ...
         },
        ...
      ]
    }
  }
}
```

### Fetch stream sources
Create a JSON object with the key listed below

* `showId`
  * Data type: `string`
  * Description: AllAnime anime ID (can be acquired by fetching search results).

* `translationType`
  * Data type: `string`
  * Values: `sub`, `dub`
  * Description: Specify which type of translation to get the stream source.

* `episodeString`
  * Data type: `string`
  * Description: Specify which episode to get the stream source.

Return JSON object containing URL's for streaming and the stream source name. Here is the data structure:

```json
{
  "data": {
    "episode": {
      "episodeString": "SELECTED_EP",
      "uploadDate": {
          ...
      },
      "sourceUrls": [
        {
          "sourceUrl": "STREAM_URL"
          "priority": FLOAT_VALUE
          "sourceName": "SOURCE_NAME"
          ...
        },
        ...
      ]
    }
  }
}
```

**NOTE: The higher the priority value are, the higher the chance of the link still working**

## Music

### Fetch anime musics title, artist, audio link, etc.
Create JSON object with the keys listed below

* `search`
  contain a JSON object with the key listed below

   * `query`
     * Data type: `string`
     * Description: Music name to search

* `limit`
  * Data type: `int`
  * Description: The limit of displayed search result.

* `page`
  * Data type: `int`
  * Description: Display the results only on the spcified page.

Return JSON object containing informations anime-related musics. Here is the data structure:

```json
{
  "data": {
    "musics": {
      "edges": [
        {
          "_id": "MUSIC_ID",
          "slug": "URL_ENCODED_MUSIC_NAME",
          "artist": {
            "name": {
              "full": "ARTIST_NAME"
            }
          },
          "musicTitle": {
            "full": "MUSIC_NAME"
          },
          "musicUrls": [
            "url": "MUSIC_URL"
            ...
          ],
          "show": {
            "name": "ANIME_NAME",
            "_id": "ANIME_ID",
            "thumbnail": "ANIME_COVER_LINK"
          },
          "cover": "MUSIC_COVER_LINK"
          "type": "ANIME_EP_OR_ED"
          ...
        },
        ...
      ]
    }
  }
}
```
