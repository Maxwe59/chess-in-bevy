use bevy::{input::keyboard::KeyboardInput, prelude::*};

const DIMENSIONS: f32 = 800.0;

#[derive(Resource)]
struct Matrix {
    board: [[SpotInstance; 8]; 8],
    selected_piece: (usize, usize),
    tile_selector: (usize, usize),
}

#[derive(Component, Default, Clone, Copy, PartialEq)]
enum EntityColor {
    //Black and White describe piece color
    Black,
    White,
    #[default]
    None,
}

#[derive(Component, Default, Clone, Copy, PartialEq)]
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

/*
tile_colour -> Black or White
piece_params -> piece occupying current tile, or none
matrix_spot -> [0,0] is top left spot, [7,7] is bottom right spot.
tile_pos -> real coordinates where tile is being rendered
*/
#[derive(Component, Default, Clone, Copy)]
struct SpotInstance {
    tile_colour: EntityColor,
    piece_params: Pieces,        //None type if piece does not occupy tile
    matrix_spot: (usize, usize), //0 based counting. represents matrix position
    tile_pos: (f32, f32),
}

#[derive(Component, PartialEq)]
struct TileInstance {
    matrix_spot: (usize, usize),
}

fn populate_board() -> [[SpotInstance; 8]; 8] {
    let mut current_piece = Pieces::None;
    let spot_instance = SpotInstance {
        ..Default::default()
    };
    let mut matrix = [[spot_instance; 8]; 8];
    let mut color_change = EntityColor::White;
    //position starts top left (negative x, positive y )
    let mut pos_change = (
        -((DIMENSIONS / 2.0) - (DIMENSIONS / 16.0)),
        ((DIMENSIONS / 2.0) - (DIMENSIONS / 16.0)),
    );

    for row in 0..8 {
        for collumn in 0..8 {
            match row {
                6 => {
                    current_piece = Pieces::Pawn {
                        entity_color: EntityColor::White,
                    }
                }
                1 => {
                    current_piece = Pieces::Pawn {
                        entity_color: EntityColor::Black,
                    }
                }
                0 => match collumn {
                    0 => {
                        current_piece = Pieces::Rook {
                            entity_color: EntityColor::Black,
                        }
                    }
                    1 => {
                        current_piece = Pieces::Knight {
                            entity_color: EntityColor::Black,
                        }
                    }
                    2 => {
                        current_piece = Pieces::Bishop {
                            entity_color: EntityColor::Black,
                        }
                    }
                    3 => {
                        current_piece = Pieces::Queen {
                            entity_color: EntityColor::Black,
                        }
                    }
                    4 => {
                        current_piece = Pieces::King {
                            entity_color: EntityColor::Black,
                        }
                    }
                    5 => {
                        current_piece = Pieces::Bishop {
                            entity_color: EntityColor::Black,
                        }
                    }
                    6 => {
                        current_piece = Pieces::Knight {
                            entity_color: EntityColor::Black,
                        }
                    }
                    7 => {
                        current_piece = Pieces::Rook {
                            entity_color: EntityColor::Black,
                        }
                    }
                    _ => (),
                },
                7 => match collumn {
                    0 => {
                        current_piece = Pieces::Rook {
                            entity_color: EntityColor::White,
                        }
                    }
                    1 => {
                        current_piece = Pieces::Knight {
                            entity_color: EntityColor::White,
                        }
                    }
                    2 => {
                        current_piece = Pieces::Bishop {
                            entity_color: EntityColor::White,
                        }
                    }
                    3 => {
                        current_piece = Pieces::Queen {
                            entity_color: EntityColor::White,
                        }
                    }
                    4 => {
                        current_piece = Pieces::King {
                            entity_color: EntityColor::White,
                        }
                    }
                    5 => {
                        current_piece = Pieces::Bishop {
                            entity_color: EntityColor::White,
                        }
                    }
                    6 => {
                        current_piece = Pieces::Knight {
                            entity_color: EntityColor::White,
                        }
                    }
                    7 => {
                        current_piece = Pieces::Rook {
                            entity_color: EntityColor::White,
                        }
                    }
                    _ => (),
                },
                _ => (current_piece = Pieces::None),
            }

            let spot_instance = SpotInstance {
                tile_colour: color_change,
                piece_params: current_piece,
                matrix_spot: (row, collumn),
                tile_pos: pos_change,
            };

            matrix[row as usize][collumn as usize] = spot_instance;
            pos_change.0 += DIMENSIONS / 8.0;
            color_change = if color_change == EntityColor::White {
                EntityColor::Black
            } else {
                EntityColor::White
            };
        }
        pos_change.1 -= DIMENSIONS / 8.0;
        pos_change.0 = -((DIMENSIONS / 2.0) - (DIMENSIONS / 16.0));
        color_change = if color_change == EntityColor::White {
            EntityColor::Black
        } else {
            EntityColor::White
        };
    }

    return matrix;
}

fn board_init(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    matrix: ResMut<Matrix>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 0.0)));

    for (row_index, row) in matrix.board.iter().enumerate() {
        for (collumn_index, tile) in row.iter().enumerate() {
            let color = if tile.tile_colour == EntityColor::Black {
                Color::BLACK
            } else {
                Color::WHITE
            };

            //spawn tile pattern (8x8 grid)
            commands.spawn((
                TileInstance {
                    matrix_spot: (row_index, collumn_index),
                },
                Mesh2d(meshes.add(Rectangle::new(DIMENSIONS / 8.0, DIMENSIONS / 8.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(color))),
                Transform::from_xyz(tile.tile_pos.0, tile.tile_pos.1, 1.0),
            ));

            if tile.piece_params != Pieces::None {
                let mut path = "path".to_string();
                match tile.piece_params {
                    //ugly ass code
                    Pieces::Pawn {
                        entity_color: EntityColor::Black,
                    } => (path = "black_pawn.png".to_string()),
                    Pieces::Pawn {
                        entity_color: EntityColor::White,
                    } => (path = "white_pawn.png".to_string()),
                    Pieces::King {
                        entity_color: EntityColor::White,
                    } => (path = "white_king.png".to_string()),
                    Pieces::King {
                        entity_color: EntityColor::Black,
                    } => (path = "black_king.png".to_string()),
                    Pieces::Queen {
                        entity_color: EntityColor::White,
                    } => (path = "white_queen.png".to_string()),
                    Pieces::Queen {
                        entity_color: EntityColor::Black,
                    } => (path = "black_queen.png".to_string()),
                    Pieces::Rook {
                        entity_color: EntityColor::Black,
                    } => (path = "black_rook.png".to_string()),
                    Pieces::Rook {
                        entity_color: EntityColor::White,
                    } => (path = "white_rook.png".to_string()),
                    Pieces::Bishop {
                        entity_color: EntityColor::Black,
                    } => (path = "black_bishop.png".to_string()),
                    Pieces::Bishop {
                        entity_color: EntityColor::White,
                    } => (path = "white_bishop.png".to_string()),
                    Pieces::Knight {
                        entity_color: EntityColor::Black,
                    } => (path = "black_knight.png".to_string()),
                    Pieces::Knight {
                        entity_color: EntityColor::White,
                    } => (path = "white_knight.png".to_string()),

                    _ => (),
                }
                commands.spawn((
                    SpotInstance {
                        tile_colour: tile.tile_colour,
                        piece_params: tile.piece_params,
                        matrix_spot: tile.matrix_spot,
                        tile_pos: tile.tile_pos,
                    },
                    Sprite::from_image(asset_server.load(path)),
                    Transform {
                        translation: Vec3 {
                            x: tile.tile_pos.0,
                            y: tile.tile_pos.1,
                            z: 5.0,
                        },
                        scale: Vec3 {
                            x: 0.45,
                            y: 0.45,
                            z: 5.0,
                        },
                        ..Default::default()
                    },
                ));
            }
        }
    }
}

fn drag_piece(
    read_test: Res<ButtonInput<MouseButton>>,
    mut inputs: EventReader<CursorMoved>,
    mut transform: Query<&mut Transform, With<Sprite>>,
) {
    for val in inputs.read() {
        let mousepos: Vec2 = val.position;
        for mut sprite in transform.iter_mut() {
            sprite.translation.x = mousepos.x;
            sprite.translation.y = mousepos.y;
        }
    }
    let test: Res<'_, ButtonInput<MouseButton>> = read_test;
}

fn keyboard_input_system(
    mut query_squares: Query<(&mut Transform, &TileInstance)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut matrix: ResMut<Matrix>,
) {
    let original_pos = matrix.tile_selector;

    if keyboard_input.just_pressed(KeyCode::KeyD) && (matrix.tile_selector.1<=6){
        matrix.tile_selector.1 += 1;
    }
    if keyboard_input.just_pressed(KeyCode::KeyA)&& (matrix.tile_selector.1>=1){
        matrix.tile_selector.1 -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::KeyW)&& (matrix.tile_selector.0>=1){
        matrix.tile_selector.0 -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::KeyS)&& (matrix.tile_selector.0<=6){
        matrix.tile_selector.0 += 1;
    }

    for mut square in query_squares.iter_mut() {
        if square.1.matrix_spot == original_pos{
            square.0.scale = Vec3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            };
        }
        if matrix.tile_selector == square.1.matrix_spot {
                square.0.scale = Vec3 {
                    x: 0.85,
                    y: 0.85,
                    z: 0.05,
                };
            
        }
    }

    /*

    if keyboard_input.pressed(KeyCode::KeyA) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_released(KeyCode::KeyA) {
        info!("'A' just released");
    }

     */
}


fn update_board(
    //target: (usize, usize),
    //new_pos: (usize, usize),
    query: Query<&SpotInstance>,
    mut matrix: ResMut<Matrix>,
    mut transform: Query<&mut Transform, With<SpotInstance>>,
    spot_query: Query<(Entity, &SpotInstance)>,
    mut commands: Commands,
) {
    // EFFECTIVE INPUTS:
    //(0,0 represents top left corner of board). first number represents row, second represents collumn
    //for example (6,7) gives the piece in the 6th row, 7th collumn in the 2d array
    let target = (1, 1);
    let new_pos = (2, 1);

    //despawn piece
    let new_pos_piece = matrix.board[new_pos.0][new_pos.1].piece_params;
    if new_pos_piece != Pieces::None {
        for (entity, component) in spot_query.iter() {
            if component.matrix_spot == new_pos {
                commands.entity(entity).despawn();
            }
        }
    }

    //rest of code to not remove
    let target_piece = matrix.board[target.0][target.1].piece_params;
    for entity in query.iter() {
        if entity.matrix_spot == target {
            matrix.board[target.0][target.1].piece_params = Pieces::None; //update resource matrix
        }
        if entity.matrix_spot == new_pos {
            matrix.board[new_pos.0][new_pos.1].piece_params = target_piece; //update resource matrix
        }
    }
    //query through pieces, find target piece that matches description
    let target_xy = matrix.board[target.0][target.1].tile_pos;
    let new_pos_xy = matrix.board[new_pos.0][new_pos.1].tile_pos;
    for mut entity in transform.iter_mut() {
        if (entity.translation.x == target_xy.0) && (entity.translation.y == target_xy.1) {
            entity.translation = Vec3 {
                x: new_pos_xy.0,
                y: new_pos_xy.1,
                z: 5.0,
            };
        }
    }
}

fn main() {
    App::new()
        .insert_resource(Matrix {
            board: populate_board(),
            selected_piece: (0, 0),
            tile_selector: (0, 0),
        })
        .add_systems(Startup, board_init)
        .add_systems(Update, keyboard_input_system)
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
