use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Cooldowns {
    pub threads: i32,
    pub replies: i32,
    pub images: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Board {
    /// The directory the board is located in.
    /// Any String
    pub board: String,

    /// The readable title at the top of the board.
    /// Any String
    pub title: String,

    /// Is the board worksafe
    pub ws_board: u8,

    /// How many threads are on a single index page
    /// Any positive integer
    pub per_page: i32,

    /// How many index pages does the board have
    /// Any positive integer
    pub pages: i32,

    /// Maximum file size allowed for non .webm attachments (in KB)
    /// Any positive integer
    pub max_filesize: i32,

    /// Maximum file size allowed for .webm attachments (in KB)
    /// Any positive integer
    pub max_webm_filesize: i32,

    /// Maximum number of characters allowed in a post comment
    /// Any positive integer
    pub max_comment_chars: i32,

    /// Maximum duration of a .webm attachment (in seconds)
    /// Any positive integer
    pub max_webm_duration: i32,

    /// Maximum number of replies allowed to a thread before the thread stops bumping
    /// Any positive integer
    pub bump_limit: i32,

    /// Maximum number of image replies per thread before image replies are discarded
    /// Any positive integer
    pub image_limit: i32,

    pub cooldowns: Cooldowns,

    /// SEO meta description content for a board
    /// Any String
    pub meta_description: String,

    /// Are spoilers enabled
    pub spoilers: Option<u8>,

    /// How many custom spoilers does the board have
    /// Any positive integer
    pub custom_spoilers: Option<i32>,

    /// Are archives enabled for the board
    pub is_archived: Option<u8>,

    /// Array of flag codes mapped to flag names
    pub board_flags: Option<HashMap<String, String>>,

    /// Are flags showing the poster's country enabled on the board
    pub country_flags: Option<u8>,

    /// Are poster ID tags enabled on the board
    pub user_ids: Option<u8>,

    /// Can users submit drawings via browser the Oekaki app
    pub oekaki: Option<u8>,

    /// Can users submit sjis drawings using the [sjis] tags
    pub sjis_tags: Option<u8>,

    /// Board supports code syntax highlighting using the [code] tags
    pub code_tags: Option<u8>,

    /// Board supports [math] TeX and [eqn] tags
    pub math_tags: Option<u8>,

    /// Is image posting disabled for the board
    pub text_only: Option<u8>,

    /// Is the name field disabled on the board
    pub forced_anon: Option<u8>,

    /// Are webms with audio allowed?
    pub webm_audio: Option<u8>,

    /// Do OPs require a subject
    pub require_subject: Option<i32>,

    /// What is the minimum image width (in pixels)
    /// Any positive integer
    pub min_image_width: Option<i32>,

    /// What is the minimum image height (in pixels)
    /// Any positive integer
    pub min_image_height: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct BoardsResponse {
    pub boards: Vec<Board>,
}
