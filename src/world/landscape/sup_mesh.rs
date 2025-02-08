use bevy::asset::RenderAssetUsages;
use bevy::prelude::{Handle, Mesh, Vec3};
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use brg_core::prelude::{Chunk, V2};

use super::dto::MeshIdent;
use super::material::TerrainMaterial;
use super::sup::SupLandscape;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum NeighbourSizeTransition {
    None,
    OneSide(Side),
    TwoSides(Side, Side),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Side {
    Top    = 0,
    Bottom = 1,
    Left   = 2,
    Right  = 3,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

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

    pub(super) fn create_mesh(
        &mut self,
        ident: MeshIdent,
        transitions: NeighbourSizeTransition,
    ) -> Handle<Mesh> {
        let key_verts_cnt = (Chunk::size() + 1) * (Chunk::size() + 1);
        let key_indexes_cnt = (Chunk::size() * Chunk::size()) * 2 * 3; // faces * 2D * triangles_per_face * vertices_per_triangle

        let additional_verts_cnt = match transitions {
            NeighbourSizeTransition::None => 0,
            NeighbourSizeTransition::OneSide(_) => Chunk::size(),
            NeighbourSizeTransition::TwoSides(_, _) => {
                let side_faces = (Chunk::size() - 1) * 2; // (-1 = without corner) (*2 = two sides)
                let corner_additional_verts = 2;

                // for every face we have 1 additional vert
                // corner have 2 additional (+1 for every side)
                side_faces + corner_additional_verts
            }
        };
        let additional_indexes_cnt = match transitions {
            NeighbourSizeTransition::None => 0,
            NeighbourSizeTransition::OneSide(_) => Chunk::size() * 3, // +1 triangle per face
            NeighbourSizeTransition::TwoSides(_, _) => {
                let side_faces = (Chunk::size() - 1) * 2; // (-1 = without corner) (*2 = two sides)

                // for every side face we have +1 triangle
                // corner have +2 additional triangles
                // every triangle have 3 vertices
                (side_faces + 2) * 3
            }
        };

        // preallocate arrays
        let verts_count = key_verts_cnt + additional_verts_cnt;
        let indexes_count = key_indexes_cnt + additional_indexes_cnt;

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(verts_count);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(verts_count);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(verts_count);
        let mut indices: Vec<u16> = Vec::with_capacity(indexes_count);

        let scale = 2u32.pow(ident.depth as u32) as f32;

        let mut push_vert = |rel_pos: V2, abs_pos: V2| {
            // local pos
            {
                let abs_height = self.heightmap.height_at_pos(abs_pos);
                positions.push([rel_pos.x, abs_height, rel_pos.y]);
            }

            // uvs
            {
                uvs.push([
                    rel_pos.x / Chunk::size() as f32,
                    rel_pos.y / Chunk::size() as f32,
                ]);
            }

            // normals
            {
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
            }
        };

        let additional_vert_rel_pos = |side: Side, face_ind: u8| -> V2 {
            let face_rel_pos = match side {
                Side::Top => V2::new(face_ind as f32, 0.0),
                Side::Bottom => V2::new(face_ind as f32, (Chunk::size() - 1) as f32),
                Side::Left => V2::new(0.0, face_ind as f32),
                Side::Right => V2::new((Chunk::size() - 1) as f32, face_ind as f32),
            } * scale;

            let rel_pos_inside_face = match side {
                Side::Top => V2::new(0.5, 0.0),
                Side::Bottom => V2::new(0.5, 1.0),
                Side::Left => V2::new(0.0, 0.5),
                Side::Right => V2::new(1.0, 0.5),
            } * scale;

            face_rel_pos + rel_pos_inside_face
        };

        let is_face_on_side = |side: Side, v_x: usize, v_y: usize| -> bool {
            match side {
                Side::Top => v_y == 0,
                Side::Bottom => v_y == Chunk::size() - 1,
                Side::Left => v_x == 0,
                Side::Right => v_x == Chunk::size() - 1,
            }
        };

        // populate basic verts
        {
            for v_y in 0..=Chunk::size() as i32 {
                for v_x in 0..=Chunk::size() as i32 {
                    let rel_pos = V2::new(v_x as f32, v_y as f32) * scale;
                    let abs_pos = ident.pos + rel_pos;
                    push_vert(rel_pos, abs_pos);
                }
            }
        }

        // populate additional verts
        {
            match transitions {
                NeighbourSizeTransition::None => {}
                NeighbourSizeTransition::OneSide(side) => {
                    for face in 0..Chunk::size() {
                        let rel_pos = additional_vert_rel_pos(side, face as u8);
                        let abs_pos = ident.pos + rel_pos;
                        push_vert(rel_pos, abs_pos);
                    }
                }
                NeighbourSizeTransition::TwoSides(side1, side2) => {
                    // corner info
                    let corner = match (side1, side2) {
                        (Side::Top, Side::Left) | (Side::Left, Side::Top) => Corner::TopLeft,
                        (Side::Top, Side::Right) | (Side::Right, Side::Top) => Corner::TopRight,
                        (Side::Bottom, Side::Left) | (Side::Left, Side::Bottom) => {
                            Corner::BottomLeft
                        }
                        (Side::Bottom, Side::Right) | (Side::Right, Side::Bottom) => {
                            Corner::BottomRight
                        }
                        _ => panic!(
                            "unexpected mesh sides {:?}, {:?} (should have shared corner)",
                            side1, side2
                        ),
                    };

                    let corner_idx = match corner {
                        Corner::TopLeft => 0,
                        Corner::TopRight => Chunk::size() - 1,
                        Corner::BottomRight => (Chunk::size() * Chunk::size()) - 1,
                        Corner::BottomLeft => (Chunk::size() * Chunk::size()) - Chunk::size(),
                    };

                    // side 1
                    {
                        for face in 0..Chunk::size() - 1 {
                            if face == corner_idx {
                                continue;
                            }

                            let rel_pos = additional_vert_rel_pos(side1, face as u8);
                            let abs_pos = ident.pos + rel_pos;
                            push_vert(rel_pos, abs_pos);
                        }
                    }

                    // side 2
                    {
                        for face in 0..Chunk::size() - 1 {
                            if face == corner_idx {
                                continue;
                            }

                            let rel_pos = additional_vert_rel_pos(side2, face as u8);
                            let abs_pos = ident.pos + rel_pos;
                            push_vert(rel_pos, abs_pos);
                        }
                    }

                    // corner
                    {
                        let (rel_pos1, rel_pos2) = match corner {
                            Corner::TopLeft => (V2::new(0.0, 0.5), V2::new(0.5, 0.0)),
                            Corner::TopRight => (V2::new(1.0, 0.5), V2::new(0.5, 0.0)),
                            Corner::BottomLeft => (V2::new(0.0, 0.5), V2::new(0.5, 1.0)),
                            Corner::BottomRight => (V2::new(1.0, 0.5), V2::new(0.5, 1.0)),
                        };

                        let (rel_pos1, rel_pos2) = (rel_pos1 * scale, rel_pos2 * scale);
                        let (abs_pos1, abs_pos2) = (ident.pos + rel_pos1, ident.pos + rel_pos2);

                        push_vert(rel_pos1, abs_pos1);
                        push_vert(rel_pos2, abs_pos2);
                    }
                }
            }
        }

        // indexes
        let is_basic_face = |v_x, v_y| -> bool {
            match transitions {
                NeighbourSizeTransition::None => true,
                NeighbourSizeTransition::OneSide(side) => !is_face_on_side(side, v_x, v_y),
                NeighbourSizeTransition::TwoSides(side1, side2) => {
                    let on_side1 = is_face_on_side(side1, v_x, v_y);
                    let on_side2 = is_face_on_side(side2, v_x, v_y);

                    !on_side1 && !on_side2
                }
            }
        };

        // populate base indexes
        {
            for v_y in 0..Chunk::size() {
                for v_x in 0..Chunk::size() {
                    let tl = (v_y * (Chunk::size() + 1) + v_x) as u16;
                    let tr = tl + 1;
                    let bl = tl + (Chunk::size() + 1) as u16;
                    let br = bl + 1;

                    if !is_basic_face(v_x, v_y) {
                        continue;
                    }

                    // triangle A
                    indices.push(tl);
                    indices.push(bl);
                    indices.push(br);

                    // triangle B
                    indices.push(br);
                    indices.push(tr);
                    indices.push(tl);
                }
            }
        }

        // populate additional indexes
        {
            let additional_idx_start_side_1: u16 =
                ((Chunk::size() + 1) * (Chunk::size() + 1)) as u16;

            let additional_idx_start_side_2 = additional_idx_start_side_1 + Chunk::size() as u16;

            let mut additional_idx_side1: u16 = 0;
            let mut additional_idx_side2: u16 = 0;

            let mut populate_transition_indexes = |side, tl, tr, bl, br, mid| {
                match side {
                    Side::Top => {
                        // triangle A
                        indices.push(tl);
                        indices.push(bl);
                        indices.push(mid);

                        // triangle B
                        indices.push(mid);
                        indices.push(bl);
                        indices.push(br);

                        // triangle C
                        indices.push(br);
                        indices.push(tr);
                        indices.push(mid);
                    }

                    Side::Bottom => {
                        // triangle A
                        indices.push(tl);
                        indices.push(bl);
                        indices.push(mid);

                        // triangle B
                        indices.push(mid);
                        indices.push(tr);
                        indices.push(tl);

                        // triangle C
                        indices.push(tr);
                        indices.push(mid);
                        indices.push(br);
                    }

                    Side::Left => {
                        // triangle A
                        indices.push(tr);
                        indices.push(tl);
                        indices.push(mid);

                        // triangle B
                        indices.push(mid);
                        indices.push(br);
                        indices.push(tr);

                        // triangle C
                        indices.push(mid);
                        indices.push(bl);
                        indices.push(br);
                    }

                    Side::Right => {
                        // triangle A
                        indices.push(tr);
                        indices.push(tl);
                        indices.push(mid);

                        // triangle B
                        indices.push(mid);
                        indices.push(tl);
                        indices.push(bl);

                        // triangle C
                        indices.push(bl);
                        indices.push(br);
                        indices.push(mid);
                    }
                }
            };

            for v_y in 0..Chunk::size() {
                for v_x in 0..Chunk::size() {
                    let tl = (v_y * (Chunk::size() + 1) + v_x) as u16;
                    let tr = tl + 1;
                    let bl = tl + (Chunk::size() + 1) as u16;
                    let br = bl + 1;

                    if is_basic_face(v_x, v_y) {
                        continue;
                    }

                    let mid_s1 = additional_idx_start_side_1 + additional_idx_side1;
                    let mid_s2 = additional_idx_start_side_2 + additional_idx_side2;

                    match transitions {
                        NeighbourSizeTransition::None => {}
                        NeighbourSizeTransition::OneSide(side) => {
                            populate_transition_indexes(side, tl, tr, bl, br, mid_s1);
                            additional_idx_side1 += 1;
                        }
                        NeighbourSizeTransition::TwoSides(side1, side2) => {
                            let on_side1 = is_face_on_side(side1, v_x, v_y);
                            let on_side2 = is_face_on_side(side2, v_x, v_y);

                            if on_side1 && on_side2 {
                                // corner
                            } else if on_side1 {
                                populate_transition_indexes(side1, tl, tr, bl, br, mid_s1);
                                additional_idx_side1 += 1;
                            } else if on_side2 {
                                populate_transition_indexes(side2, tl, tr, bl, br, mid_s2);
                                additional_idx_side2 += 1;
                            }
                        }
                    };
                }
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
