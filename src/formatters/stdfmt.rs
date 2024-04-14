use crate::rfs::RfsScan;

pub enum FormatterType {
    YAML,
    CSV,
    TEXT,
}

/// Get formatter choices
pub fn get_fmt_choice(c: String) -> FormatterType {
    match c.to_lowercase().as_str() {
        "yaml" => FormatterType::YAML,
        "csv" => FormatterType::CSV,
        &_ => FormatterType::TEXT,
    }
}

/// Returns string representations of formatter choices
pub fn get_fmt_choices() -> [&'static str; 3] {
    // Yes, it is possible also to write this with impl to the enum
    // as an iterator etc. But this way is just stupid simpler.
    ["yaml", "csv", "text"]
}

/// DataFormatter is an interface to the formatters
pub trait DataFormatter<'a> {
    fn new(rfs: &'a RfsScan) -> Box<dyn DataFormatter + 'a>
    where
        Self: Sized;

    /// Formats the licensing data output
    fn format(&self) -> String;
}
