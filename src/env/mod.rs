pub fn check_exist_key() {
    let ref_key: Vec<&str> = include_str!("../../.env.example")
        .split('\n')
        .map(|value| value.split('=').collect::<Vec<&str>>()[0])
        .collect();

    ref_key.into_iter().for_each(|key| {
        if !key.contains('#') && !key.is_empty() {
            dotenv::var(key).unwrap_or_else(|_| panic!("{key} not set"));
        }
    });
}
