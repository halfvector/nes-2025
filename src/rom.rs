use std::fs::File;
use std::io::Read;
use std::fmt;

// Constants for ROM page sizes
pub const PRG_ROM_PAGE_SIZE: usize = 0x4000; // 16 KiB
pub const CHR_ROM_PAGE_SIZE: usize = 0x2000; // 8 KiB
const INES_HEADER_SIZE: usize = 16;
const TRAINER_SIZE: usize = 512;
const NES_SIGNATURE: [u8; 4] = [b'N', b'E', b'S', 0x1A];

/// Represents the different mirroring modes used by NES mappers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
    // Note: Some mappers override this static flag.
}

/// Represents the iNES header structure.
/// #[repr(C, packed)] ensures that the struct layout in memory
/// matches the C/C++ equivalent, with no padding bytes added by the compiler.
/// This is crucial for correctly reading the header directly from a byte stream.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)] // Add common traits for usability
pub struct RomHeader {
    /// Should be "NES\x1A" to identify an iNES file.
    pub signature: [u8; 4],
    /// Number of 16 KiB PRG-ROM pages.
    pub num_prg_pages: u8,
    /// Number of 8 KiB CHR-ROM pages. (0 means CHR-RAM is used)
    pub num_chr_pages: u8,
    /// Control Byte 1: Contains mapper lower nibble, mirroring, battery, trainer info.
    pub control_byte1: u8,
    /// Control Byte 2: Contains mapper upper nibble, NES 2.0 identifier.
    pub control_byte2: u8,
    /// Reserved bytes, typically unused in standard iNES format.
    pub _reserved: [u8; 8], // Leading underscore indicates potentially unused field
}

impl RomHeader {
    /// Determines the mirroring type based on control byte 1.
    /// Bit 0: 0 = Horizontal, 1 = Vertical
    /// Bit 3: 1 = Four-screen mode (overrides Bit 0)
    pub fn mirroring(&self) -> Mirroring {
        let four_screen = (self.control_byte1 & 0b0000_1000) != 0;
        let vertical = (self.control_byte1 & 0b0000_0001) != 0;

        if four_screen {
            Mirroring::FourScreen
        } else if vertical {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        }
    }

    /// Checks if the cartridge has battery-backed PRG RAM ($6000-7FFF).
    /// Based on Bit 1 of control byte 1.
    pub fn has_battery_backed_ram(&self) -> bool {
        (self.control_byte1 & 0b0000_0010) != 0
    }

    /// Checks if a 512-byte trainer is present at $7000-$71FF.
    /// Based on Bit 2 of control byte 1.
    pub fn has_trainer(&self) -> bool {
        (self.control_byte1 & 0b0000_0100) != 0
    }

    /// Calculates the iNES mapper number from control bytes 1 and 2.
    /// Mapper ID = Upper nibble of Control Byte 2 | Lower nibble of Mapper ID (from upper nibble of CB1)
    pub fn mapper_id(&self) -> u8 {
        // Standard calculation: (Upper nibble CB2) | (Upper nibble CB1 shifted down)
        (self.control_byte2 & 0xF0) | (self.control_byte1 >> 4)
    }
}

impl fmt::Display for RomHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the write! macro to format the output into the formatter `f`.
        // The `?` operator handles potential write errors.
        write!(f, "iNES Header:\n")?;
        // Format signature bytes as hexadecimal
        write!(f, "  Signature: {:02X?}\n", self.signature)?;
        write!(f, "  PRG ROM Pages: {}\n", self.num_prg_pages)?;
        write!(f, "  CHR ROM Pages: {}\n", self.num_chr_pages)?;
        // Use the Debug format for the Mirroring enum
        write!(f, "  Mirroring: {:?}\n", self.mirroring())?;
        write!(f, "  Battery RAM: {}\n", self.has_battery_backed_ram())?;
        write!(f, "  Trainer Present: {}\n", self.has_trainer())?;
        write!(f, "  Mapper ID: {}\n", self.mapper_id())?; // Add mapper ID to display
        // Indicate success
        Ok(())
    }
}

/// Represents a loaded NES ROM, containing the header and the PRG/CHR data.
#[derive(Debug, Clone)] // Add common traits
pub struct Rom {
    /// The parsed iNES header.
    pub header: RomHeader,
    /// Program ROM data (CPU code and data).
    /// Stored as a Vec<u8>, Rust's growable array type.
    pub prg_rom: Vec<u8>,
    /// Character ROM data (graphics tiles).
    /// Can be empty if the cartridge uses CHR-RAM.
    pub chr_rom: Vec<u8>,
    // We might add fields later for trainer data, mapper type, etc.
}

// We can add methods to Rom later, e.g., for loading from a file.
impl Rom {
    /// Loads an iNES ROM file from the given path.
    /// 
    /// Reads the header, validates it, and extracts PRG and CHR ROM data.
    /// Returns a Result containing the Rom struct or an error message string.
    pub fn load(path: &str) -> Result<Self, String> {
        // Attempt to open the file
        let mut file = File::open(path)
            .map_err(|e| format!("Failed to open file '{}': {}", path, e))?;
        
        // Read the entire file content into a buffer (Vec<u8>)
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Failed to read file '{}': {}", path, e))?;

        // Check if the buffer is large enough to contain the header
        if buffer.len() < INES_HEADER_SIZE {
            return Err(format!("File '{}' is too small to be a valid iNES ROM ({} bytes)", path, buffer.len()));
        }

        // --- Parse Header Safely --- 
        // Directly transmuting &[u8] to RomHeader is unsafe if the alignment or size
        // doesn't exactly match, even with #[repr(C, packed)]. 
        // Copying field by field is safer.
        let header = RomHeader {
            signature: [buffer[0], buffer[1], buffer[2], buffer[3]],
            num_prg_pages: buffer[4],
            num_chr_pages: buffer[5],
            control_byte1: buffer[6],
            control_byte2: buffer[7],
            // Assuming the reserved field is less critical, but we can parse it too
            _reserved: buffer[8..INES_HEADER_SIZE].try_into().map_err(|_| "Failed to read reserved header bytes".to_string())?,
        };
        
        // Validate the iNES signature
        if header.signature != NES_SIGNATURE {
            return Err(format!("Invalid iNES signature found in '{}'", path));
        }
        
        // --- Calculate Sizes and Offsets --- 
        let has_trainer = (header.control_byte1 & 0b0000_0100) != 0;
        let trainer_offset = if has_trainer { TRAINER_SIZE } else { 0 };
        
        let prg_rom_size = header.num_prg_pages as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = header.num_chr_pages as usize * CHR_ROM_PAGE_SIZE;

        let prg_rom_start = INES_HEADER_SIZE + trainer_offset;
        let chr_rom_start = prg_rom_start + prg_rom_size;

        let expected_total_size = INES_HEADER_SIZE + trainer_offset + prg_rom_size + chr_rom_size;

        // Check if the file size matches the expected size based on header info
        if buffer.len() < expected_total_size {
             return Err(format!(
                "File '{}' size ({}) does not match expected size ({}) based on header.",
                path, buffer.len(), expected_total_size
            ));
        }
        // Optionally, warn if file is larger than expected
        if buffer.len() > expected_total_size {
            println!(
                "Warning: File '{}' size ({}) is larger than expected size ({}). Extra data ignored.",
                path, buffer.len(), expected_total_size
            );
        }

        // --- Extract ROM Data --- 
        let prg_rom = buffer[prg_rom_start..prg_rom_start + prg_rom_size].to_vec();
        let chr_rom = buffer[chr_rom_start..chr_rom_start + chr_rom_size].to_vec();

        // Create and return the Rom struct
        Ok(Rom {
            header,
            prg_rom,
            chr_rom,
        })
    }
}
