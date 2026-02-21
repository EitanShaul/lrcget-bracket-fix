use collapse::collapse;
use regex::Regex;
use secular::lower_lay_string;

pub fn prepare_input(input: &str) -> String {
    let mut prepared_input = lower_lay_string(&input);

    let re = Regex::new(r#"[`~!@#$%^&*()_|+\-=?;:",.<>\{\}\[\]\\\/]"#).unwrap();
    prepared_input = re.replace_all(&prepared_input, " ").to_string();

    let re = Regex::new(r#"['’]"#).unwrap();
    prepared_input = re.replace_all(&prepared_input, "").to_string();

    prepared_input = prepared_input.to_lowercase();
    prepared_input = collapse(&prepared_input);

    prepared_input
}

/// Strips trailing bracketed text from a string.
/// Removes content in (), [], and {} at the end of the string.
/// For example: "Song Name [Explicit]" -> "Song Name"
///              "Cool Songs (Deluxe Edition)" -> "Cool Songs"
pub fn strip_trailing_brackets(input: &str) -> String {
    let re = Regex::new(r"(\s*(\([^)]*\)|\[[^\]]*\]|\{[^}]*\})\s*)+$").unwrap();
    re.replace(input, "").trim().to_string()
}

pub fn strip_timestamp(synced_lyrics: &str) -> String {
    let re = Regex::new(r"^\[(.*)\] *").unwrap();
    let plain_lyrics = re.replace_all(synced_lyrics, "");
    plain_lyrics.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_trailing_brackets_square() {
        assert_eq!(strip_trailing_brackets("Song Name [Explicit]"), "Song Name");
    }

    #[test]
    fn test_strip_trailing_brackets_round() {
        assert_eq!(strip_trailing_brackets("Song Name (Explicit)"), "Song Name");
    }

    #[test]
    fn test_strip_trailing_brackets_curly() {
        assert_eq!(strip_trailing_brackets("Song Name {Explicit}"), "Song Name");
    }

    #[test]
    fn test_strip_trailing_brackets_album_deluxe() {
        assert_eq!(strip_trailing_brackets("Cool Songs (Deluxe Edition)"), "Cool Songs");
    }

    #[test]
    fn test_strip_trailing_brackets_multiple() {
        assert_eq!(strip_trailing_brackets("Song [Explicit] (Remastered)"), "Song");
    }

    #[test]
    fn test_strip_trailing_brackets_no_brackets() {
        assert_eq!(strip_trailing_brackets("Song Name"), "Song Name");
    }

    #[test]
    fn test_strip_trailing_brackets_empty_string() {
        assert_eq!(strip_trailing_brackets(""), "");
    }

    #[test]
    fn test_strip_trailing_brackets_middle_brackets_preserved() {
        assert_eq!(strip_trailing_brackets("Song (feat. Artist) Name"), "Song (feat. Artist) Name");
    }

    #[test]
    fn test_strip_trailing_brackets_consecutive_trailing_removed() {
        assert_eq!(strip_trailing_brackets("Song (feat. Artist) [Explicit]"), "Song");
    }

    #[test]
    fn test_strip_trailing_brackets_non_trailing_preserved() {
        assert_eq!(strip_trailing_brackets("Song (feat. Artist) Name [Explicit]"), "Song (feat. Artist) Name");
    }
}
