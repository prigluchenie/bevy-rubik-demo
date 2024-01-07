use std::mem::swap;

use bevy::render::{
    color::Color,
    mesh::{Indices, Mesh},
    render_resource::PrimitiveTopology,
};

pub struct BeveledCube {
    pub radius: f32,
    pub bevel: f32,
    pub color_left: Color,
    pub color_right: Color,
    pub color_bottom: Color,
    pub color_top: Color,
    pub color_back: Color,
    pub color_front: Color,
    pub color_bevel: Color,
}

impl Default for BeveledCube {
    fn default() -> Self {
        BeveledCube {
            radius: 1.0,
            bevel: 0.2,
            color_left: Color::BLACK,
            color_right: Color::BLACK,
            color_bottom: Color::BLACK,
            color_top: Color::BLACK,
            color_back: Color::BLACK,
            color_front: Color::BLACK,
            color_bevel: Color::BLACK,
        }
    }
}

impl From<BeveledCube> for Mesh {
    fn from(s: BeveledCube) -> Self {
        let bevel_color = s.color_bevel.as_rgba_f32();
        let face_colors = [
            s.color_front.as_rgba_f32(),
            s.color_right.as_rgba_f32(),
            s.color_top.as_rgba_f32(),
            s.color_back.as_rgba_f32(),
            s.color_left.as_rgba_f32(),
            s.color_bottom.as_rgba_f32(),
        ];

        let mut vert = Vec::new();
        let mut ind = Vec::new();
        let mut colors = Vec::new();
        let mut normals = Vec::new();

        let face_rad = s.radius * (1.0 - s.bevel);

        for face in 0..6 {
            let ci = vert.len() as u32;
            let mut v: [f32; 3] = [0.0; 3];
            let mut i = face % 3;
            let mut j = (face + 1) % 3;
            let k = (face + 2) % 3;
            if face >= 3 {
                swap(&mut i, &mut j);
            }
            v[k] = (1.0 - ((face / 3) as f32) * 2.0) * s.radius;
            v[i] = face_rad;
            v[j] = face_rad;
            vert.push(v);
            v[i] = -face_rad;
            vert.push(v);
            v[j] = -face_rad;
            vert.push(v);
            v[i] = face_rad;
            vert.push(v);

            let mut n: [f32; 3] = [0.0; 3];
            n[k] = 1.0 - ((face / 3) as f32) * 2.0;
            for _ in 0..4 {
                colors.push(face_colors[face]);
                normals.push(n);
            }
            for fi in [0, 1, 3, 3, 1, 2] {
                ind.push(ci + fi);
            }
        }

        let corner_order = [
            [0, 4, 8],    // 0 1 2
            [1, 11, 16],  // 3 4 5
            [2, 19, 21],  // 6 7 8
            [3, 20, 5],   // 9 10 11
            [6, 23, 13],  // 12 13 14
            [7, 12, 9],   // 15 16 17
            [10, 15, 17], // 18 19 20
            [14, 22, 18], // 21 22 23
        ];
        for corner_vertices in &corner_order {
            for corener_vertex in corner_vertices {
                ind.push(vert.len() as u32);
                vert.push(vert[*corener_vertex]);
                normals.push(normals[*corener_vertex]);
                colors.push(bevel_color);
            }
        }

        let edge_order = [
            [0, 2, 4, 3],
            [2, 1, 15, 17],
            [1, 0, 9, 11],
            [3, 5, 7, 6],
            [10, 9, 6, 8],
            [11, 10, 13, 12],
            [14, 13, 22, 21],
            [23, 22, 8, 7],
            [5, 4, 18, 20],
            [19, 18, 17, 16],
            [12, 14, 16, 15],
            [20, 19, 21, 23],
        ];
        for edge_vertices in &edge_order {
            for i in [0, 1, 2] {
                ind.push(24 + (*edge_vertices)[i]);
            }
            for i in [0, 2, 3] {
                ind.push(24 + (*edge_vertices)[i]);
            }
        }

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vert)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals.to_vec())
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors.to_vec())
            .with_indices(Some(Indices::U32(ind)))
    }
}
