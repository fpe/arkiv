pub mod local;

use std::path::PathBuf;

#[async_trait]
pub trait Storage
where
    Self: Clone + Send + Sync + 'static,
{
    async fn save(&self, key: &str, subdir: Option<&str>, body: &[u8]) -> anyhow::Result<()>;
    async fn get(&self, key: &str, subdir: Option<&str>) -> anyhow::Result<Vec<u8>>;
    async fn exists(&self, key: &str, subdir: Option<&str>) -> anyhow::Result<bool>;
}

#[must_use]
pub fn split_path(key: &str) -> PathBuf {
    let pathbuf = key
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .take(4)
        .map(|ca| ca.iter().collect::<String>())
        .fold(PathBuf::new(), |mut path, dir| {
            path.push(dir);
            path
        });
    pathbuf
}

#[test]
fn test_path_split() {
    assert_eq!(PathBuf::from("12/3"), split_path("123"));
    assert_eq!(PathBuf::from("12/34/56/78"), split_path("1234567890"));
}
