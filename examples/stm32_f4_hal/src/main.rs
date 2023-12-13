//! Target board: stm32f303RETx (stm32nucleo)
//! Manual: https://www.st.com/resource/en/reference_manual/dm00043574-stm32f303xb-c-d-e-stm32f303x6-8-stm32f328x8-stm32f358xc-stm32f398xe-advanced-arm-based-mcus-stmicroelectronics.pdf
#![no_main]
#![no_std]

// Panic handler
use panic_halt as _;

use cortex_m_rt::entry;
use mavlink;
use core::fmt::Write;

use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    serial::config::Config, 
    serial::Serial,
};
use fugit::{Rate};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(Rate::<u32, 1, 1>::from_raw(8000000)).freeze();
    // let clocks = rcc.cfgr.use_hse(8.mhz()).freeze();

    let gpioa = dp.GPIOA.split();
    

    let tx_pin = gpioa.pa2;
    let mut tx = Serial::tx(dp.USART2, tx_pin, Config::default().baudrate(57600.bps()), &clocks).unwrap();


    let mut value: u8 = 0;

    // Create our mavlink header and heartbeat message
    let header = mavlink_header();
    let heartbeat = mavlink_heartbeat_message();
    let attitude = mavlink_attitude_message();

    // Create a delay object based on SysTick
    // let mut delay = hal::delay::Delay::new(cp.SYST, clocks);
    let mut delay = dp.TIM1.delay_ms(&clocks);

    // Main loop
    loop {
        // Write the mavlink message via serial
        mavlink::write_versioned_msg(&mut tx, mavlink::MavlinkVersion::V2, header, &heartbeat)
            .unwrap();

        mavlink::write_versioned_msg(&mut tx, mavlink::MavlinkVersion::V2, header, &attitude)
            .unwrap();

        // Delay for 1 second
        delay.delay_ms(1_000u32);
    }
}

fn mavlink_header() -> mavlink::MavHeader {
    mavlink::MavHeader {
        system_id: 1,
        component_id: 1,
        sequence: 42,
    }
}

pub fn mavlink_heartbeat_message() -> mavlink::common::MavMessage {
    mavlink::common::MavMessage::HEARTBEAT(mavlink::common::HEARTBEAT_DATA {
        custom_mode: 0,
        mavtype: mavlink::common::MavType::MAV_TYPE_SUBMARINE,
        autopilot: mavlink::common::MavAutopilot::MAV_AUTOPILOT_ARDUPILOTMEGA,
        base_mode: mavlink::common::MavModeFlag::empty(),
        system_status: mavlink::common::MavState::MAV_STATE_STANDBY,
        mavlink_version: 0x3,
    })
}


pub fn mavlink_attitude_message() -> mavlink::common::MavMessage {
    mavlink::common::MavMessage::ATTITUDE(mavlink::common::ATTITUDE_DATA {
        pitch: 0.1,
        pitchspeed: 2.0,
        roll: 0.0,
        rollspeed: 0.0,
        yaw: 0.0,
        yawspeed: 0.0,
        time_boot_ms: 100,
    })
}
