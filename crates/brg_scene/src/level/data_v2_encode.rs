use anyhow::{Context, Result};
use binary_rw::{BinaryReader, BinaryWriter, Endian, MemoryStream, SliceStream};
use brg_core::prelude::T_LIB_CONT_SIZE_SQ;

use super::data_v2::{
    LevelData,
    LevelDataLandscape,
    LevelDataLandscapeArea,
    LevelDataLandscapeChunk,
};

impl LevelData {
    pub fn encode(&self) -> Result<Vec<u8>> {
        let mut buff = MemoryStream::new();
        let mut data = BinaryWriter::new(&mut buff, Endian::Little);

        // basic fields
        {
            data.write_u8(self.version)?;
            data.write_string(self.name.clone())?;
            data.write_u32(self.width)?;
            data.write_u32(self.height)?;
        }

        // landscape
        {
            let (w, h) = (self.width, self.height);
            self.landscape
                .encode(w, h, &mut data)
                .context("encode landscape")?;
        }

        Ok(buff.into())
    }

    pub fn decode(data: Vec<u8>) -> Result<Self> {
        let mut stream = SliceStream::new(&data);
        let mut data = BinaryReader::new(&mut stream, Endian::Little);

        let version = data.read_u8()?;
        assert_eq!(version, 1, "require version 1");

        let name = data.read_string()?;
        let width = data.read_u32()?;
        let height = data.read_u32()?;

        let mut level = Self::new(name, width, height);
        level
            .landscape
            .decode(width, height, &mut data)
            .context("decode landscape")?;

        Ok(level)
    }
}

impl LevelDataLandscape {
    fn encode(&self, w: u32, h: u32, data: &mut BinaryWriter) -> Result<()> {
        // asserts
        {
            let expected_len = (w * h) as usize;
            let actual_len = self.areas.len();

            assert_eq!(
                expected_len, actual_len,
                "landscape areas len is not EQ: expected={}, actual={}",
                expected_len, actual_len
            );
        }

        // write areas
        {
            for (ind, area) in self.areas.iter().enumerate() {
                area.encode(data).context(format!("encode area: {}", ind))?;
            }
        }

        Ok(())
    }

    fn decode(&mut self, w: u32, h: u32, data: &mut BinaryReader) -> Result<()> {
        // read areas
        {
            let len = (w * h) as usize;

            for ind in 0..len {
                self.areas[ind]
                    .decode(data)
                    .context(format!("decode area {}", ind))?;
            }
        }

        Ok(())
    }
}

impl LevelDataLandscapeArea {
    fn encode(&self, data: &mut BinaryWriter) -> Result<()> {
        // write area heights
        {
            // center
            data.write_f32(self.heights.0)?;

            // corners
            for h in self.heights.1 {
                data.write_f32(h)?;
            }
        }

        // write area chunks
        {
            data.write_bool(self.has_chunks)?;

            if self.has_chunks {
                for (ind, chunk) in self.chunks.iter().enumerate() {
                    chunk
                        .encode(data)
                        .context(format!("encode chunk: {}", ind))?;
                }
            }
        }

        Ok(())
    }

    fn decode(&mut self, data: &mut BinaryReader) -> Result<()> {
        // read area heights
        {
            // center
            self.heights.0 = data.read_f32()?;

            // corners
            for ind in 0..self.heights.1.len() {
                self.heights.1[ind] = data.read_f32()?;
            }
        }

        // read chunks
        {
            self.has_chunks = data.read_bool()?;
            if self.has_chunks {
                self.chunks = vec![
                    LevelDataLandscapeChunk::new([0.0; T_LIB_CONT_SIZE_SQ]);
                    T_LIB_CONT_SIZE_SQ
                ];

                for ind in 0..T_LIB_CONT_SIZE_SQ {
                    self.chunks[ind]
                        .decode(data)
                        .context(format!("decode chunk {}", ind))?;
                }
            }
        }

        Ok(())
    }
}

impl LevelDataLandscapeChunk {
    fn encode(&self, data: &mut BinaryWriter) -> Result<()> {
        // write heights
        {
            for h in self.heights {
                data.write_f32(h)?;
            }
        }

        Ok(())
    }

    fn decode(&mut self, data: &mut BinaryReader) -> Result<()> {
        // read heights
        {
            for ind in 0..self.heights.len() {
                self.heights[ind] = data.read_f32()?;
            }
        }

        Ok(())
    }
}
