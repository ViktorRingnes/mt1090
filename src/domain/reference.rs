use uuid::Uuid;

pub fn generate_reference() -> String {
    Uuid::now_v7().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn references_are_unique_v7_uuids() {
        let refs: HashSet<_> = (0..1000).map(|_| generate_reference()).collect();
        assert_eq!(refs.len(), 1000);
        for r in &refs {
            assert_eq!(Uuid::parse_str(r).unwrap().get_version_num(), 7);
        }
    }
}
