use std::{error::Error, path::PathBuf};

use crate::{model::FileAction, secret_key::SecretKey};

use super::{Environment, Outcome, Repository};

struct TestEnvironment {}

impl Environment for TestEnvironment {
    fn home_dir() -> Result<std::path::PathBuf, Box<dyn Error>> {
        Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join("home"))
    }

    fn config_dir() -> Result<std::path::PathBuf, Box<dyn Error>> {
        Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join("home")
            .join(".config")
            .join("dotium"))
    }
}

#[test]
fn track_regular_files() -> Result<(), Box<dyn Error>> {
    let tmp_repo = tempfile::tempdir()?;
    let secret_key = SecretKey::generate();

    let mut repository = Repository::<TestEnvironment>::init(
        tmp_repo.path().to_path_buf(),
        secret_key.as_recipient("Test"),
    )?;

    repository.add_files(
        FileAction::AsIs,
        vec![PathBuf::from(".config/someapp/config")],
    )?;
    repository.store()?;

    let outcomes = repository
        .outcomes(&[secret_key])?
        .collect::<Result<Vec<Outcome>, Box<dyn Error>>>()?;

    assert_eq!(outcomes.len(), 1);

    assert_eq!(
        outcomes[0].target,
        TestEnvironment::home_dir()?.join(".config/someapp/config")
    );
    assert_eq!(
        outcomes[0].content,
        "This is some\nfancy \nconfig file\nthat\ndoes not\nrequire\nprotection\n"
    );

    Ok(())
}

#[test]
fn track_secret_files() -> Result<(), Box<dyn Error>> {
    let tmp_repo = tempfile::tempdir()?;
    let secret_key = SecretKey::generate();

    let mut repository = Repository::<TestEnvironment>::init(
        tmp_repo.path().to_path_buf(),
        secret_key.as_recipient("Test"),
    )?;

    repository.add_files(
        FileAction::Crypted,
        vec![PathBuf::from(".config/someotherapp/secret_config")],
    )?;
    repository.store()?;

    let outcomes = repository
        .outcomes(&[secret_key])?
        .collect::<Result<Vec<Outcome>, Box<dyn Error>>>()?;

    assert_eq!(outcomes.len(), 1);

    assert_eq!(
        outcomes[0].target,
        TestEnvironment::home_dir()?.join(".config/someotherapp/secret_config")
    );
    assert_eq!(
        outcomes[0].content,
        "This is\na very\nsecret config\nthat has to be\nfully protected.\nFailing to do so\nwill reveal\nhow the moon landing was staged and\nthat the earth is indeed flat.\n"
    );

    Ok(())
}