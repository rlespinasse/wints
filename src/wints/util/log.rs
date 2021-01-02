pub static CAUTION: &str = "⚠️";
pub static INFO: &str = "ℹ️";
pub static SAD: &str = "😢";
pub static DRY_RUN: &str = "🌀";

pub static SEARCH: &str = "🔎";
pub static WRITE: &str = "📝";
pub static TRY: &str = "🧭";
pub static DONE: &str = "✅";
pub static GOTO: &str = "↗️";

#[cfg(test)]
mod tests {
    use crate::util::log::*;

    #[test]
    fn caution_output() {
        assert_eq!("[⚠️]", format!("[{}]", CAUTION));
    }

    #[test]
    fn info_output() {
        assert_eq!("[ℹ️]", format!("[{}]", INFO));
    }

    #[test]
    fn sad_output() {
        assert_eq!("[😢]", format!("[{}]", SAD));
    }

    #[test]
    fn dry_run_output() {
        assert_eq!("[🌀]", format!("[{}]", DRY_RUN));
    }

    #[test]
    fn search_output() {
        assert_eq!("[🔎]", format!("[{}]", SEARCH));
    }

    #[test]
    fn write_output() {
        assert_eq!("[📝]", format!("[{}]", WRITE));
    }

    #[test]
    fn try_output() {
        assert_eq!("[🧭]", format!("[{}]", TRY));
    }

    #[test]
    fn done_output() {
        assert_eq!("[✅]", format!("[{}]", DONE));
    }

    #[test]
    fn goto_output() {
        assert_eq!("[↗️]", format!("[{}]", GOTO));
    }
}
