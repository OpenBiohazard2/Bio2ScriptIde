#[allow(dead_code)]
pub struct RDTHeader {
    pub num_sprites: u8,
    pub num_cameras: u8,
    pub num_models: u8,
    pub num_items: u8,
    pub num_doors: u8,
    pub num_rooms: u8,
    pub num_reverb: u8, // related to sound
    pub sprite_max: u8, // max number of .pri sprites used by one of the room's cameras
    pub offsets: [u32; 23],
}

impl RDTHeader {
    pub fn from(v: &[u8]) -> Result<RDTHeader, &'static str> {
        const HEADER_SIZE: usize = 8;
        const OFFSET_COUNT: usize = 23;
        const OFFSET_SIZE: usize = 4;
        const TOTAL_OFFSET_SIZE: usize = OFFSET_COUNT * OFFSET_SIZE;
        const MIN_FILE_SIZE: usize = HEADER_SIZE + TOTAL_OFFSET_SIZE;

        if v.len() < MIN_FILE_SIZE {
            return Err("File too small: expected at least 100 bytes for RDT header");
        }

        let header = &v[0..HEADER_SIZE];
        let rdt_offsets_bytes = &v[HEADER_SIZE..HEADER_SIZE + TOTAL_OFFSET_SIZE];

        let mut rdt_offsets = [0; OFFSET_COUNT];
        for (i, chunk) in rdt_offsets_bytes.chunks(OFFSET_SIZE).enumerate() {
            if chunk.len() != OFFSET_SIZE {
                return Err("Invalid offset data: incomplete offset entry");
            }
            rdt_offsets[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }

        let num_sprites = header[0];
        let num_cameras = header[1];
        let num_models = header[2];
        let num_items = header[3];
        let num_doors = header[4];
        let num_rooms = header[5];
        let num_reverb = header[6];
        let sprite_max = header[7];

        Ok(RDTHeader {
            num_sprites,
            num_cameras,
            num_models,
            num_items,
            num_doors,
            num_rooms,
            num_reverb,
            sprite_max,
            offsets: rdt_offsets,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a minimal valid 100-byte RDT header: the 8 counter bytes followed by 23
    /// little-endian u32 offsets, defaulting to zero except where overridden.
    fn header_bytes(counters: [u8; 8], offset_overrides: &[(usize, u32)]) -> Vec<u8> {
        let mut bytes = counters.to_vec();
        let mut offsets = [0u32; 23];
        for &(i, value) in offset_overrides {
            offsets[i] = value;
        }
        for offset in offsets {
            bytes.extend_from_slice(&offset.to_le_bytes());
        }
        bytes
    }

    #[test]
    fn parses_counters_and_offsets() {
        let bytes = header_bytes([1, 2, 3, 4, 5, 6, 7, 8], &[(16, 0x1234), (17, 0xabcd)]);
        let header = RDTHeader::from(&bytes).unwrap();

        assert_eq!(header.num_sprites, 1);
        assert_eq!(header.num_cameras, 2);
        assert_eq!(header.num_models, 3);
        assert_eq!(header.num_items, 4);
        assert_eq!(header.num_doors, 5);
        assert_eq!(header.num_rooms, 6);
        assert_eq!(header.num_reverb, 7);
        assert_eq!(header.sprite_max, 8);
        assert_eq!(header.offsets[16], 0x1234);
        assert_eq!(header.offsets[17], 0xabcd);
        assert_eq!(header.offsets.len(), 23);
    }

    #[test]
    fn rejects_input_shorter_than_min_size() {
        let bytes = header_bytes([0; 8], &[]);
        // One byte short of the required 100.
        assert!(RDTHeader::from(&bytes[..bytes.len() - 1]).is_err());
        assert!(RDTHeader::from(&[]).is_err());
    }

    #[test]
    fn accepts_exactly_min_size() {
        let bytes = header_bytes([0; 8], &[]);
        assert_eq!(bytes.len(), 100);
        assert!(RDTHeader::from(&bytes).is_ok());
    }
}
