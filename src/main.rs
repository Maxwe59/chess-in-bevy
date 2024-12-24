use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::WHITE))),
    ));
}

#[derive(Component)]
struct Player;
