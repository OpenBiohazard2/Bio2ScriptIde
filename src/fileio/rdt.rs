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
