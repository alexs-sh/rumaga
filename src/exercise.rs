use rand::Rng;

pub struct Settings {
    pub level: Level,
    pub mode: Mode,
}

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Easy,
    Normal,
    Hard,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    SumOfTwo,
    MulOfTwo,
    DivOfTwo,
    SubOfTwo,
    Square,
}

#[derive(Debug)]
pub struct Exercise {
    pub text: String,
    pub answer: i64,
}

pub struct Generator {
    settings: Settings,
}

impl Generator {
    pub fn new(settings: Settings) -> Generator {
        Generator { settings }
    }

    pub fn make_excercise(&self) -> Exercise {
        let limit = self.get_limit();
        let x = rand::thread_rng().gen_range(0..limit as i64);
        let y = rand::thread_rng().gen_range(0..limit as i64);

        let (answer, text) = match self.settings.mode {
            Mode::SumOfTwo => (x + y, format!("{} + {} = ?", x, y)),

            Mode::SubOfTwo => (x - y, format!("{} - {} = ?", x, y)),

            Mode::MulOfTwo => (x * y, format!("{} * {} = ?", x, y)),

            Mode::DivOfTwo => {
                let divisor = std::cmp::max(1, y);
                let result = x * divisor;
                (x, format!("{} / {} = ", result, divisor))
            }

            Mode::Square => (x * x, format!("{} * {} = ?", x, x)),
        };
        Exercise { text, answer }
    }

    fn get_limit(&self) -> i64 {
        match (self.settings.mode, self.settings.level) {
            (Mode::SumOfTwo, Level::Easy) => 10,
            (Mode::SumOfTwo, Level::Normal) => 100,
            (Mode::SumOfTwo, Level::Hard) => 1000,

            (Mode::SubOfTwo, Level::Easy) => 10,
            (Mode::SubOfTwo, Level::Normal) => 100,
            (Mode::SubOfTwo, Level::Hard) => 1000,

            (Mode::MulOfTwo, Level::Easy) => 10,
            (Mode::MulOfTwo, Level::Normal) => 20,
            (Mode::MulOfTwo, Level::Hard) => 100,

            (Mode::DivOfTwo, Level::Easy) => 10,
            (Mode::DivOfTwo, Level::Normal) => 20,
            (Mode::DivOfTwo, Level::Hard) => 100,

            (Mode::Square, Level::Easy) => 10,
            (Mode::Square, Level::Normal) => 20,
            (Mode::Square, Level::Hard) => 100,
        }
    }
}

impl Iterator for Generator {
    type Item = Exercise;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.make_excercise())
    }
}

impl Exercise {
    pub fn validate_answer(&self, answer: i64) -> bool {
        self.answer == answer
    }
}
