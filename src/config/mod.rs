pub mod manifest;

use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
}

pub struct Entry {
}

pub type Configuration = Vec<Vec<Entry>>;

impl manifest::Manifest {
    pub fn into_config(self) -> Result<Configuration, Error> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_presets() {
        let config = serde_yaml::from_str::<manifest::Manifest>(&format!(
            r#"
            categories:
              - a
              - b
            "#,
        ))
        .unwrap()
        .into_config()
        .unwrap();
        assert_eq!(config.len(), 0)
    }
}
