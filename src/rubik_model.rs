use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};
use rand::random;

#[derive(Clone, Copy, PartialEq)]
pub enum Movement {
    Le,
    Ri,
    Bo,
    To,
    Ba,
    Fr,
}

impl Movement {
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Movement::Le,
            1 => Movement::Ri,
            2 => Movement::Bo,
            3 => Movement::To,
            4 => Movement::Ba,
            _ => Movement::Fr,
        }
    }

    pub fn rand() -> Self {
        Movement::from_index((random::<u32>() % 6) as u8)
    }

    pub fn to_index(&self) -> u8 {
        match self {
            Movement::Le => 0,
            Movement::Ri => 1,
            Movement::Bo => 2,
            Movement::To => 3,
            Movement::Ba => 4,
            Movement::Fr => 5,
        }
    }

    pub fn axis_index(&self) -> u8 {
        self.to_index() / 2 as u8
    }
}

pub struct RubicItem {
    pub num: u8,
    pub original_position: [i8; 3],
    pub position: [i8; 3],
    pub rotation: Quat,
}

impl RubicItem {
    pub fn colored_faces(&self) -> [bool; 6] {
        [
            self.original_position[0] == -1,
            self.original_position[0] == 1,
            self.original_position[1] == -1,
            self.original_position[1] == 1,
            self.original_position[2] == -1,
            self.original_position[2] == 1,
        ]
    }

    pub fn rotation_sign(&self, movement: Movement) -> i8 {
        let position =  self.position[movement.axis_index() as usize];
        let sign = if movement.to_index() % 2 == 0 { -1 } else { 1 };
        if position == sign {
            return sign;
        }
        return 0
    }
}

pub struct Rubic {
    items: Vec<RubicItem>,
}

impl Rubic {
    pub fn new() -> Self {
        let mut rubic = Rubic { items: Vec::new() };
        let mut num = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    let item = RubicItem {
                        num,
                        original_position: [x, y, z],
                        position: [x, y, z],
                        rotation: Quat::IDENTITY,
                    };
                    num += 1;
                    rubic.items.push(item);
                }
            }
        }
        rubic
    }

    pub fn items(&self) -> &[RubicItem] {
        &self.items
    }

    pub fn make_movement(&mut self, movement: Movement, steps: i8) {
        if steps == 0 {
            return;
        }

        let axis_permutations = [[0, 1, 2], [1, 2, 0], [2, 0, 1]];

        let movement_index = movement.to_index() as usize;
        let position = if movement_index % 2 == 0 { -1 } else { 1 };
        let axis_perm = &axis_permutations[movement_index / 2];

        let mut rotation_axis = Vec3::ZERO;
        rotation_axis[axis_perm[0]] = position as f32;

        let mut steps = steps;
        let steps_sign = if steps > 0 { 1 } else { -1 };

        let change_state = |item: &mut RubicItem| {
            if item.position[axis_perm[0]] == position {
                let a = item.position[axis_perm[1]];
                let b = item.position[axis_perm[2]];
                item.position[axis_perm[1]] = -1 * steps_sign * position * b;
                item.position[axis_perm[2]] = steps_sign * position * a;
                let rotation = Quat::from_axis_angle(rotation_axis, PI / 2.0 * steps_sign as f32);
                item.rotation = rotation * item.rotation;
            }
        };

        while steps != 0 {
            for item in &mut self.items {
                change_state(item);
            }
            steps -= steps_sign;
        }
    }
}
