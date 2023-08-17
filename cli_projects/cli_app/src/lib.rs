pub fn match_pattern(
    pattern: &str,
    content: &str,
    mut output: impl std::io::Write,
) -> Result<(), std::io::Error> {
    for (counter, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            writeln!(&mut output, "{:},{:}", counter, line)?
        }
    }
    Ok(())
}
