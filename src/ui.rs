pub fn bar(percent: f32) -> String {
    let width = 20;

    let filled = ((percent / 100.0) * width as f32) as usize;

    format!(
        "{}{}",
        "█".repeat(filled),
        "░".repeat(width - filled)
    )
}
