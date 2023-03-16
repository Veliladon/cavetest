mod map;
mod tile;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::{thread_rng, Rng};

use bevy_inspector_egui::quick::WorldInspectorPlugin;


const BACKGROUND_SHEET: &str = "tilesheet.png";
const SPRITE_WIDTH: f32 = 64.;
const SPRITE_SCALE: f32 = 1.;
const MAP_COL: usize = 20;
const MAP_ROW: usize = 12;
const LAND_PROB: usize = 55;

pub struct WinSize{
    pub w: f32,
    pub h: f32,
}

pub struct TileData{
    pub north: Option<TileProb>,
    pub south: Option<TileProb>,
    pub east: Option<TileProb>,
    pub west: Option<TileProb>,
}
pub struct TileProb{
    pub index: i32,
    pub probability: f32,
}



#[derive(Resource)]
struct GameAssets{
    background: Handle<TextureAtlas>,
}


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.5)))
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_plugin(WorldInspectorPlugin::default())
    .add_startup_system(setup_system.in_base_set(StartupSet::Startup))
    .add_startup_system(draw_background.in_base_set(StartupSet::PostStartup))
    .add_startup_system(set_camera.in_base_set(StartupSet::PostStartup))

    .run();
    
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // mut windows: ResMut<Windows>,
   
){
    commands.spawn(Camera2dBundle::default());
    
    //let window = windows.get_primary_mut().unwrap();
    //let win_size = (window.width(), window.height());
    //println!("Got window size {} {}", window.width(), window.height());
    //let win_size = WinSize {w: win_w, h: win_h};
    //commands.insert_resource(win_size);

    //let window = windows.get_primary_mut().unwrap();
    //let (win_w, win_h) = (window.width(), window.height());


    // position window
    // window.set_position(IVec2::new(2780, 4900));

    // add win_size resource
    //let win_size = WinSize {w: win_w, h: win_h };
    //commands.insert_resource(win_size);

    let texture_handle = asset_server.load(BACKGROUND_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITE_WIDTH, SPRITE_WIDTH), 17, 12, None, None);
    let background = texture_atlases.add(texture_atlas);
    let game_assets = GameAssets {
        background,
    };

    commands.insert_resource(game_assets);
    


}

fn set_camera(
    mut camera_query: Query<&mut Transform, With<Camera>>) {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = 608.;
        camera_transform.translation.y = -328.;
    }



/* fn generate_river(
    map: &mut [i32]){
        let mut rng = thread_rng();
        let x = rng.gen_range(0..MAP_COL);
        let y = rng.gen_range(0..MAP_ROW);
        println!("River Column: {}", x);
        println!("River Row: {}", y);
        
        for i in 0..MAP_ROW{
            map[i*MAP_COL+x] = 48;
        }
        for i in 0..MAP_COL{
            map[y*MAP_COL+i] = 12;
        }
        map[y * MAP_COL + x] = 95;
} */

fn draw_background(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // winsize: Res<WinSize>,
){
    //let mut tiles = Vec::new();
    
    let map_background = vec![203; MAP_ROW * MAP_COL];
    let map = map::generate_map();
    let mut map_entities = Vec::new();

    let window = window_query.get_single().unwrap();

    println!("Drawing Background");
    let columns = window.width() as i32 / (SPRITE_WIDTH as i32 * SPRITE_SCALE as i32);
    let rows = window.height() as i32 / (SPRITE_WIDTH as i32 * SPRITE_SCALE as i32);
    println!("Window Size: {}", window.width());
    println!("Detected {} columns", columns);
    println!("Detected {} rows", rows);
    
    

    
    for y in 0..MAP_ROW{
        for x in 0..MAP_COL {
            let tile = commands.spawn(SpriteSheetBundle {
                texture_atlas: game_assets.background.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 * SPRITE_WIDTH * SPRITE_SCALE, -(y as f32 * SPRITE_WIDTH * SPRITE_SCALE), 1.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(map_background[y * MAP_COL + x].try_into().unwrap()),
                ..Default::default()
                
            }).id();
            map_entities.push(tile);
            //tiles.push(tile);
            //println!("Added sprite");
                
    
            
        }

    } 
    
    for y in 0..MAP_ROW{
        for x in 0..MAP_COL {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: game_assets.background.clone(),
                transform: Transform {
                    translation: Vec3::new(x as f32 * SPRITE_WIDTH * SPRITE_SCALE, -(y as f32 * SPRITE_WIDTH * SPRITE_SCALE), 2.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(map[y * MAP_COL + x].try_into().unwrap()),
                ..Default::default()
                
            });
            //tiles.push(tile);
            //println!("Added sprite");
                
    
            
        }

    } 
  
    
}