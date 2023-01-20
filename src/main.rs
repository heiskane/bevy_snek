use bevy::prelude::*;

// TODO: Global snek size

// TODO: Make direction request to avoit fast movement to go wrong way
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
struct Snek(i32);

#[derive(Resource)]
struct MoveTimer(Timer);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn create_snek(mut commands: Commands) {
    let snek_sprite = SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        ..default()
    };
    commands.spawn((Snek(3), Direction::Left, snek_sprite));
}

fn move_snek(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut snek_query: Query<(&Snek, &Direction, &mut Transform)>,
    mut snek_block_query: Query<(Entity, &mut SnekBlock)>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    for (entity, mut snek_block) in snek_block_query.iter_mut() {
        print!("{snek_block:?}");
        if snek_block.0 == 0 {
            commands.entity(entity).despawn();
        } else {
            snek_block.0 -= 1;
        }
    }

    for (snek, dir, mut transform) in snek_query.iter_mut() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(25.0, 25.0)),
                    ..default()
                },
                transform: transform.clone(),
                ..default()
            },
            SnekBlock(snek.0),
        ));

        println!("{snek:?} - {dir:?}");
        match dir {
            Direction::Left => transform.translation.x -= 25.0,
            Direction::Right => transform.translation.x += 25.0,
            Direction::Up => transform.translation.y += 25.0,
            Direction::Down => transform.translation.y -= 25.0,
        };
    }
}

fn snek_controls(
    key_input: Res<Input<KeyCode>>,
    mut snek_dir_query: Query<&mut Direction, With<Snek>>,
) {
    for mut snek_dir in snek_dir_query.iter_mut() {
        if key_input.just_pressed(KeyCode::Up) && *snek_dir != Direction::Down {
            *snek_dir = Direction::Up
        }
        if key_input.just_pressed(KeyCode::Right) && *snek_dir != Direction::Left {
            *snek_dir = Direction::Right
        }
        if key_input.just_pressed(KeyCode::Left) && *snek_dir != Direction::Right {
            *snek_dir = Direction::Left
        }
        if key_input.just_pressed(KeyCode::Down) && *snek_dir != Direction::Up {
            *snek_dir = Direction::Down
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MoveTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_startup_system(setup)
        .add_startup_system(create_snek)
        .add_system(snek_controls)
        .add_system(move_snek)
        .run();
}
