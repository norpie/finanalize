use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use uuid::Uuid;

use crate::{db::DB, prelude::*};

use std::{env, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDBPersistedBlob {
    pub id: Thing,
    pub path: PathBuf,
}

impl From<SDBPersistedBlob> for PersistedBlob {
    fn from(blob: SDBPersistedBlob) -> Self {
        PersistedBlob {
            id: blob.id.id.to_string(),
            path: blob.path,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedBlob {
    pub id: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersistedBlob {
    pub path: PathBuf,
}

pub fn get_persistance_dir() -> Result<PathBuf> {
    let persistance_dir_path = env::var("PERSISTANCE_DIR")?;
    let path_buf = PathBuf::from(persistance_dir_path);
    if !path_buf.exists() {
        std::fs::create_dir_all(&path_buf)?;
    }
    Ok(path_buf)
}

pub async fn persist(file: PathBuf) -> Result<PersistedBlob> {
    let uuid = Uuid::new_v4();
    let persistance_dir = get_persistance_dir()?;
    let new_dir = persistance_dir.join(uuid.to_string());
    std::fs::create_dir(&new_dir)?;
    let new_file = new_dir.join(file.file_name().unwrap());
    std::fs::copy(&file, &new_file)?;
    let created: PersistedBlob = DB
        .get()
        .unwrap()
        .create("blob")
        .content(CreatePersistedBlob { path: new_file })
        .await?
        .map(|sdb: SDBPersistedBlob| sdb.into())
        .ok_or(FinanalizeError::NotFound)?;
    Ok(created)
}

pub async fn retrieve(id: &str) -> Result<PersistedBlob> {
    let sdb: Option<SDBPersistedBlob> = DB.get().unwrap().select(("blob", id)).await?;
    let Some(sdb) = sdb else {
        return Err(FinanalizeError::NotFound);
    };
    Ok(sdb.into())
}

#[cfg(test)]
mod tests {
    use crate::db;

    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[tokio::test]
    #[ignore = "Depends on external state"]
    async fn test_persist() {
        env_logger::init();
        db::init().await.unwrap();
        env::set_var("PERSISTANCE_DIR", "/tmp/persists");
        let file_path = PathBuf::from("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, world!").unwrap();
        let blob = persist(file_path).await.unwrap();
        assert!(blob.path.exists());
        dbg!(&blob);
    }

    #[tokio::test]
    #[ignore = "Depends on external state"]
    async fn test_retrieve() {
        env_logger::init();
        db::init().await.unwrap();
        env::set_var("PERSISTANCE_DIR", "/tmp/persists");
        let file_path = PathBuf::from("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, world!").unwrap();
        let blob = persist(file_path).await.unwrap();
        println!("Blob: {:?}", blob);
        let retrieved = retrieve(&blob.id).await.unwrap();
        assert_eq!(retrieved.id, blob.id);
        assert!(retrieved.path.exists());
        dbg!(&retrieved);
    }
}
