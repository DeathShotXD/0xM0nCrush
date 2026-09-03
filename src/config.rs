pub const DEFAULT_CONF: &str = "targets.conf";

pub fn load_names_file(path: &str) -> Option<Vec<String>> {
    let content = std::fs::read_to_string(path).ok()?;
    Some(
        content
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(str::to_string)
            .collect(),
    )
}

pub fn parse_names_csv(s: &str) -> Vec<String> {
    s.split([',', ';', ' '])
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(str::to_string)
        .collect()
}

pub fn load_default_or(builtin: &[String]) -> Vec<String> {
    if let Some(names) = load_names_file(DEFAULT_CONF) {
        if !names.is_empty() {
            return names;
        }
    }
    builtin.to_vec()
}
