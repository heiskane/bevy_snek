use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::{thread_rng, Rng};

const SNEK_SIZE: f32 = 25.0;
const PROJECTILE_VELOCITY: f32 = 15.0;

#[derive(Component, Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Resource)]
struct UiFont(Handle<Font>);

#[derive(Component)]
struct Snack;

#[derive(Component, Debug)]
struct SnekBlock {
    ttl: i32,
}

#[derive(Component)]
struct Projectile;

#[derive(Component, Default)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component, Default)]
struct Points {
    val: i32,
}

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

fn snek_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut snek_query: Query<(&mut Snek, &Direction, &mut Transform)>,
    mut snek_block_query: Query<(Entity, &mut SnekBlock)>,
) {
    for (mut snek, dir, mut transform) in snek_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            // Meh
            match (dir, snek.direction) {
                (Direction::Right, Direction::Left) => (),
                (Direction::Left, Direction::Right) => (),
                (Direction::Up, Direction::Down) => (),
                (Direction::Down, Direction::Up) => (),
                _ => snek.direction = *dir,
            }

            let block_location = transform.clone();
            match snek.direction {
                Direction::Left => transform.translation.x -= SNEK_SIZE,
                Direction::Right => transform.translation.x += SNEK_SIZE,
                Direction::Up => transform.translation.y += SNEK_SIZE,
                Direction::Down => transform.translation.y -= SNEK_SIZE,
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::new(SNEK_SIZE, SNEK_SIZE)),
                        ..default()
                    },
                    transform: block_location,
                    ..default()
                },
                SnekBlock { ttl: snek.length },
            ));

            for (entity, mut snek_block) in snek_block_query.iter_mut() {
                if snek_block.ttl == 1 {
                    commands.entity(entity).despawn();
                } else {
                    snek_block.ttl -= 1;
                }
            }
        }
    }
}

fn snek_shoot(
    mut commands: Commands,
    key_input: Res<Input<KeyCode>>,
    snek_query: Query<(&Transform, &Snek)>,
) {
    snek_query.for_each(|(loc, snek)| {
        if key_input.just_pressed(KeyCode::Space) {
            let mut projectile_velocity = Velocity::default();
            let mut proj_spawn = loc.clone();
            // TODO: set Velocity relative to snek speed
            match snek.direction {
                Direction::Left => {
                    projectile_velocity.x -= PROJECTILE_VELOCITY;
                    proj_spawn.translation.x -= SNEK_SIZE + 1.0;
                }
                Direction::Right => {
                    projectile_velocity.x += PROJECTILE_VELOCITY;
                    proj_spawn.translation.x += SNEK_SIZE + 1.0;
                }
                Direction::Up => {
                    projectile_velocity.y += PROJECTILE_VELOCITY;
                    proj_spawn.translation.y += SNEK_SIZE + 1.0;
                }
                Direction::Down => {
                    projectile_velocity.y -= PROJECTILE_VELOCITY;
                    proj_spawn.translation.y -= SNEK_SIZE + 1.0;
                }
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::new(SNEK_SIZE, SNEK_SIZE)),
                        ..default()
                    },
                    transform: proj_spawn,
                    ..default()
                },
                Projectile,
                projectile_velocity,
            ));

            println!("fire!");
        }
    });
}

fn despawn_projectiles(
    mut commands: Commands,
    windows: Res<Windows>,
    query: Query<(Entity, &Transform), With<Projectile>>,
) {
    let win_x = windows.primary().width() / 2.0;
    let win_y = windows.primary().height() / 2.0;
    query.for_each(|(e, p_loc)| {
        if p_loc.translation.x > win_x
            || p_loc.translation.x < -win_x
            || p_loc.translation.y > win_y
            || p_loc.translation.y < -win_y
        {
            println!("despawning projectile");
            commands.entity(e).despawn();
        }
    });
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    query.for_each_mut(|(mut transform, vel)| {
        transform.translation.x += vel.x;
        transform.translation.y += vel.y;
    });
}

fn snek_controls(
    key_input: Res<Input<KeyCode>>,
    mut snek_dir_query: Query<&mut Direction, With<Snek>>,
) {
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

fn generate_snacks(mut commands: Commands, snack_query: Query<&Snack>, windows: Res<Windows>) {
    if !snack_query.is_empty() {
        return;
    }

    let mut rng = thread_rng();
    // TODO: Dont spawn inside snek

    let win_x = windows.primary().width() / 2.0;
    let win_y = windows.primary().height() / 2.0;

    let x: f32 = rng.gen_range(-win_x..win_x);
    let y: f32 = rng.gen_range(-win_y..win_y);
    println!("Spawning snacks");
    commands.spawn((
        SpriteBundle {
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
        },
        Snack,
    ));
}

fn eat_snacks(
    mut timer: ResMut<MoveTimer>,
    mut commands: Commands,
    mut snek_block_query: Query<&mut SnekBlock>,
    mut snek_query: Query<(&mut Snek, &Transform, &Sprite)>,
    mut points_query: Query<&mut Points>,
    projectile_query: Query<(&Transform, &Sprite), With<Projectile>>,
    snack_query: Query<(Entity, &Transform, &Sprite), With<Snack>>,
) {
    for (entity, snack, snack_sprite) in snack_query.iter() {
        for (mut snek, snek_loc, snek_sprite) in snek_query.iter_mut() {
            let head_collision = collide(
                snek_loc.translation,
                snek_sprite.custom_size.unwrap(), // Should be fine right?
                snack.translation,
                snack_sprite.custom_size.unwrap(),
            );

            let projectile_collision = projectile_query.iter().any(|(p, s)| {
                let collision = collide(
                    p.translation,
                    s.custom_size.unwrap(),
                    snack.translation,
                    snack_sprite.custom_size.unwrap(),
                );
                collision.is_some()
            });
            if projectile_collision {
                println!("projectile hit");
            }
            if head_collision.is_some() || projectile_collision {
                println!("Eating snack");
                commands.entity(entity).despawn();
                snek.length += 1;
                snek_block_query.for_each_mut(|mut block| {
                    block.ttl += 1;
                });
                let curr_dur = timer.0.duration();
                timer.0.set_duration(curr_dur.mul_f32(0.97));

                points_query.for_each_mut(|mut points| {
                    points.val += 1;
                });
            }
        }
    }
}

fn grim_reaper(
    mut timer: ResMut<MoveTimer>,
    snek_query: Query<(&Transform, &Sprite), With<Snek>>,
    proj_query: Query<(&Transform, &Sprite), With<Projectile>>,
    snek_block_query: Query<(&Transform, &Sprite), With<SnekBlock>>,
) {
    snek_query.for_each(|(snek_loc, snek_sprite)| {
        snek_block_query.for_each(|(block, b_sprite)| {
            let projectile_collision = proj_query.iter().any(|(p, p_sprite)| {
                let collision = collide(
                    p.translation,
                    p_sprite.custom_size.unwrap(),
                    block.translation,
                    b_sprite.custom_size.unwrap(),
                );
                collision.is_some()
            });
            let body_collision = collide(
                snek_loc.translation,
                snek_sprite.custom_size.unwrap(),
                block.translation,
                b_sprite.custom_size.unwrap(),
            )
            .is_some();
            if body_collision || projectile_collision {
                println!("Game Over");
                timer.0.pause();
            }
        })
    });
}

fn load_font(mut commands: Commands, server: Res<AssetServer>) {
    let handle: Handle<Font> = server.load("CascadiaCode-Regular.otf");
    commands.insert_resource(UiFont(handle));
    println!("Inserted font handle");
}

fn display_ui(mut commands: Commands, handle: Res<UiFont>) {
    println!("Creating UI");
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Hello World",
                TextStyle {
                    font: handle.0.clone(),
                    font_size: 60.0,
                    color: Color::BLACK,
                },
            ),
            ..default()
        },
        Points::default(),
    ));
}

fn update_points(mut points_query: Query<(&mut Text, &Points)>, handle: Res<UiFont>) {
    points_query.for_each_mut(|(mut txt, points)| {
        // println!("{:?}", txt.sections);
        *txt = Text::from_section(
            points.val.to_string(),
            TextStyle {
                font: handle.0.clone(),
                font_size: 60.0,
                color: Color::BLACK,
            },
        );
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MoveTimer(Timer::from_seconds(0.15, TimerMode::Repeating)))
        .add_startup_system_to_stage(StartupStage::PreStartup, load_font)
        .add_startup_system(setup)
        .add_startup_system(create_snek)
        .add_startup_system(generate_snacks)
        .add_startup_system(display_ui)
        .add_system(snek_controls)
        .add_system(snek_movement)
        .add_system(generate_snacks)
        .add_system(eat_snacks)
        .add_system(grim_reaper)
        .add_system(update_points)
        .add_system(snek_shoot)
        .add_system(apply_velocity)
        .add_system(despawn_projectiles)
        .run();
}
