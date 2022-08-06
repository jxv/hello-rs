use std::time::{Duration, SystemTime};

#[cfg(test)]

pub trait Console {
    fn stdout(&mut self, message: &String);
    fn sysarg(&mut self);
}

pub trait FileSystem {
    fn read_file(&mut self, file_path: &String) -> String;
}

pub trait Configuration {
    fn target(&mut self) -> String;
}

pub trait Greeter {
    fn greet(&mut self, target: &String);
}

pub trait Clock {
    fn get_current_time(&mut self) -> SystemTime;
}

pub trait Notifier {
    fn take_time(&mut self, time: &SystemTime);
}

pub trait Timer {
    fn measure(&mut self, f: &dyn FnMut());
}

struct TimerImpl<'a> {
    pub clock: &'a mut dyn Clock,
}

impl<'a> Timer for TimerImpl<'a> {
    fn measure(&mut self, func: &dyn FnMut()) {
        let start = self.clock.get_current_time();
        // TODO: call `func`
        let end = self.clock.get_current_time(); 
    }
}

pub fn run(timer: &mut dyn Timer, greeter: &mut dyn Greeter, configuration: &mut dyn Configuration) {
    let target = configuration.target();
    timer.measure(&|| { greeter.greet(&target) });
}

mod tests {
    #[test]
    fn exploration() {
        assert_eq!(true, true);
    }
    
    #[test]
    fn another() {
        assert_eq!(true, true);
    }
}
