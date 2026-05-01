
pub struct ProgressBar {
    pub current: u32,
    pub total: u32,
}

impl ProgressBar {
    // Public methods
    pub fn new(current: &u32, total: &u32) -> Self {
        let mut progress = ProgressBar { current: *current, total: *total };
        progress.render();
        progress
    }
    pub fn update(&mut self, current: &u32) {
        self.current = *current;
        self.render();
    }

    // Private methods
    fn render(&self) {
        println!("Progress: {}/{}", self.current, self.total);
    }
}