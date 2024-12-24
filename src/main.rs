use bevy::prelude::*;

const DIMENSIONS: f32 = 800.0;

#[derive(Resource)]
struct Matrix {
    board: [[SpotInstance; 8]; 8],
}

#[derive(Component, Default, Clone, Copy, PartialEq)]
enum EntityColor {
    //Black and White describe piece color
    Black,
    White,
    #[default]
    None,
}

#[derive(Component, Default, Clone, Copy)]
enum Pieces {
    Pawn {
        entity_color: EntityColor,
    },
    Knight {
        entity_color: EntityColor,
    },
    Rook {
        entity_color: EntityColor,
    },
    Bishop {
        entity_color: EntityColor,
    },
    Queen {
        entity_color: EntityColor,
    },
    King {
        entity_color: EntityColor,
    },
    #[default]
    None,
}

#[derive(Component, Default, Clone, Copy)]
struct SpotInstance {
    tile_colour: EntityColor,
    piece_params: Pieces,    //None type if piece does not occupy tile
    matrix_spot: (i32, i32), //uses 1 based counting. Range (inclusive): (1,8)
    tile_pos: (f32, f32),
}

fn board_init(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut matrix: ResMut<Matrix>,
) {
    commands.spawn(Camera2d::default());

    for row in &matrix.board {
        for tile in row {
            let color = if tile.tile_colour == EntityColor::Black {
                Color::BLACK
            } else {
                Color::WHITE
            };

            commands.spawn((
                SpotInstance {
                    tile_colour: tile.tile_colour,
                    piece_params: tile.piece_params,
                    matrix_spot: tile.matrix_spot,
                    tile_pos: tile.tile_pos,
                },
                Mesh2d(meshes.add(Rectangle::new(DIMENSIONS / 8.0, DIMENSIONS / 8.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
                Transform::from_xyz(tile.tile_pos.0, tile.tile_pos.1, 0.0),
            ));
        }
    }
}

fn populate_board() -> [[SpotInstance; 8]; 8] {
    let spot_instance = SpotInstance {
        ..Default::default()
    };
    let mut matrix = [[spot_instance; 8]; 8];
    let mut color_change = EntityColor::White;
    let mut pos_change = (
        -((DIMENSIONS / 2.0) - (DIMENSIONS / 16.0)),
        ((DIMENSIONS / 2.0) - (DIMENSIONS / 16.0)),
    );

    for row in 0..8 {
        for collumn in 0..8 {
            let spot_instance = SpotInstance {
                tile_colour: color_change,
                piece_params: Pieces::None, //FIX LATER
                matrix_spot: (row + 1, collumn + 1),
                tile_pos: pos_change,
            };

            matrix[row as usize][collumn as usize] = spot_instance;
            pos_change.0 += (DIMENSIONS / 8.0);
            color_change = if color_change == EntityColor::White {
                EntityColor::Black
            } else {
                EntityColor::White
            };
        }
        pos_change.1 -= (DIMENSIONS / 8.0);
        pos_change.0 = -((DIMENSIONS / 2.0) - (DIMENSIONS / 16.0));
        color_change = if color_change == EntityColor::White {
            EntityColor::Black
        } else {
            EntityColor::White
        };
    }

    return matrix;
}

fn main() {
    App::new()
        .insert_resource(Matrix {
            board: populate_board(),
        })
        .add_systems(Startup, board_init)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bgame".to_string(),
                resize_constraints: WindowResizeConstraints {
                    max_width: DIMENSIONS,
                    min_height: DIMENSIONS,
                    min_width: DIMENSIONS,
                    max_height: DIMENSIONS,
                },
                ..default()
            }),
            ..default()
        }))
        .run();
}
