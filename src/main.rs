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

#[derive(Component, Default, Clone, Copy)]
struct SpotInstance {
    tile_colour: EntityColor,
    piece_params: Pieces,        //None type if piece does not occupy tile
    matrix_spot: (usize, usize), //uses 1 based counting. Range (inclusive): (1,8)
    tile_pos: (f32, f32),
}

fn board_init(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut matrix: ResMut<Matrix>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2d::default(), Transform::from_xyz(0.0, 0.0, 0.0)));

    for row in &matrix.board {
        for tile in row {
            let color = if tile.tile_colour == EntityColor::Black {
                Color::BLACK
            } else {
                Color::WHITE
            };

            commands.spawn((
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

fn populate_board() -> [[SpotInstance; 8]; 8] {
    let mut current_piece = Pieces::None;
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
                matrix_spot: (row + 1, collumn + 1),
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

fn drag_piece(mut inputs: EventReader<CursorMoved>) {
    for val in inputs.read() {
        let mousepos = val.position;
        print!("x: {:?}, y: {:?}\n", mousepos.x, mousepos.y);
    }
}

fn test_resource(
    mut matrix: ResMut<Matrix>,
    mut commands: Commands,
    query: Query<Entity, With<SpotInstance>>,
    asset_server: Res<AssetServer>,
) {
    matrix.board[5][5].piece_params = Pieces::Bishop {
        entity_color: EntityColor::Black,
    };
    commands.spawn((
        Sprite::from_image(asset_server.load("black_bishop.png")),
        Transform {
            translation: Vec3 {
                x: matrix.board[5][5].tile_pos.0,
                y: matrix.board[5][5].tile_pos.1,
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
/*
fn update_board(piece1: (x,y), new_pos: (x,y))->mutate matrix resource

resource -> matrix of board, containing data on every single spot
every single tile -> component containing information on piece condition
update_board -> system used to update the piece movement. updates the data in matrix resource,

*/

fn update_board(
    //target: (usize, usize),
    //new_pos: (usize, usize),
    query: Query<&SpotInstance>,
    mut matrix: ResMut<Matrix>,
    mut transform: Query<&mut Transform, With<SpotInstance>>,
    spot_query: Query<(Entity, &SpotInstance)>,
    mut commands: Commands
) {
    //Target is matrix position of piece to move to new_pos.
    //!!!COUNTING STARTS FROM 1-8. USE 1-8 BASE COUNTING!!!
    // EFFECTIVE INPUTS: 
    //IMPORTANT FIX: Fix new_pos x and y dimensions being reversed. 
    let target = (1, 1);
    let new_pos = (7, 1);


    //despawn piece
    let new_pos_piece = matrix.board[new_pos.0-1][new_pos.1-1].piece_params;
    if new_pos_piece!=Pieces::None{
        for (entity, component) in spot_query.iter(){
            if component.matrix_spot == new_pos{
                commands.entity(entity).despawn(); 
            }
        }
        
    }
    
    //rest of code to not remove
    let target_piece = matrix.board[target.0-1][target.1-1].piece_params;
    for entity in query.iter() {
        if entity.matrix_spot == target {
            matrix.board[target.0-1][target.1-1].piece_params = Pieces::None; //update resource matrix
        }
        if entity.matrix_spot == new_pos {
            matrix.board[new_pos.0-1][new_pos.1-1].piece_params = target_piece; //update resource matrix
        }
    }
    //query through pieces, find target piece that matches description
    let target_xy = matrix.board[target.0-1][target.1-1].tile_pos;
    let new_pos_xy = matrix.board[new_pos.0-1][new_pos.1-1].tile_pos;
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
        })
        .add_systems(Startup, board_init)
        .add_systems(FixedUpdate, update_board)
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
