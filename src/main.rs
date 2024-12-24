use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resize_constraints: WindowResizeConstraints {
                    max_width: 1000.0,
                    min_height: 1000.0,
                    min_width: 1000.0,
                    max_height: 1000.0,
                },
                ..default()
            }),
            ..default()
        }))
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
