use serde_yaml::Mapping;

fn merge(mut source: Mapping, override_map: Mapping) -> Mapping {
    for (k, v) in override_map {
        source.insert(k, v);
    };
    return source
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
        assert_eq!(merge(source.clone(), Mapping::new()), source)
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
        assert_eq!(merge(source, override_map.clone()), override_map)
    }
}
