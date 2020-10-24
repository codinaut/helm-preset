use serde_yaml::Mapping;

fn merge(source: Mapping, _override_map: Mapping) -> Mapping {
    source
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge_with_empty() {
        let source = serde_yaml::from_str::<Mapping>(
            r"
                a: b
            "
        ).unwrap();
        assert_eq!(merge(source.clone(), Mapping::new()), source)
    }
}
