use std::{collections::HashMap, error::Error, path::PathBuf};

use crate::{
    model::{FileAction, MachineContext, SecretKey},
    repository::outcome::OutcomeError,
};

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

    fn permission_to_string(_: std::fs::Permissions) -> String {
        Default::default()
    }

    fn permission_from_string(_: &str) -> Option<std::fs::Permissions> {
        None
    }
}

#[test]
fn track_regular_files() -> Result<(), Box<dyn Error>> {
    let tmp_repo = tempfile::tempdir()?;
    let secret_key = SecretKey::generate();
    let context = MachineContext {
        recipient: secret_key.as_recipient("test"),
        variables: HashMap::new(),
    };

    let mut repository = Repository::<TestEnvironment>::init(
        tmp_repo.path().to_path_buf(),
        secret_key.as_recipient("Test"),
    )?;

    repository.add_files(
        FileAction::AsIs,
        vec![PathBuf::from(".config/someapp/config")],
    )?;
    repository.store()?;

    let secret_keys = &[secret_key];
    let outcomes = repository
        .files()
        .map(|f| f.outcome(&context, secret_keys))
        .collect::<Result<Vec<Outcome<_>>, OutcomeError>>()?;

    assert_eq!(outcomes.len(), 1);

    assert_eq!(
        outcomes[0].target,
        TestEnvironment::home_dir()?.join(".config/someapp/config")
    );
    assert_eq!(
        outcomes[0].content,
        b"This is some\nfancy \nconfig file\nthat\ndoes not\nrequire\nprotection\n"
    );

    Ok(())
}

#[test]
fn track_secret_files() -> Result<(), Box<dyn Error>> {
    let tmp_repo = tempfile::tempdir()?;
    let secret_key = SecretKey::generate();
    let context = MachineContext {
        recipient: secret_key.as_recipient("test"),
        variables: HashMap::new(),
    };

    let mut repository = Repository::<TestEnvironment>::init(
        tmp_repo.path().to_path_buf(),
        secret_key.as_recipient("Test"),
    )?;

    repository.add_files(
        FileAction::Crypted,
        vec![PathBuf::from(".config/someotherapp/secret_config")],
    )?;
    repository.store()?;

    let secret_keys = &[secret_key];
    let outcomes = repository
        .files()
        .map(|f| f.outcome(&context, secret_keys))
        .collect::<Result<Vec<Outcome<_>>, OutcomeError>>()?;

    assert_eq!(outcomes.len(), 1);

    assert_eq!(
        outcomes[0].target,
        TestEnvironment::home_dir()?.join(".config/someotherapp/secret_config")
    );
    assert_eq!(
        outcomes[0].content,
        b"This is\na very\nsecret config\nthat has to be\nfully protected.\nFailing to do so\nwill reveal\nhow the moon landing was staged and\nthat the earth is indeed flat.\n"
    );

    Ok(())
}
