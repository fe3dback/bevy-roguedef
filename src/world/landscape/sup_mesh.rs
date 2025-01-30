use bevy::asset::RenderAssetUsages;
use bevy::prelude::{Handle, Mesh, Vec3};
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use brg_core::prelude::{BlockPosition, Chunk, V2};

use super::dto::MeshIdent;
use super::enum_lod_level::EChunkLodLevel;
use super::material::TerrainMaterial;
use super::sup::SupLandscape;

impl<'w, 's> SupLandscape<'w, 's> {
    pub(super) fn ensure_terrain_material_exist(&mut self) {
        if self.state.terrain_material.is_some() {
            return;
        }

        self.state.terrain_material = Some(self.materials.add(TerrainMaterial::new(
            self.heightmap.world_size(),
            self.assets.landscape().texture_world_albedo.clone(),
            self.assets.landscape().texture_ground_grass.clone(),
        )));
    }

    pub(super) fn create_mesh(&mut self, chunk: Chunk, lod: EChunkLodLevel) -> Handle<Mesh> {
        let faces = match lod {
            EChunkLodLevel::LOD0 => Chunk::size(),
            EChunkLodLevel::LOD1 => 1,
        };

        let scale = (Chunk::size() / faces) as f32;

        // --

        let verts_width = faces + 1;
        let verts_height = faces + 1;

        let verts_count = verts_width * verts_height;
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(verts_count);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(verts_count);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(verts_count);

        // each face has 2 triangles, each triangle has 3 vertices
        let mut indices: Vec<u16> =
            Vec::with_capacity(((verts_width - 1) * (verts_height - 1)) * 6);

        let mut index = 0;
        for v_y in 0..verts_height as i32 {
            for v_x in 0..verts_width as i32 {
                let rel_pos = V2::new(v_x as f32, v_y as f32) * scale;
                let abs_pos = chunk.position_tl() + rel_pos;

                // local pos
                let abs_height = self.heightmap.height_at_pos(abs_pos);
                positions.push([rel_pos.x, abs_height, rel_pos.y]);

                // uvs
                uvs.push([
                    v_x as f32 / (verts_width - 1) as f32,
                    v_y as f32 / (verts_height - 1) as f32,
                ]);

                // normals
                let ntl = self.heightmap.height_at_pos(abs_pos + V2::new(-1.0, -1.0));
                let nt = self.heightmap.height_at_pos(abs_pos + V2::new(0.0, -1.0));
                let ntr = self.heightmap.height_at_pos(abs_pos + V2::new(1.0, -1.0));

                let nl = self.heightmap.height_at_pos(abs_pos + V2::new(-1.0, 0.0));
                let nr = self.heightmap.height_at_pos(abs_pos + V2::new(1.0, 0.0));

                let nbl = self.heightmap.height_at_pos(abs_pos + V2::new(-1.0, 1.0));
                let nb = self.heightmap.height_at_pos(abs_pos + V2::new(0.0, 1.0));
                let nbr = self.heightmap.height_at_pos(abs_pos + V2::new(1.0, 1.0));

                let norm_x = (ntr + 2.0 * nr + nbr) - (ntl + 2.0 * nl + nbl);
                let norm_y = (nbl + 2.0 * nb + nbr) - (ntl + 2.0 * nt + ntr);

                let normal = Vec3::new(norm_x, 1.0, norm_y).normalize();
                normals.push([normal.x, normal.y, normal.z]);

                // indexes
                // todo: triangle strip mem optimization
                if v_x < (verts_width - 1) as i32 && v_y < (verts_height - 1) as i32 {
                    let tl: u16 = index as u16;
                    let tr: u16 = tl + 1;
                    let bl: u16 = tl + verts_width as u16;
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

        let mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(positions),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs))
        .with_inserted_indices(Indices::U16(indices))
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(normals),
        );

        let ident = MeshIdent { chunk, lod };
        match self.state.meshes.get(&ident) {
            Some(exist_handle) => {
                // replace already exist mesh
                self.assets_meshes.insert(exist_handle, mesh);
                exist_handle.clone()
            }
            None => {
                // create new
                let new_handle = self.assets_meshes.add(mesh);
                self.state.meshes.insert(ident, new_handle.clone());
                new_handle
            }
        }
    }
}
