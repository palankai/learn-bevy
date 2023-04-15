pub fn path_join(segments: Vec<&str>) -> String {
    segments.join(std::path::MAIN_SEPARATOR_STR)
}
