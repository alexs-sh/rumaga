use std::fmt::Display;
use tokio::io::{AsyncRead, AsyncWrite, Result};
use tokio::net::TcpListener;

use crate::exercise::{Generator, Level, Mode, Settings};
use crate::io::{StdInOut, IO, LONG_LINE};
use crate::statistic::Statistic;

pub struct Game {
    address: String,
}

impl Game {
    pub fn new(address: &str) -> Game {
        Game {
            address: address.to_owned(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        if self.address.is_empty() {
            self.run_std().await?;
        } else {
            self.run_net().await?;
        }
        Ok(())
    }

    async fn run_std(&self) -> Result<()> {
        Game::run_forever(StdInOut::new()).await;
        Ok(())
    }

    async fn run_net(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        loop {
            let (socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                Game::run_forever(socket).await;
            });
        }
    }

    async fn run_forever<T>(stream: T)
    where
        T: AsyncRead + AsyncWrite + Unpin,
    {
        let mut io = GameIO::new(stream);
        let times = 20;

        loop {
            io.show_header("New game").await;

            let settings = io.read_settings().await;
            let mut statistic = Statistic::new(times as u32);

            statistic.start();

            for (idx, exercise) in Generator::new(settings).into_iter().take(times).enumerate() {
                let request = format!("{}) {}", idx + 1, exercise.text.as_str());
                let response = io.ask(request.as_str()).await;

                let is_correct = if let Ok(num) = response.parse::<i64>() {
                    exercise.validate_answer(num)
                } else {
                    false
                };

                let report = if is_correct {
                    "Correct".to_owned()
                } else {
                    format!("Incorrect. The valid answer is {}", exercise.answer)
                };

                io.raw_write(report.as_str()).await;
                statistic.add(is_correct);
            }

            statistic.finish();
            io.show_report(&statistic).await;
            io.ask("Press any key to continue...").await;
        }
    }
}
pub struct GameIO<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    low: IO<T>,
}

impl<T> GameIO<T>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    fn new(stream: T) -> GameIO<T> {
        GameIO {
            low: IO::new(stream),
        }
    }

    async fn show_header(&mut self, text: &str) {
        self.low.write(LONG_LINE).await;
        self.low.write(text).await;
        self.low.write(LONG_LINE).await;
    }

    async fn show_report(&mut self, statistic: &Statistic) {
        self.show_header("Results").await;
        self.low
            .write(format!("Correct:{}/{}", statistic.correct, statistic.expect).as_str())
            .await;

        if let Some(duration) = statistic.duration_in_ms() {
            let sec = duration as f64 / 1000.0;
            self.low
                .write(format!("Duration,s:{:.2}", sec).as_str())
                .await;
        }

        self.low.write(LONG_LINE).await;
    }

    async fn read_settings(&mut self) -> Settings {
        let modes = vec![
            (Mode::SumOfTwo, "sum of two (x + y)"),
            (Mode::SubOfTwo, "dif of two (x - y)"),
            (Mode::MulOfTwo, "mul of two (x * y)"),
            (Mode::DivOfTwo, "div of two (x / y)"),
            (Mode::Square, "square (x * x)"),
        ];

        let levels = vec![
            (Level::Easy, "easy"),
            (Level::Normal, "normal"),
            (Level::Hard, "hard"),
        ];

        let mode_names: Vec<&str> = modes.iter().map(|(_, y)| *y).collect();
        let level_names: Vec<&str> = levels.iter().map(|(_, y)| *y).collect();

        self.low.write("Select mode").await;
        let mode_idx = self.read_index(&mode_names).await;

        self.low.write("Select level").await;
        let level_idx = self.read_index(&level_names).await;

        Settings {
            level: levels[level_idx].0,
            mode: modes[mode_idx].0,
        }
    }

    async fn ask(&mut self, what: &str) -> String {
        self.low.write(what).await;
        self.low.read().await
    }

    async fn read_index<P>(&mut self, objects: &[P]) -> usize
    where
        P: Display,
    {
        loop {
            for (i, o) in objects.iter().enumerate() {
                self.raw_write(format!("  {}:{}", i + 1, o).as_str()).await;
            }

            let response = self.raw_read().await;

            if let Ok(num) = response.parse::<i64>() {
                let idx = num - 1;
                let is_correct = idx >= 0 && (0..objects.len() as i64).contains(&idx);

                if is_correct {
                    return idx as usize;
                }
            }
        }
    }

    async fn raw_write(&mut self, text: &str) {
        self.low.write(text).await;
    }

    async fn raw_read(&mut self) -> String {
        self.low.read().await
    }
}
