use bevy::{prelude::*, ui::RelativeCursorPosition};
use rand::{thread_rng, Rng};
#[derive(Component, Clone)]
struct Player {
    potion_cooldown: Timer,
}

#[derive(Component)]
struct Particle {
    x: f32,
    y: f32,
    life: Timer,
}

#[derive(Component)]
struct Stone;

#[derive(Component)]
struct Ore {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, stone_setup)
        .add_systems(Update, ore_movment)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(200., 200.)),
                color: Color::Rgba {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.5,
                    alpha: 0.5,
                },
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Stone)
        .insert(RelativeCursorPosition {
            ..Default::default()
        });
}

fn stone_setup(
    mut commands: Commands,
    mut q: Query<(&Transform, &mut Sprite), With<Stone>>,
    cursor: Query<&RelativeCursorPosition>,
    input: Res<Input<MouseButton>>,
) {
    let cursor = cursor.single();
    for (transform, mut stone) in q.iter_mut() {
        if input.just_pressed(MouseButton::Left) {
            stone.custom_size = Some(Vec2::new(400., 400.));
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(10., 10.)),
                        color: Color::Rgba {
                            red: 0.0,
                            green: 0.0,
                            blue: 0.5,
                            alpha: 0.,
                        },
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0., 0., 0.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Ore {
                    x: {
                        let mut rng = rand::thread_rng();
                        transform.translation.x + f32::sin(rng.gen_range(0..360) as f32) * 50.
                    },
                    y: {
                        let mut rng = rand::thread_rng();
                        transform.translation.y + f32::cos(rng.gen_range(0..360) as f32) * 50.
                    },
                });
        }
        let size = stone.custom_size.unwrap();
        stone.custom_size = Some(size + ((Vec2::new(200., 200.) - size) / 30.));
        // stone.custom_size = Some(size - 0.1);
    }
}

fn ore_movment() {}
