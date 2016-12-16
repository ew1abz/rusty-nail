// extern crate sysfs_gpio;
// extern crate sysfs_pwm;
//
// use sysfs_gpio::{Direction, Pin};
// use std::thread::sleep;
// use std::time::Duration;
//
// fn main() {
//     let my_pin = Pin::new(68);
//     println!("{:?}", my_pin);
//
//     my_pin.with_exported(|| {
//         my_pin.set_direction(Direction::Out).unwrap();
//         loop {
//             my_pin.set_value(0).unwrap();
//             sleep(Duration::from_millis(1000));
//             my_pin.set_value(1).unwrap();
//             sleep(Duration::from_millis(1000));
//         }
//     }).unwrap();
// }

// Copyright 2016, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.
//
// Portions of this implementation are based on work by Nat Pryce:
// https://github.com/npryce/rusty-pi/blob/master/src/pi/gpio.rs

extern crate sysfs_pwm;
use sysfs_pwm::{Pwm, Result};

// PIN: EHRPWM0A (P8_13)
const BB_PWM_CHIP: u32 = 6;
const BB_PWM_NUMBER: u32 = 1;

fn pwm_increase_to_max(pwm: &Pwm,
                       duration_ms: u32,
                       update_period_ms: u32) -> Result<()> {
    let step: f32 = duration_ms as f32 / update_period_ms as f32;
    let mut duty_cycle = 0.0;
    let period_ns: u32 = try!(pwm.get_period_ns());
    while duty_cycle < 1.0 {
        try!(pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32));
        duty_cycle += step;
    }
    pwm.set_duty_cycle_ns(period_ns)
}

fn pwm_decrease_to_minimum(pwm: &Pwm,
                           duration_ms: u32,
                           update_period_ms: u32) -> Result<()> {
    let step: f32 = duration_ms as f32 / update_period_ms as f32;
    let mut duty_cycle = 1.0;
    let period_ns: u32 = try!(pwm.get_period_ns());
    while duty_cycle > 0.0 {
        try!(pwm.set_duty_cycle_ns((duty_cycle * period_ns as f32) as u32));
        duty_cycle -= step;
    }
    pwm.set_duty_cycle_ns(0)
}

/// Make an LED "breathe" by increasing and
/// decreasing the brightness
fn main() {
    let pwm = Pwm::new(BB_PWM_CHIP, BB_PWM_NUMBER).unwrap(); // number depends on chip, etc.
    pwm.with_exported(|| {
        pwm.enable(true).unwrap();
        pwm.set_period_ns(20_000).unwrap();
        pwm.set_duty_cycle_ns(1000)
    }).unwrap();
}
