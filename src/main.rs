use bevy::prelude::*;
use rand::{thread_rng, Rng};

const SNEK_SIZE: f32 = 25.0;

#[derive(Component, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
struct SnekBlock(i32); // Custom type for this? To share it

#[derive(Component, Debug)]
struct Snek {
    length: i32,
    direction: Direction,
}

#[derive(Resource)]
struct MoveTimer(Timer);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn create_snek(mut commands: Commands) {
    let snek_sprite = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.05, 0.05, 0.05),
            custom_size: Some(Vec2::new(SNEK_SIZE, SNEK_SIZE)),
            ..default()
        },
        ..default()
    };
    commands.spawn((
        Snek {
            length: 3,
            direction: Direction::Left,
        },
        Direction::Left,
        snek_sprite,
    ));
}

fn move_snek(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut snek_query: Query<(&mut Snek, &Direction, &mut Transform)>,
    mut snek_block_query: Query<(Entity, &mut SnekBlock)>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for (entity, mut snek_block) in snek_block_query.iter_mut() {
        if snek_block.0 == 1 {
            commands.entity(entity).despawn();
        } else {
            snek_block.0 -= 1;
        }
    }

    for (mut snek, dir, mut transform) in snek_query.iter_mut() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(SNEK_SIZE, SNEK_SIZE)),
                    ..default()
                },
                transform: transform.clone(),
                ..default()
            },
            SnekBlock(snek.length),
        ));

        println!("{snek:?} - {dir:?}");

        if *dir == Direction::Left && snek.direction != Direction::Right {
            snek.direction = Direction::Left;
        }
        if *dir == Direction::Right && snek.direction != Direction::Left {
            snek.direction = Direction::Right;
        }
        if *dir == Direction::Up && snek.direction != Direction::Down {
            snek.direction = Direction::Up;
        }
        if *dir == Direction::Down && snek.direction != Direction::Up {
            snek.direction = Direction::Down;
        }

        match snek.direction {
            Direction::Left => transform.translation.x -= SNEK_SIZE,
            Direction::Right => transform.translation.x += SNEK_SIZE,
            Direction::Up => transform.translation.y += SNEK_SIZE,
            Direction::Down => transform.translation.y -= SNEK_SIZE,
        };
    }
}

fn snek_controls(
    key_input: Res<Input<KeyCode>>,
    mut snek_dir_query: Query<&mut Direction, With<Snek>>,
) {
    // TODO: implement queue fot this not to cancel out fast keystrokes?
    for mut snek_dir in snek_dir_query.iter_mut() {
        if key_input.just_pressed(KeyCode::Up) {
            *snek_dir = Direction::Up
        }
        if key_input.just_pressed(KeyCode::Right) {
            *snek_dir = Direction::Right
        }
        if key_input.just_pressed(KeyCode::Left) {
            *snek_dir = Direction::Left
        }
        if key_input.just_pressed(KeyCode::Down) {
            *snek_dir = Direction::Down
        }
    }
}

fn generate_snacks(mut commands: Commands) {
    let mut rng = thread_rng();
    let x: f32 = rng.gen_range(-250.0..250.0);
    let y: f32 = rng.gen_range(-250.0..250.0);
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(SNEK_SIZE, SNEK_SIZE)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            x - (x % SNEK_SIZE),
            y - (y % SNEK_SIZE),
            0.0,
        )),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MoveTimer(Timer::from_seconds(0.15, TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_startup_system(create_snek)
        .add_startup_system(generate_snacks)
        .add_system(snek_controls)
        .add_system(move_snek)
        .run();
}
