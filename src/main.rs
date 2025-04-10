#![no_std]
#![no_main]

use embassy_stm32::{
    ospi::{
        AddressSize, ChipSelectHighTime, FIFOThresholdLevel, MemorySize, MemoryType, Ospi,
        OspiWidth, TransferConfig, WrapSize,
    },
    rcc,
};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let mut config = embassy_stm32::Config::default();
    config.rcc.msis = Some(rcc::MSIRange::RANGE_4MHZ);
    config.rcc.pll1 = Some(rcc::Pll {
        source: rcc::PllSource::MSIS,
        prediv: rcc::PllPreDiv::DIV1,
        mul: rcc::PllMul::MUL80,
        divp: Some(rcc::PllDiv::DIV2),
        divq: Some(rcc::PllDiv::DIV2),
        divr: Some(rcc::PllDiv::DIV2),
    });
    config.rcc.sys = rcc::Sysclk::PLL1_R;
    config.rcc.mux.octospisel = rcc::mux::Octospisel::SYS;

    let p = embassy_stm32::init(config);

    let flash_config = embassy_stm32::ospi::Config {
        fifo_threshold: FIFOThresholdLevel::_4Bytes,
        memory_type: MemoryType::Macronix,
        device_size: MemorySize::_64MiB,
        chip_select_high_time: ChipSelectHighTime::_2Cycle,
        free_running_clock: false,
        clock_mode: false,
        wrap_size: WrapSize::None,
        clock_prescaler: 4,
        sample_shifting: false,
        delay_hold_quarter_cycle: false,
        chip_select_boundary: 0,
        delay_block_bypass: false,
        max_transfer: 0,
        refresh: 0,
    };

    let mut octospi = Ospi::new_blocking_octospi(
        p.OCTOSPI2,
        p.PF4,
        p.PF0,
        p.PF1,
        p.PF2,
        p.PF3,
        p.PH9,
        p.PH10,
        p.PH11,
        p.PH12,
        p.PI5,
        flash_config,
    );

    let read_identification_command = TransferConfig {
        iwidth: OspiWidth::SING,
        instruction: Some(0x9f),
        isize: AddressSize::_8Bit,
        idtr: false,
        dwidth: OspiWidth::SING,
        ddtr: false,
        ..Default::default()
    };

    let mut buf = [0u8; 3];
    octospi
        .blocking_read(&mut buf, read_identification_command)
        .unwrap();

    defmt::info!("identification: {}", buf);
}
