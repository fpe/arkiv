use crate::Post;

#[derive(Debug, Deserialize)]
pub enum ThreadResponse {
    Thread(ThreadResponseInner),
    NotModified,
    NotFound,
}

#[derive(Debug, Deserialize)]
pub struct ThreadResponseInner {
    pub posts: Vec<Post>,
}

#[derive(Debug, Deserialize)]
pub struct ThreadEntry {
    pub no: i64,
    pub last_modified: i64,
    pub replies: i64,
}

#[derive(Debug, Deserialize)]
pub struct ThreadListPage {
    pub page: i64,
    pub threads: Vec<ThreadEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ThreadPageListResponse(pub Vec<ThreadListPage>);
