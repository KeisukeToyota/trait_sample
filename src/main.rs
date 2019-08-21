use std::cmp::min;

struct Robot<T> {
    battery: T,
    storage: i32,
}

trait BatteryAdapter {
    fn charge(&self, minutes: i32) -> i32;
}

struct BlueBattery {}
struct GreenBattery {}
struct RedBattery {}

struct BlueBatteryAdapter {
    battery: BlueBattery,
}

struct GreenBatteryAdapter {
    battery: GreenBattery,
}

struct RedBatteryAdapter {
    max: i32,
    battery: RedBattery,
}

impl BlueBattery {
    fn charge(&self, minutes: i32) -> i32 {
        1 * minutes
    }
}

impl GreenBattery {
    fn charge(&self, minutes: i32) -> i32 {
        100 * minutes
    }
}

impl RedBattery {
    fn supercharge(&self, minutes: i32, max: i32) -> i32 {
        min(1000 * minutes, max)
    }
}

impl BlueBatteryAdapter {
    fn new() -> Self {
        Self {
            battery: BlueBattery {},
        }
    }
}

impl BatteryAdapter for BlueBatteryAdapter {
    fn charge(&self, minutes: i32) -> i32 {
        self.battery.charge(minutes)
    }
}

impl GreenBatteryAdapter {
    fn new() -> Self {
        Self {
            battery: GreenBattery {},
        }
    }
}

impl BatteryAdapter for GreenBatteryAdapter {
    fn charge(&self, minutes: i32) -> i32 {
        self.battery.charge(minutes)
    }
}

impl RedBatteryAdapter {
    fn new(max: i32) -> Self {
        Self {
            max: max,
            battery: RedBattery {},
        }
    }
}

impl BatteryAdapter for RedBatteryAdapter {
    fn charge(&self, minutes: i32) -> i32 {
        self.battery.supercharge(minutes, self.max)
    }
}

impl<T: BatteryAdapter> Robot<T> {
    fn new(battery: T) -> Self {
        Self {
            battery: battery,
            storage: 0,
        }
    }

    fn charge(&mut self, minutes: i32) {
        self.storage = self.battery.charge(minutes);
    }

    fn punch(&mut self) {
        for _ in 0..self.storage {
            print!("オラ");
        }

        println!("ァ!\n");

        self.storage = 0;
    }
}

fn main() {
    let blue_battery = BlueBatteryAdapter::new();
    let mut blue_robot = Robot::new(blue_battery);
    blue_robot.charge(10);
    blue_robot.punch();

    let green_battery = GreenBatteryAdapter::new();
    let mut green_robot = Robot::new(green_battery);
    green_robot.charge(10);
    green_robot.punch();

    let red_battery = RedBatteryAdapter::new(10000);
    let mut red_robot = Robot::new(red_battery);
    red_robot.charge(10);
    red_robot.punch();
}
