// size of square grid. A grid of size 2 has 4 lights (0,0 - 1,1)
const GRID_SIZE: usize = 1000;

pub fn process_lights(instructions: String) -> u32 {
    let mut grid: Vec<u32> = vec![0; GRID_SIZE * GRID_SIZE];
    for instruction in instructions.split('\n') {
        process_line(&mut grid, instruction.trim());
    }
    count_lights(&grid)
}

fn process_line(grid: &mut Vec<u32>, instruction: &str) {
    let (cmd, x1, y1, x2, y2) = compile_instruction(instruction);
    change_state_of_range(grid, &cmd, x1, y1, x2, y2)
}

fn compile_instruction(instruction: &str) -> (char, u16, u16, u16, u16) {
    const CMD_ON: &str = "turn on";
    const CMD_OFF: &str = "turn off";
    const CMD_TOGGLE: &str = "toggle";
    let compact_instruction = instruction
        .replace(CMD_ON, "+,")
        .replace(CMD_OFF, "-,")
        .replace(CMD_TOGGLE, "x,")
        .replace(" ", "")
        .replace("through", ",");
    let components: Vec<&str> = compact_instruction.split(',').collect();
    (
        components[0].chars().nth(0).unwrap(),
        components[1].parse::<u16>().unwrap(),
        components[2].parse::<u16>().unwrap(),
        components[3].parse::<u16>().unwrap(),
        components[4].parse::<u16>().unwrap(),
    )
}

fn change_state_of_range(grid: &mut Vec<u32>, cmd: &char, x1: u16, y1: u16, x2: u16, y2: u16) {
    for x in x1..x2 + 1 {
        for y in y1..y2 + 1 {
            let old_value = get_value(grid, x, y);
            if cmd == &'+' {
                set_value(grid, x, y, old_value + 1)
            } else if cmd == &'-' {
                set_value(grid, x, y, if old_value == 0 { 0 } else { old_value - 1 })
            } else if cmd == &'x' {
                set_value(grid, x, y, old_value + 2)
            }
        }
    }
}

fn count_lights(grid: &Vec<u32>) -> u32 {
    grid.iter().sum()
}

fn set_value(grid: &mut Vec<u32>, x: u16, y: u16, value: u32) {
    grid[usize::from(y) * GRID_SIZE + usize::from(x)] = value
}

fn get_value(grid: &mut Vec<u32>, x: u16, y: u16) -> u32 {
    grid[usize::from(y) * GRID_SIZE + usize::from(x)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_10x10_on_works() {
        let instructions = "turn on 0,0 through 9,9".to_string();
        assert_eq!(process_lights(instructions), 100);
    }

    #[test]
    fn donut_4x4_works() {
        let instructions = "turn on 0,0 through 3,3
        turn off 1,1 through 2,2"
            .to_string();
        assert_eq!(process_lights(instructions), 12);
    }

    #[test]
    fn donut_4x4_toggle_works() {
        let instructions = "turn on 0,0 through 3,3
        toggle 1,1 through 2,2"
            .to_string();
        assert_eq!(process_lights(instructions), 24);
    }

    #[test]
    fn santas_instructions_work() {
        let instructions = "turn on 887,9 through 959,629
                turn on 454,398 through 844,448
                turn off 539,243 through 559,965
                turn off 370,819 through 676,868
                turn off 145,40 through 370,997
                turn off 301,3 through 808,453
                turn on 351,678 through 951,908
                toggle 720,196 through 897,994
                toggle 831,394 through 904,860"
            .to_string();
        assert_eq!(process_lights(instructions), 539560);
    }
}
