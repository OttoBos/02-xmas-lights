// size of grid, which is square and 0-based. A grid of size 1 has therefore 4 lights (0,0 - 1,1)
const GRID_SIZE: usize = 9;
const CMD_ON: &str = "turn on";
const CMD_OFF: &str = "turn off";
const CMD_TOGGLE: &str = "toggle";

pub fn process_lights(instructions: String) -> u8 {
    let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let instructions = "turn on 0,0 through 9,9".to_string();
        assert_eq!(process_lights(instructions), 100);
    }
}
