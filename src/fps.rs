extern crate time;

pub struct FPS {
    value: i32,
    value_string: String,
    counter: i32,
    last_time: i32,
}

impl FPS {
    pub fn new() -> FPS {
        FPS {
            value: 0,
            value_string: String::from("fps: 0"),
            counter: 0,
            last_time: time::now().tm_sec,
        }
    }

    pub fn calc(&mut self) -> i32 {
        self.counter = self.counter + 1;
        let current_time = time::now().tm_sec;
        if self.last_time < current_time {
            self.value = self.counter;
            self.value_string = format!("fps: {}", self.value);
            self.last_time = current_time;
            self.counter = 0;
        }
        self.value
    }

    pub fn to_string(&self) -> String {
        String::from(self.value_string.to_owned())
    }
}

#[test]
fn test_fps_new() {
    let fps = FPS::new();
    assert_eq!(fps.to_string(), "fps: 0");
}

#[test]
fn test_fps_calc() {
    use std::thread;
    use std::time::Duration;

    let sleep_1sec = Duration::from_secs(1u64);

    let mut fps = FPS::new();

    for _ in 1..10 {
        assert_eq!(fps.calc(), 0);
    }

    thread::sleep(sleep_1sec);

    assert_eq!(fps.calc(), 10);
    assert_eq!(fps.to_string(), "fps: 10");

    thread::sleep(sleep_1sec);

    assert_eq!(fps.calc(), 1);
    assert_eq!(fps.to_string(), "fps: 1");
}
