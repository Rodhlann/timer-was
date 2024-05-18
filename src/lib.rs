use wasm_bindgen::prelude::*;
use web_time::Instant;
mod utils;

#[wasm_bindgen]
pub enum Time {
    Seconds,
    Millis,
    Micros,
    Nanos,
}

#[wasm_bindgen]
pub struct Timer {
    start: Instant,
}

#[wasm_bindgen]
impl Timer {
    pub fn start() -> Timer {
        utils::set_panic_hook();
        Timer {
            start: Instant::now(),
        }
    }

    pub fn end(&self, time: Time) -> String {
        let end = Instant::now();
        let duration = end.duration_since(self.start);
        match time {
            Time::Seconds => duration.as_secs().to_string(),
            Time::Millis => duration.as_millis().to_string(),
            Time::Micros => duration.as_micros().to_string(),
            Time::Nanos => duration.as_nanos().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn it_works() {
        let timer = Timer::start();
        std::thread::sleep(Duration::from_secs(1));
        let end = timer.end(Time::Seconds);

        assert_eq!(end, "1");
    }
}
