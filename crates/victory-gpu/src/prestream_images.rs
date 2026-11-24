// Capture the normal presentation image only; never enlist a frame in operation logging.
const PICK: &[u64] = &[1600, 1630, 1650, 1700, 1750, 1800, 1850, 1900, 1950, 2000, 2047, 2048, 2049, 2100, 2101, 2102, 2300, 2301, 2302, 2600, 2601, 2602, 3000, 3001];
pub(super) fn selected(n: u64) -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var("VICTORY_PRESTREAM_IMAGES").as_deref() == Ok("1"))
        && PICK.binary_search(&n).is_ok()
}
