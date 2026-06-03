pub fn filter_items(labels: &[String], query: &str) -> Vec<usize> {
    if query.is_empty() {
        return (0..labels.len()).collect();
    }
    let q = query.to_ascii_lowercase();
    labels
        .iter()
        .enumerate()
        .filter(|(_, l)| l.to_ascii_lowercase().contains(&q))
        .map(|(i, _)| i)
        .collect()
}
