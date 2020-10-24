use serde_yaml::{Mapping, Value};

fn merge(mut source: Mapping, override_value: Value) -> Mapping {
    if let Value::Mapping(override_map) = override_value {
        for (k, v) in override_map {
            source.insert(k, v);
        }
    }
    return source;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge_with_empty() {
        let source = serde_yaml::from_str::<Mapping>(
            r"
                a: b
            ",
        )
        .unwrap();
        assert_eq!(
            merge(source.clone(), Value::Mapping(Mapping::new())),
            source
        )
    }

    #[test]
    fn merge_with_replacement() {
        let source = serde_yaml::from_str::<Mapping>(
            r"
                a: b
            ",
        )
        .unwrap();
        let override_map = serde_yaml::from_str::<Mapping>(
            r"
                a: c
            ",
        )
        .unwrap();
        assert_eq!(
            merge(source, Value::Mapping(override_map.clone())),
            override_map
        )
    }
}
