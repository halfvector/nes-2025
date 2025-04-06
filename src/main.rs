

mod gui;
mod rom;

// Import the Rom struct so we can use it
use crate::rom::Rom;

// `fn` declares functions. Main is entry point like C/Java, but no args needed
fn main() {
    println!("NES Emulator starting...");

    // Define the path to the ROM file
    let rom_path = "roms/smb.nes";
    println!("Attempting to load ROM: {}", rom_path);

    // Call the load function and handle the Result
    match Rom::load(rom_path) {
        Ok(rom) => {
            println!("Successfully loaded ROM!");
            println!("{}", rom.header);
            println!("  PRG ROM size: {} bytes", rom.prg_rom.len());
            println!("  CHR ROM size: {} bytes", rom.chr_rom.len());
        }
        Err(e) => {
            eprintln!("Error loading ROM: {}", e);
        }
    }

    // Example of how you *could* call the GUI code if needed later:
    // gui::run_gui(); 
}
