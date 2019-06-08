use std::thread::sleep;
use core::borrow::Borrow;
use std::time::Duration;
use std::cmp::Ordering::Greater;

// 把红绿灯看成一个状态机，状态转换过程如下：
//+-----------+      +------------+      +---------+
//|   Green   +----->+   Yellow   +----->+   Red   |
//+-----+-----+      +------------+      +----+----+
//      ^                                     |
//      |                                     |
//      +-------------------------------------+

#[derive(Debug)]
struct Red {
    wait_time: Duration
}

impl Red {
    fn new() -> Self {
        Red{
            wait_time: Duration::new(60, 0)
        }
    }
}

#[derive(Debug)]
struct Green {
    wait_time: std::time::Duration
}

impl Green {
    fn new() -> Self {
        Green{
            wait_time: Duration::new(60, 0)
        }
    }
}

#[derive(Debug)]
struct Yellow {
    wait_time: std::time::Duration
}

impl Yellow {
    fn new() -> Self {
        Yellow{
            wait_time: Duration::new(10, 0)
        }
    }
}

#[derive(Debug)]
struct TrafficLight<TLS> {
    state: TLS,
}

impl TrafficLight<Green> {
    fn new() -> Self {
        TrafficLight {
            state: Green::new(),
        }
    }
}

impl From<TrafficLight<Green>> for TrafficLight<Yellow> {
    fn from(green: TrafficLight<Green>) -> TrafficLight<Yellow> {
        println!("last state is {:?}", green);
        sleep(green.state.wait_time);
        TrafficLight {
            state: Yellow::new(),
        }
    }
}

impl From<TrafficLight<Yellow>> for TrafficLight<Red> {
    fn from(yellow: TrafficLight<Yellow>) -> TrafficLight<Red> {
        println!("last state is {:?}", yellow);
        sleep(yellow.state.wait_time);
        TrafficLight {
            state: Red::new(),
        }
    }
}

impl From<TrafficLight<Red>> for TrafficLight<Green> {
    fn from(red: TrafficLight<Red>) -> TrafficLight<Green> {
        println!("last state is {:?}", red);
        sleep(red.state.wait_time);
        TrafficLight {
            state: Green::new(),
        }
    }
}

enum TrafficLightWrapper {
    Red(TrafficLight<Red>),
    Green(TrafficLight<Green>),
    Yellow(TrafficLight<Yellow>),
}

impl TrafficLightWrapper {
    fn new() -> Self {
        TrafficLightWrapper::Green(TrafficLight::new())
    }
    fn step(mut self) -> Self {
        match self {
            TrafficLightWrapper::Green(green) => TrafficLightWrapper::Yellow(green.into()),
            TrafficLightWrapper::Yellow(yellow) => TrafficLightWrapper::Red(yellow.into()),
            TrafficLightWrapper::Red(red) => TrafficLightWrapper::Green(red.into())
        }
    }
}

fn main() {
    let mut state_machine = TrafficLightWrapper::new();

    loop {
        state_machine = state_machine.step();
    }
}
