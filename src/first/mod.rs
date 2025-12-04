use std::fs;
enum Direction {
    Right,
    Left,
}
struct Instruction{
    direction: Direction,
    steps: i32
}

pub fn first_day(){
    let input = parse_input();
    let password = follow_instructions(&input);
    println!("Password: {}", password)
}

fn parse_input() -> Vec<Instruction>{
    let input = fs::read_to_string("inputs/first_input.txt")
        .expect("File input for 01 not found");
    let mut instructions = Vec::new();
    for line in input.lines(){
        let direction: Direction;
        if line[0..1].matches("R").count() == 1 {
            direction = Direction::Right;
        }
        else if line[0..1].matches("L").count() == 1 {
            direction = Direction::Left;
        }
        else {
            panic!("Could not parse line {}", line);
        }
        instructions.push(
            Instruction{
                direction,
                steps: line[1..].parse::<i32>().unwrap()
            }
        );
    }
    instructions
}

fn follow_instructions(instructions: &Vec<Instruction>) -> i32{
    let mut password = 0;
    let mut state = 50;
    for ins in instructions{
        print!("{} | {} \n", ins.steps % 100, ins.steps);
        match ins.direction {

            Direction::Right => {
                state += ins.steps % 100;
                if state > 99 {
                    state = state % 100;
                }
            }
            Direction::Left => {
                state -= ins.steps % 100;
                if state < 0 {
                    state = state % 100;
                }
            }
        }
        if state == 0 {
            password += 1;
        }
    }
    password
}