use bevy::asset::RenderAssetUsages;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use brg_core::prelude::consts::TERRAIN_HEIGHT;
use brg_core::prelude::{BlockChild, BlockPosition, Chunk, T_LIB_CONT_SIZE_SQ};

use super::sup::SupLandscape;

impl<'w, 's> SupLandscape<'w, 's> {
    pub(super) fn create_mesh(&mut self, chunk: Chunk) -> Mesh {
        let mesh_width = Chunk::size();
        let mesh_height = Chunk::size();

        let mut chunk_heights = [0.0; T_LIB_CONT_SIZE_SQ];
        for (ind, tile) in chunk.child_range().into_iter().enumerate() {
            chunk_heights[ind] = self.hm.height_at_pos(tile.position_tl());
        }

        let size = mesh_width * mesh_height;
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(size);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(size);
        // let mut normals: Vec<[f32; 3]> = Vec::with_capacity(size);

        // each face has 2 triangles, each triangle has 3 vertices
        let mut indices: Vec<u16> = Vec::with_capacity((mesh_width - 1 * mesh_height - 1) * 6);

        let mut index = 0;
        for y in 0..mesh_height as i32 {
            for x in 0..mesh_width as i32 {
                // find height

                // local pos
                let tile_height = chunk_heights[index];
                positions.push([x as f32, tile_height * TERRAIN_HEIGHT, y as f32]);

                // uvs
                uvs.push([
                    x as f32 / (mesh_width - 1) as f32,
                    y as f32 / (mesh_height - 1) as f32,
                ]);

                // indexes
                // todo: triangle strip mem optimization
                if x < (mesh_width - 1) as i32 && y < (mesh_height - 1) as i32 {
                    let tl: u16 = index as u16;
                    let tr: u16 = tl + 1;
                    let bl: u16 = tl + mesh_width as u16;
                    let br: u16 = bl + 1;

                    // triangle A
                    indices.push(tl);
                    indices.push(bl);
                    indices.push(br);

                    // triangle B
                    indices.push(br);
                    indices.push(tr);
                    indices.push(tl);
                }

                // move cursor
                index += 1;
            }
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(positions),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs))
        .with_inserted_indices(Indices::U16(indices))
        // .with_inserted_attribute(
        //     Mesh::ATTRIBUTE_NORMAL,
        //     VertexAttributeValues::Float32x3(normals),
        // )
    }
}
