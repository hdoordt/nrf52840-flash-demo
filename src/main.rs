#![no_std]
#![no_main]

use cortex_m_rt::entry;
use hal::pac;
use nrf52840_flash_demo as _;
use nrf52840_hal as hal;

// Some address that is in the flash region.
// Needs to be 32-bit aligned
const ADDR: u32 = 0x0000_3000;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let nvmc = p.NVMC;

    // Delay for a bit so that defmt has time to connect before doing flash operations
    defmt::info!("Delaying...");
    cortex_m::asm::delay(1000_000);
    
    // Read data from flash
    let data: u32 = unsafe { *(ADDR as *const u32) };
    defmt::info!("Read data: {:x}", data);
    
    // Enable erasing
    nvmc.config.write(|w| w.wen().een());
    // Erase page starting at addr
    nvmc.erasepage()
        .write(|w| unsafe { w.erasepage().bits(ADDR) });

    // Enable writing
    nvmc.config.write(|w| w.wen().wen());
    
    // Wait until ready for next write operation
    while nvmc.readynext.read().readynext().is_busy() {}
    // write data
    defmt::info!("Writing...");
    // Alter this number for result
    let word: u32 = 0x13371337;
    *unsafe { &mut *(ADDR as *mut u32) } = word;

    // Wait until writing is done
    while nvmc.ready.read().ready().is_busy() {}
    // Read data again
    defmt::info!("Done! Reading data again: {:x}", unsafe { *(ADDR as *const u32) });
    defmt::info!("Press the RESET button to see the effect!");
    loop {}
}
