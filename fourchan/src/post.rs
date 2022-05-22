use serde::Deserialize;

fn default_name() -> String {
    "Anonymous".to_string()
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Post {
    /// The numeric post ID
    ///
    /// Type `integer`
    ///
    /// Appears `always`
    ///
    /// Possible values: `any positive integer`
    pub no: i64,

    /// For replies: this is the ID of the thread being replied to. For OP: this value is zero
    ///
    /// Type `integer`
    ///
    /// Appears `always`
    ///
    /// Possible values: `0` or `any positive integer`
    pub resto: i64,

    /// If the thread is being pinned to the top of the page
    ///
    /// Type `integer`
    ///
    /// Appears `OP only, if thread is currently stickied`
    ///
    /// Possible values: `1` or not set
    #[serde(default)]
    pub sticky: i64,

    /// If the thread is closed to replies
    ///
    /// Type `integer`
    ///
    /// Appears `OP only, if thread is currently closed`
    ///
    /// Possible values: `1` or not set
    #[serde(default)]
    pub closed: i64,

    /// MM/DD/YY(Day)HH: M (: S on some boards), EST/EDT timezone
    ///
    /// Type `string`
    ///
    /// Appears `always`
    ///
    /// Possible values: `string`
    pub now: String,

    /// UNIX timestamp the post was created
    ///
    /// Type `integer`
    ///
    /// Appears `always`
    ///
    /// Possible values: `UNIX timestamp`
    pub time: i64,

    /// Name user posted with. Defaults to `Anonymous`
    ///
    /// Type `string`
    ///
    /// Appears `always`
    ///
    /// Possible values: `any string`
    #[serde(default = "default_name")]
    pub name: String,

    /// The user's tripcode, in format: `!tripcode` or `!!securetripcode`
    ///
    /// Type `string`
    ///
    /// Appears `if post has tripcode`
    ///
    /// Possible values: `any string`
    pub trip: Option<String>,

    /// The poster's ID
    ///
    /// Type `string`
    ///
    /// Appears `if post has ID`
    ///
    /// Possible values: `any 8 characters`
    pub id: Option<String>,

    /// The capcode identifier for a post
    ///
    /// Type `string`
    ///
    /// Appears `if post has capcode`
    ///
    /// Possible values: Not set, `mod`, `admin`, `admin_highlight`, `manager`, `developer`, `founder`
    pub capcode: Option<String>,

    /// Poster's [ISO 3166-1 alpha-2 country code](https: /en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
    ///
    /// Type `string`
    ///
    /// Appears `if country flags are enabled`
    ///
    /// Possible values: `2 character string` or `XX` if unknown
    pub country: Option<String>,

    /// Poster's country name
    ///
    /// Type `string`
    ///
    /// Appears `if country flags are enabled`
    ///
    /// Possible values: `Name of any country`
    pub country_name: Option<String>,

    /// Poster's board flag code
    ///
    /// Type `string`
    ///
    /// Appears `if board flags are enabled`
    ///
    /// Possible values:
    pub board_flag: Option<String>,

    /// Poster's board flag name
    ///
    /// Type `string`
    ///
    /// Appears `if board flags are enabled`
    ///
    /// Possible values: `Name of a board flag`
    pub flag_name: Option<String>,

    /// OP Subject text
    ///
    /// Type `string`
    ///
    /// Appears `OP only, if subject was included`
    ///
    /// Possible values: `any string`
    pub sub: Option<String>,

    /// Comment (HTML escaped)
    ///
    /// Type `string`
    ///
    /// Appears `if comment was included`
    ///
    /// Possible values: `any HTML escaped string`
    pub com: Option<String>,

    /// Unix timestamp + microtime that an image was uploaded
    ///
    /// Type `integer`
    ///
    /// Appears `always if post has attachment`
    ///
    /// Possible values: `integer`
    pub tim: Option<i64>,

    /// Filename as it appeared on the poster's device
    ///
    /// Type `string`
    ///
    /// Appears `always if post has attachment`
    ///
    /// Possible values: `any string`
    pub filename: Option<String>,

    /// Filetype
    ///
    /// Type `string`
    ///
    /// Appears `always if post has attachment`
    ///
    /// Possible values: `.jpg`, `.png`, `.gif`, `.pdf`, `.swf`, `.webm`
    pub ext: Option<String>,

    /// Size of uploaded file in bytes
    ///
    /// Type `integer`
    ///
    /// Appears `always if post has attachment`
    ///
    /// Possible values: `any positive integer`
    pub fsize: Option<i64>,

    /// 24 character, packed base64 MD5 hash of file
    ///
    /// Type `string`
    ///
    /// Appears `always if post has attachment`
    ///
    /// Possible values:
    pub md5: Option<String>,

    /// Image width dimension
    ///
    /// Type `integer`
    ///
    /// Appears `always if post has attachment`
    ///
    /// Possible values: `any positive integer`
    pub w: Option<i64>,

    /// Image height dimension
    ///
    /// Type `integer`
    ///
    /// Appears `always if post has attachment`
    ///
    /// Possible values: `any positive integer`
    pub h: Option<i64>,

    /// Thumbnail image width dimension
    ///
    /// Type `integer`
    ///
    /// Appears `always if post has attachment`
    ///
    /// Possible values: `any positive integer`
    pub tn_w: Option<i64>,

    /// Thumbnail image height dimension
    ///
    /// Type `integer`
    ///
    /// Appears `always if post has attachment`
    ///
    /// Possible values: `any positive integer`
    pub tn_h: Option<i64>,

    /// If the file was deleted from the post
    ///
    /// Type `integer`
    ///
    /// Appears `if post had attachment and attachment is deleted`
    ///
    /// Possible values: `1` or not set
    #[serde(default)]
    pub filedeleted: i64,

    /// If the image was spoilered or not
    ///
    /// Type `integer`
    ///
    /// Appears `if post has attachment and attachment is spoilered`
    ///
    /// Possible values: `1` or not set
    #[serde(default)]
    pub spoiler: i64,

    /// `1-10` or not set
    ///
    /// Type `if post has attachment and attachment is spoilered`
    ///
    /// Appears The custom spoiler ID for a spoilered image
    ///
    /// Possible values:
    pub custom_spoiler: Option<i64>,

    /// Total number of replies to a thread
    ///
    /// Type `integer`
    ///
    /// Appears `OP only`
    ///
    /// Possible values: `0` or `any positive integer`
    pub replies: Option<i64>,

    /// Total number of image replies to a thread
    ///
    /// Type `integer`
    ///
    /// Appears `OP only`
    ///
    /// Possible values: `0` or `any positive integer`
    pub images: Option<i64>,

    /// If a thread has reached bumplimit, it will no longer bump
    ///
    /// Type `integer`
    ///
    /// Appears `OP only, only if bump limit has been reached`
    ///
    /// Possible values: `1` or not set
    #[serde(default)]
    pub bumplimit: i64,

    /// If an image has reached image limit, no more image replies can be made
    ///
    /// Type `integer`
    ///
    /// Appears `OP only, only if image limit has been reached`
    ///
    /// Possible values: `1` or not set
    #[serde(default)]
    pub imagelimit: i64,

    /// The category of `.swf` upload
    ///
    /// Type `string`
    ///
    /// Appears `OP only`, `/f/ only`
    ///
    /// Possible values: `Game`, `Loop`, etc..
    pub tag: Option<String>,

    /// SEO URL slug for thread
    ///
    /// Type `string`
    ///
    /// Appears `OP only`
    ///
    /// Possible values: `string`
    pub semantic_url: Option<String>,

    /// Year 4chan pass bought
    ///
    /// Type `integer`
    ///
    /// Appears `if poster put 'since4pass' in the options field`
    ///
    /// Possible values: `any 4 digit year`
    pub since4pass: Option<i64>,

    /// Number of unique posters in a thread
    ///
    /// Type `integer`
    ///
    /// Appears `OP only, only if thread has NOT been archived`
    ///
    /// Possible values: `any positive integer`
    pub unique_ips: Option<i64>,

    /// Mobile optimized image exists for post
    ///
    /// Type `integer`
    ///
    /// Appears `any post that has a mobile-optimized image`
    ///
    /// Possible values: `1` or not set
    #[serde(default)]
    pub m_img: i64,

    /// Thread has reached the board's archive
    ///
    /// Type `integer`
    ///
    /// Appears `OP only, if thread has been archived`
    ///
    /// Possible values: `1` or not set
    #[serde(default)]
    pub archived: i64,

    /// UNIX timestamp the post was archived
    ///
    /// Type `integer`
    ///
    /// Appears `OP only, if thread has been archived`
    ///
    /// Possible values: `UNIX timestamp`
    pub archived_on: Option<i64>,

    #[serde(skip_deserializing)]
    pub board: String,
}

impl Post {
    #[must_use]
    pub fn attachment(&self) -> Option<PostAttachment> {
        if let Some(tim) = self.tim
            && let Some(filename) = self.filename.clone()
            && let Some(ext) = self.ext.clone()
            && let Some(fsize) = self.fsize
            && let Some(md5) = self.md5.clone()
            && let Some(w) = self.w
            && let Some(h) = self.h
            && let Some(tn_w) = self.tn_w
            && let Some(tn_h) = self.tn_h
        {
            Some(PostAttachment {
                tim,
                filename,
                ext,
                fsize,
                md5,
                w,
                h,
                tn_w,
                tn_h,
                filedeleted: self.filedeleted,
                spoiler: self.spoiler,
                custom_spoiler: self.custom_spoiler,
            })
        } else {
            None
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct PostAttachment {
    pub tim: i64,
    pub filename: String,
    pub ext: String,
    pub fsize: i64,
    pub md5: String,
    pub w: i64,
    pub h: i64,
    pub tn_w: i64,
    pub tn_h: i64,
    pub filedeleted: i64,
    pub spoiler: i64,
    pub custom_spoiler: Option<i64>,
}
