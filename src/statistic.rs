use std::time::SystemTime;

pub struct Statistic {
    pub expect: u32,
    pub correct: u32,
    start: Option<SystemTime>,
    finish: Option<SystemTime>,
}

impl Statistic {
    pub fn new(expect: u32) -> Statistic {
        Statistic {
            expect,
            correct: 0,
            start: None,
            finish: None,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(SystemTime::now());
    }

    pub fn add(&mut self, result: bool) {
        self.correct += result as u32;
    }

    pub fn finish(&mut self) {
        self.finish = Some(SystemTime::now());
    }

    pub fn duration_in_ms(&self) -> Option<u32> {
        if self.start.is_some() && self.finish.is_some() {
            let duration = self
                .finish
                .unwrap()
                .duration_since(self.start.unwrap())
                .unwrap();
            Some(duration.as_millis() as u32)
        } else {
            None
        }
    }
}
