mod beveled_cube;
mod rubik_model;
mod style;

use beveled_cube::BeveledCube;
use rand::random;
use rubik_model::{Movement, Rubic, RubicItem};

use std::f32::consts::PI;

use bevy::prelude::*;

const ITEM_RADIUS: f32 = 1.0;
const SHOW_SOLVED_SECONDS: f32 = 3.0;
const MIN_SHUFFLING_STEPS: u32 = 4;
const MAX_SHUFFLING_STEPS: u32 = 15;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rubik Demo".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (make_movement, rotate_cube))
        .run();
}

struct MovementState {
    movement: Movement,
    expected_steps: i8,
    current_steps: f32,
}

struct HistoryItem {
    movement: Movement,
    steps: i8,
}

enum SolvingState {
    Movement(MovementState),
    ShowSolvedSince(f32),
}

#[derive(Component)]
struct RubicSolution {
    rubic: Rubic,
    solving_state: SolvingState,
    history: Vec<HistoryItem>,
    shuffling_steps: u32,
}

#[derive(Component)]
struct ShapeItem {
    num: u8,
}

#[derive(Component)]
struct Rotator;

fn make_cube_item(item: &RubicItem) -> BeveledCube {
    let mut cube = BeveledCube {
        radius: ITEM_RADIUS,
        bevel: style::BEVEL_FRACTION,
        color_bevel: style::COLOR_BEVEL,
        ..default()
    };
    let cf = item.colored_faces();
    for (i, color) in [
        &mut cube.color_left,
        &mut cube.color_right,
        &mut cube.color_bottom,
        &mut cube.color_top,
        &mut cube.color_back,
        &mut cube.color_front,
    ]
    .into_iter()
    .enumerate()
    {
        if cf[i] {
            *color = *style::COLORS[i];
        }
    }
    cube
}

fn delta_by_offset(offset: i8) -> f32 {
    offset as f32 * ITEM_RADIUS * 2.0
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rubic = Rubic::new();

    commands
        .spawn((PbrBundle { ..default() }, Rotator))
        .with_children(|parent| {
            for item in rubic.items() {
                let mesh = meshes.add(make_cube_item(item).into());
                parent.spawn((
                    PbrBundle {
                        mesh,
                        material: materials.add(StandardMaterial { ..default() }),
                        transform: Transform::from_xyz(
                            delta_by_offset(item.position[0]),
                            delta_by_offset(item.position[1]),
                            delta_by_offset(item.position[2]),
                        )
                        .with_rotation(item.rotation),
                        ..default()
                    },
                    ShapeItem { num: item.num },
                ));
            }
        });

    commands.spawn(RubicSolution {
        rubic,
        solving_state: SolvingState::ShowSolvedSince(0.0),
        history: Vec::new(),
        shuffling_steps: 0,
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 4.0, 8.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 12., 16.0).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..default()
    });
}

fn make_movement(
    mut query_solution: Query<&mut RubicSolution>,
    mut query: Query<(&mut Transform, &ShapeItem), With<ShapeItem>>,
    time: Res<Time>,
) {
    let RubicSolution {
        rubic,
        solving_state,
        history,
        shuffling_steps,
    } = &mut *query_solution.get_single_mut().unwrap();

    let t = time.elapsed_seconds();
    if let SolvingState::ShowSolvedSince(since) = solving_state {
        if t < *since + SHOW_SOLVED_SECONDS {
            return;
        }
        *solving_state = SolvingState::Movement(MovementState {
            movement: Movement::rand(),
            expected_steps: if random::<bool>() { -1 } else { 1 },
            current_steps: 0.0,
        });
        *shuffling_steps = random::<u32>() % (MAX_SHUFFLING_STEPS + 1 - MIN_SHUFFLING_STEPS) + MIN_SHUFFLING_STEPS;
    }
    let SolvingState::Movement(state) = solving_state else {
        unreachable!()
    };

    let dt = time.delta_seconds();
    let omega = dt * 1.0;

    state.current_steps += omega * state.expected_steps as f32;
    let last_expected_steps = state.expected_steps;
    if state.current_steps.abs() >= state.expected_steps.abs() as f32 {
        rubic.make_movement(state.movement, state.expected_steps);
        if *shuffling_steps > 0 {
            history.push(HistoryItem {
                movement: state.movement,
                steps: state.expected_steps,
            });
            *shuffling_steps -= 1;
        }
        state.current_steps = 0.0;
        state.expected_steps = 0;
    }

    for (mut transform, item) in &mut query {
        let item = &rubic.items()[item.num as usize];
        transform.translation.x = delta_by_offset(item.position[0]);
        transform.translation.y = delta_by_offset(item.position[1]);
        transform.translation.z = delta_by_offset(item.position[2]);
        transform.rotation = item.rotation;

        if state.expected_steps == 0 {
            continue;
        }
        let rotation_sign = item.rotation_sign(state.movement);
        if rotation_sign == 0 {
            continue;
        }
        let mut axis = Vec3::ZERO;
        axis[state.movement.axis_index() as usize] = 1.0;
        let mut matrix = Mat4::from_axis_angle(
            axis,
            PI / 2.0 * (rotation_sign as f32) * state.current_steps,
        );
        matrix *= transform.compute_matrix();
        *transform = Transform::from_matrix(matrix);
    }

    if state.expected_steps != 0 {
        return;
    }

    if *shuffling_steps > 0 {
        loop {
            let next_movement = Movement::rand();
            let next_expected_steps = if random::<bool>() { -1 } else { 1 };
            if next_movement != state.movement || next_expected_steps != -last_expected_steps {
                *solving_state = SolvingState::Movement(MovementState {
                    movement: next_movement,
                    expected_steps: next_expected_steps,
                    current_steps: 0.0,
                });
                break;
            }
        }
    } else {
        match history.pop() {
            None => *solving_state = SolvingState::ShowSolvedSince(t),
            Some(HistoryItem{movement, steps}) =>
            *solving_state = SolvingState::Movement(MovementState {
                movement,
                expected_steps: -steps,
                current_steps: 0.0,
            })
        }
    }
}

fn rotate_cube(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    let mut transform = query.get_single_mut().unwrap();

    let t = time.elapsed_seconds();
    let v = time.delta_seconds() * 1.0;

    let drx = (t * 0.7 + 0.1).sin();
    let dry = (t * 1.5 + 0.5).sin();
    let drz = (t * 0.6 - 0.2).sin();

    transform.rotate_local_x(v * drx);
    transform.rotate_local_y(v * dry);
    transform.rotate_local_z(v * drz);
}
