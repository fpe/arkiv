use std::path::PathBuf;

use tokio::io::AsyncWriteExt;
use tracing::{debug, debug_span};

use crate::storage::{split_path, Storage};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct LocalStorage {
    /// Path at which the files will be stored
    path: PathBuf,
}

impl LocalStorage {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }
}

#[async_trait]
impl Storage for LocalStorage {
    async fn save(&self, key: &str, subdir: Option<&str>, body: &[u8]) -> anyhow::Result<()> {
        let _span = debug_span!("localstorage");

        debug!("saving file {:?}", (&key, &subdir));
        let subdirs = split_path(key);

        let mut root_path = self.path.clone();
        if let Some(subdir) = &subdir {
            root_path.push(subdir);
        }
        root_path.push(&subdirs);
        tokio::fs::create_dir_all(&root_path).await?;
        root_path.push(&key);

        let mut file = tokio::fs::File::create(root_path).await?;
        file.write_all(body).await?;
        debug!("saved file {:?}", (&key, &subdir));

        Ok(())
    }
    async fn get(&self, key: &str, subdir: Option<&str>) -> anyhow::Result<Vec<u8>> {
        let subdirs = split_path(key);

        let mut root_path = self.path.clone();
        if let Some(subdir) = subdir {
            root_path.push(subdir);
        }
        root_path.push(subdirs);
        root_path.push(key);

        let body = tokio::fs::read(root_path).await?;

        Ok(body)
    }
    async fn exists(&self, key: &str, subdir: Option<&str>) -> anyhow::Result<bool> {
        let subdirs = split_path(key);

        let mut root_path = self.path.clone();
        if let Some(subdir) = subdir {
            root_path.push(subdir);
        }
        root_path.push(subdirs);
        root_path.push(key);

        Ok(root_path.exists())
    }
}
