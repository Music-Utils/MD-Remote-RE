use gpio::{
    dummy::{DummyGpioIn, DummyGpioOut},
    GpioIn, GpioValue,
};
use std::{
    thread,
    time::{self, Duration, Instant, SystemTime},
};

fn main() {
    let mut timed_gpio = DummyGpioIn::new(|| {
        std::time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_micros()
            % 2
            == 0
    });
    let mut count_total = 0;
    let mut count_high = 0;
    let mut s = "".to_string();
    let mut c: usize = 0;
    loop {
        if c == 10000 {
            println!("{}", s);
            let s = "".to_string();
            c = 0;
        }
        if timed_gpio.read_value().unwrap() == GpioValue::High {
            s.push_str("-");
        } else {
            s.push_str("_");
        }
        c += 1;
    }
}
