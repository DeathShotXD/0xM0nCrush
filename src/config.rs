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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_splits_on_common_separators() {
        let names = parse_names_csv("a.exe, b.exe;c.exe  d.exe");
        assert_eq!(names, vec!["a.exe", "b.exe", "c.exe", "d.exe"]);
    }

    #[test]
    fn csv_drops_empty_parts() {
        let names = parse_names_csv(" , ; ,");
        assert!(names.is_empty());
    }

    #[test]
    fn csv_trims_whitespace() {
        let names = parse_names_csv("  spaced.exe  ");
        assert_eq!(names, vec!["spaced.exe"]);
    }

    #[test]
    fn file_skips_blanks_and_comments() {
        let dir = std::env::temp_dir().join("moncrush_test_conf");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("targets.conf");
        std::fs::write(&path, "# comment\n\nfoo.exe\n  bar.exe  \n").unwrap();
        let names = load_names_file(path.to_str().unwrap()).unwrap();
        assert_eq!(names, vec!["foo.exe", "bar.exe"]);
    }

    #[test]
    fn missing_file_returns_none() {
        assert!(load_names_file("/nonexistent/moncrush/targets.conf").is_none());
    }
}
