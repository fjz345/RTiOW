pub struct ProgressBar {
    pub progress_max: f64,
    pub progress_num_steps: i64,
    progress: f64,
}

impl ProgressBar {
    pub fn new(progress_max: f64, progress_num_steps: i64) -> Self {
        ProgressBar {
            progress_max: progress_max,
            progress_num_steps: progress_num_steps,
            progress: 0.0,
        }
    }

    pub fn progress(&self) -> f64 {
        (self.progress / self.progress_max)
    }

    pub fn progress_precent(&self) -> f64 {
        100.0 * self.progress()
    }

    pub fn calc_increment(&self) -> f64 {
        self.progress_max / (self.progress_num_steps as f64)
    }

    pub fn inc(&mut self) {
        self.progress += self.calc_increment();
    }

    pub fn is_finished(&self) -> bool {
        self.progress >= self.progress_max
    }

    pub fn print_progress_percent(&self) {
        println!("{:.1}%...", self.progress_precent());
    }
}
