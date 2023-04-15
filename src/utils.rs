pub fn path_join(segments: Vec<&str>) -> String {
    segments.join(std::path::MAIN_SEPARATOR_STR)
}

#[derive(Debug, Clone)]
pub struct PathJoin<'a> {
    segments: Vec<&'a str>,
}

impl<'a> PathJoin<'a> {
    pub fn new(s: &'a str) -> Self {
        PathJoin { segments: vec![s] }
    }

    pub fn join(&mut self, s: &'a str) -> Self {
        self.segments.push(s);
        self.clone()
    }

    pub fn build(&self) -> String {
        self.segments.join(std::path::MAIN_SEPARATOR_STR)
    }
}
