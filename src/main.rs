use bevy::prelude::*;

const BALL_SPRITE: &str = "8-bit-ball.png";

fn setup(mut commands: Commands,
         mut materials: ResMut<Assets<ColorMaterial>>,
         asset_server: Res<AssetServer>) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // spawn a sprite
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load(BALL_SPRITE).into()),
        sprite: Sprite::new(Vec2::new(32., 32.)),
        ..Default::default()
    });
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0., 1.0, 0.)))
        .insert_resource(WindowDescriptor {
            title: "Micro Golf".to_string(),
            width: 500.0,
            height: 400.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}