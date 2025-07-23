pub struct QueryContent {
    pub id: String,
    pub title: String,
    pub detail_url: String

}

pub struct StreamContent {
    pub id: String,
    pub title: String,
    pub selected: Vec<i32>,
    pub available: Vec<String>,
    pub player: String

}

pub struct CommandArguments {
    pub query: String,
    pub action: i32, // 1 for streaming, 2 for downloading
    pub player: String,
    pub range: Vec<i32>

}
