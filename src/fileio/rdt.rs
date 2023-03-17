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
        if v.len() < 8 {
            return Err::<RDTHeader, &str>("invalid header length");
        }

        let header = &v[0..8];

        let offset_length = 23 * 4;
        let rdt_offsets_bytes = &v[8..8 + offset_length];

        let mut rdt_offsets = [0; 23];
        let mut i = 0;
        for x in rdt_offsets_bytes.chunks(4) {
            rdt_offsets[i] = u32::from_le_bytes([x[0], x[1], x[2], x[3]]);
            i += 1;
        }

        Ok(RDTHeader {
            num_sprites: header[0],
            num_cameras: header[1],
            num_models: header[2],
            num_items: header[3],
            num_doors: header[4],
            num_rooms: header[5],
            num_reverb: header[6],
            sprite_max: header[7],
            offsets: rdt_offsets,
        })
    }
}
