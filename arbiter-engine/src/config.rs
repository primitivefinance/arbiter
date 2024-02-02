struct Config<C: Deserialize> {
    map: HashMap<String, Vec<C>>,
}

impl<C: Deserialize> Config<C> {
    fn load(path: &str) -> Config<C> {
        let file = std::fs::read_to_string(path).unwrap();
        let map: HashMap<String, C> = serde_json::from_str(&file).unwrap();
        Config { map }
    }
}
