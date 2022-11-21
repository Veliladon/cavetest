use crate::{*, tile::Tile};

pub struct Map{
    pub length: usize,
    pub height: usize,
    pub tiles: Vec<Tile>,

}

impl Map{
    pub fn new(length: usize, height: usize) -> Self {
        Self {length, height,
            tiles:vec![Tile::Water; length * height],
        }
    }
}



pub fn map_idx(x: usize, y:usize) -> usize {
    (y * MAP_COL) + x
}

pub fn generate_map() -> Vec<i32> {
    //let columns = winsize.w as i32 / (SPRITE_WIDTH as i32 * SPRITE_SCALE as i32);
    //let rows = winsize.h as i32 / (SPRITE_WIDTH as i32 * SPRITE_SCALE as i32);
    // let mut map: Vec::new(<TileData>);

    let mut map = vec![16; MAP_COL * MAP_ROW];
    // generate_river(&mut map);
    seed_land(&mut map);
    for i in 0..5 {
        smooth_land(&mut map);
        println!("Finished Smooth Iteration #{}", i);
    }
    add_corners(&mut map);
    add_edges(&mut map);
    map
}

pub fn seed_land(map: &mut [i32]) {
    let mut rng = thread_rng();
    for y in 0..MAP_ROW {
        for x in 0..MAP_COL {
            let decision = rng.gen_range(0..100);
            if x == 0 || x == MAP_COL - 1 || y == 0 || y == MAP_ROW - 1 {
                map[y * MAP_COL + x] = 16;
            } else if decision <= LAND_PROB {
                map[y * MAP_COL + x] = 23;
            }
            if decision > LAND_PROB {}
        }
    }
}
 
fn smooth_land(map: &mut Vec<i32>) {
    let mut smoothed_map = map.clone();
    for y in 1..(MAP_ROW - 1) {
        for x in 1..(MAP_COL - 1) {
            let water_count = count_neighbors(map, x as i32, y as i32, 16);
            //println!("Land Count for {}, {}: {}", x, y, land_count);
            if water_count > 4 {
                smoothed_map[y * MAP_COL + x] = 16;
            } else {
                smoothed_map[y * MAP_COL + x] = 23;
            }
        }
    }
    *map = smoothed_map;
}

fn count_neighbors(map: &mut Vec<i32>, x: i32, y: i32, tileindex: i32) -> i32 {
    let mut neighbors = 0;
    let columns = MAP_COL as i32;
    for iy in -1..=1 {
        for ix in -1..=1 {
            if !(ix == 0 && iy == 0) && map[(((y + iy) * columns + x + ix) as usize)] == tileindex {
                neighbors += 1;
            }
        }
    }

    neighbors
}

fn add_corners(map: &mut Vec<i32>) {
    tl_corners(map);
    tr_corners(map);
    br_corners(map);
    bl_corners(map);
    tl_corners(map);
    fix_single_width_tiles(map);

    /* for y in 1..(MAP_ROW - 1) {
        for x in 1..(MAP_COL - 1) {
            /* if map[y * MAP_COL + x] == 23
                && (map[y * MAP_COL + x - 1] == 16 && map[(y - 1) * MAP_COL + x] == 16)
            {
                map[y * MAP_COL + x] = 5;
                if map[(y + 1) * MAP_COL + x] == 16 {
                    map[(y + 1) * MAP_COL + x] = 23;
                }
                if map[y * MAP_COL + x + 1] == 16 {
                    map[y * MAP_COL + x + 1] = 23;
                }
            } */
            /* if map[y * MAP_COL + x] == 23
                && (map[y * MAP_COL + x + 1] == 16 && map[(y - 1) * MAP_COL + x] == 16)
            {
                map[y * MAP_COL + x] = 7;
                if map[(y + 1) * MAP_COL + x] == 16 {
                    map[(y + 1) * MAP_COL + x] = 23;
                }
            } */
            /*if map[y * MAP_COL + x] == 23
                && ((map[y * MAP_COL + x - 1] == 16) && (map[(y + 1) * MAP_COL + x] == 16))
            {
                map[y * MAP_COL + x] = 39;
                if map[(y * MAP_COL + x + 1)] == 16 {
                    map[(y * MAP_COL + x + 1)] = 23;
                }
            }*/
            /*if map[y * MAP_COL + x] == 23
                && (map[y * MAP_COL + x + 1] == 16 && map[(y + 1) * MAP_COL + x] == 16)
            {
                map[y * MAP_COL + x] = 41;
            }*/
        }
    } */
}

fn tl_corners(map: &mut Vec<i32>) {
    for y in 1..(MAP_ROW - 1) {
        for x in 1..(MAP_COL - 1) {
            if (map[y * MAP_COL + x] == 23)
                && (map[y * MAP_COL + x - 1]) == 16
                && (map[(y - 1) * MAP_COL + x] == 16)
            {
                map[y * MAP_COL + x] = 5;
                if map[(y + 1) * MAP_COL + x] == 16 {
                    map[(y + 1) * MAP_COL + x] = 23;
                }
                if map[y * MAP_COL + x + 1] == 16 {
                    map[y * MAP_COL + x + 1] = 23;
                }
            }
        }
    }
}

fn tr_corners(map: &mut Vec<i32>) {
    for y in 1..(MAP_ROW - 1) {
        for x in 1..(MAP_COL - 1) {
            if (map[y * MAP_COL + x] == 23)
                && (map[y * MAP_COL + x + 1] == 16)
                && (map[(y - 1) * MAP_COL + x] == 16)
            {
                map[y * MAP_COL + x] = 7;
                if map[(y + 1) * MAP_COL + x] == 16 {
                    map[(y + 1) * MAP_COL + x] = 23;
                }
            }
        }
    }
}

fn bl_corners(map: &mut Vec<i32>) {
    for y in (1..(MAP_ROW - 1)).rev() {
        for x in 1..(MAP_COL - 1) {
            if (map[y * MAP_COL + x] == 23)
                && (map[y * MAP_COL + x - 1] == 16)
                && (map[(y + 1) * MAP_COL + x] == 16)
            {
                map[y * MAP_COL + x] = 39;
                if map[(y * MAP_COL + x - 1)] == 16 {
                    map[((y - 1) * MAP_COL + x)] = 23;
                    println!("Generating Tile via BL at {}, {}", x, y);
                }
            }
        }
    }
}

fn br_corners(map: &mut Vec<i32>) {
    for y in (1..(MAP_ROW - 1)).rev() {
        for x in (1..(MAP_COL - 1)).rev() {
            if (map[y * MAP_COL + x] == 23)
                && (map[y * MAP_COL + x + 1] == 16)
                && (map[(y + 1) * MAP_COL + x] == 16)
            {
                map[y * MAP_COL + x] = 41;
                if map[(y * MAP_COL + x - 1)] == 16 {
                    map[(y * MAP_COL + x - 1)] = 23;
                    println!("Generating Tile via BR at {}, {}", x, y);
                }
            }
        }
    }
}

fn fix_single_width_tiles(map: &mut Vec<i32>) {
    for y in 1..(MAP_ROW - 1) {
        for x in 1..(MAP_COL - 1) {
            if (map[y * MAP_COL + x] == 23)
                && (map[(y - 1) * MAP_COL + x] == 16)
                && (map[(y + 1) * MAP_COL + x] == 16)
            {
                map[(y + 1) * MAP_COL + x] = 23;
                println!("Generating Tile via FSWT at {}, {}", x, y);
                println!("{}, {} was {}", x, y - 1, map[(y - 1) + MAP_COL + x],);
                println!("{}, {} was {}", x, y + 1, map[(y + 1) + MAP_COL + x],);
            }
        }
    }
}

fn add_edges(map: &mut Vec<i32>) {
    for y in 1..(MAP_ROW - 1) {
        for x in 1..(MAP_COL - 1) {
            if (map[y * MAP_COL + x] == 23) && map[y * MAP_COL + x - 1] == 16 {
                map[y * MAP_COL + x] = 22;
            }
            if (map[y * MAP_COL + x] == 23) && map[y * MAP_COL + x + 1] == 16 {
                map[y * MAP_COL + x] = 24;
            }
            if (map[y * MAP_COL + x] == 23) && map[(y - 1) * MAP_COL + x] == 16 {
                map[y * MAP_COL + x] = 6;
            }
            if (map[y * MAP_COL + x] == 23) && map[(y + 1) * MAP_COL + x] == 16 {
                map[y * MAP_COL + x] = 40;
            }
        }
    }
}

fn inner_corners(map: &mut Vec<i32>) {
    for y in 1..(MAP_ROW - 1) {
        for x in 1..(MAP_COL - 1) {
            if (map[y * MAP_COL + x] == 23)
                && (map[y * MAP_COL + x - 1] == 22)
                && (map[(y - 1) * MAP_COL + x] == 6)
            {
                map[y * MAP_COL + x] = 26;
            } else if (map[y * MAP_COL + x] == 23)
                && (map[y * MAP_COL + x - 1] == 22)
                && (map[(y - 1) * MAP_COL + x] == 6)
            {
            }
        }
    }
}
