use spin::Mutex;

#[derive(Clone, Debug, Copy)]
pub enum BlockType {
    Air,
    Grass,
    Dirt,
    Stone,
    Wood,
    Leaves,
    Water,
}

pub struct CraftBlocksWorld {
    pub player_x: f32,
    pub player_y: f32,
    pub player_z: f32,
    pub player_yaw: f32,
    pub selected_block: BlockType,
    pub world_grid: [[[BlockType; 16]; 16]; 16],
}

pub static GAME_WORLD: Mutex<Option<CraftBlocksWorld>> = Mutex::new(None);

pub fn run_craftblocks() {
    let mut world_guard = GAME_WORLD.lock();
    if world_guard.is_none() {
        let mut grid = [[[BlockType::Air; 16]; 16]; 16];
        for x in 0..16 {
            for z in 0..16 {
                grid[x][0][z] = BlockType::Stone;
                grid[x][1][z] = BlockType::Dirt;
                grid[x][2][z] = BlockType::Grass;
            }
        }
        // Build a tree
        grid[8][3][8] = BlockType::Wood;
        grid[8][4][8] = BlockType::Wood;
        grid[8][5][8] = BlockType::Leaves;

        *world_guard = Some(CraftBlocksWorld {
            player_x: 8.0,
            player_y: 3.5,
            player_z: 2.0,
            player_yaw: 0.0,
            selected_block: BlockType::Grass,
            world_grid: grid,
        });
    }
}

pub fn place_block(x: usize, y: usize, z: usize, block: BlockType) {
    let mut guard = GAME_WORLD.lock();
    if let Some(ref mut world) = *guard {
        if x < 16 && y < 16 && z < 16 {
            world.world_grid[x][y][z] = block;
        }
    }
}

pub fn break_block(x: usize, y: usize, z: usize) {
    place_block(x, y, z, BlockType::Air);
}
