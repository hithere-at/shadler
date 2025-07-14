pub struct QueryContent {
    pub id: String,
    pub title: String,
    pub detail_url: String

}

pub struct StreamContent {
    pub id: String,
    pub title: String,
    pub selected: Vec<i32>,
    pub available: Vec<String>

}
