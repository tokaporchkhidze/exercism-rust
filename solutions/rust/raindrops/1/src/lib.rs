pub fn raindrops(n: u32) -> String {
    let sounds: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .into_iter()
        .filter_map(|(divider, str)| (n % divider == 0).then_some(str))
        .collect();
    if sounds.is_empty() {
        n.to_string()
    } else {
        sounds
    }
}
