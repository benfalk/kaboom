use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone)]
pub enum State {
    Ready,
    Started(Instant),
    Victory(Duration),
    Failure(Duration),
}

use State::*;

impl Default for State {
    fn default() -> Self {
        Ready
    }
}

impl State {
    pub fn seconds_running(&self) -> u64 {
        match self {
            Ready => 0,
            Victory(time) => time.as_secs(),
            Failure(time) => time.as_secs(),
            Started(when) => Instant::now().duration_since(*when).as_secs()
        }
    }

    pub fn is_over(&self) -> bool {
        match self {
            Victory(_) => true,
            Failure(_) => true,
            _ => false
        }
    }

    pub fn start() -> Self {
        Started(Instant::now())
    }

    pub fn to_failure(self) -> Self {
        match self {
            Ready => Failure(Duration::new(0, 0)),
            Victory(time) => Failure(time),
            Started(when) => Failure(Instant::now().duration_since(when)),
            failure => failure
        }
    }

    pub fn to_victory(self) -> Self {
        match self {
            Ready => Victory(Duration::new(0, 0)),
            Failure(time) => Victory(time),
            Started(when) => Victory(Instant::now().duration_since(when)),
            victory => victory
        }
    }

    pub fn is_ready(&self) -> bool {
        match self {
            Ready => true,
            _ => false,
        }
    }
}
