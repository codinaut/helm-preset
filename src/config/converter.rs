use super::{Category, Manifest};
use serde_yaml::Mapping;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {}

#[derive(Debug, Eq, PartialEq)]
pub struct Entry {
    name: String,
    values: Mapping,
}

pub type Configuration = Vec<Vec<Entry>>;

impl Manifest {
    pub fn into_config(self) -> Result<Configuration, Error> {
        Ok(self
            .categories
            .into_iter()
            .filter_map(|category| match category {
                Category::NameOnly(_) => None,
                Category::Explicit(explicit_category) => Some(
                    explicit_category
                        .presets
                        .into_iter()
                        .map(|(name, values)| Entry { name, values })
                        .collect(),
                ),
            })
            .collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maplit::hashmap;
    use serde_yaml::{Mapping, Value::String};
    use std::iter::FromIterator;

    #[test]
    fn empty() {
        let config = Manifest::default().into_config().unwrap();
        assert_eq!(config.len(), 0)
    }

    #[test]
    fn inline() {
        let config = serde_yaml::from_str::<Manifest>(
            r"
                categories:
                  - name: category-1
                    presets:
                      key-1:
                        a: b
            ",
        )
        .unwrap()
        .into_config()
        .unwrap();
        assert_eq!(
            config,
            vec![vec![Entry {
                name: "key-1".to_string(),
                values: Mapping::from_iter(hashmap! {
                    String("a".to_string()) => String("b".to_string()),
                })
            }]]
        )
    }
}
