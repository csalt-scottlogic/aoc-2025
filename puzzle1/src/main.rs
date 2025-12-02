use anyhow::{anyhow, Context};

fn main() {
    let instructions = load_instructions("input.txt").expect("Couldn't load input");
    let mut dial_pos: i16 = 50;
    let mut answer_counter = 0;
    let mut zeroes: i16;
    for command in instructions {
        (dial_pos, zeroes) = do_command(dial_pos, &command);
        answer_counter += zeroes;
    }
    println!("{}", answer_counter);
}

enum Direction {
    Left,
    Right
}

struct Command {
    direction: Direction,
    amount: i16,
}

fn load_instructions(filename: &str) -> Result<Vec<Command>, anyhow::Error> {
    let mut output: Vec<Command> = Vec::new();
    let instruction_list = std::fs::read_to_string(filename).context("Failed to open file")?;
    for line in instruction_list.lines() {
        let command_char = &line[0..1];
        let amount = line[1..].trim_end();
        let the_direction = (
            match command_char {
                "L" => Ok::<Direction, anyhow::Error>(Direction::Left),
                "R" => Ok(Direction::Right),
                _ => return Err(anyhow!("Invalid input"))
            }
        )?;
        let the_amount = amount.parse::<i16>().context("Failed to parse amount {amount}")?;
        output.push(Command {
            direction: the_direction,
            amount: the_amount,
        });
    }
    Ok(output)
}

fn do_command(start_at: i16, command: &Command) -> (i16, i16) {
    let mut val: i16;
    let mut full_loops = command.amount / 100;
    val = match command.direction {
        Direction::Left => start_at - command.amount % 100,
        Direction::Right => start_at + command.amount % 100,
    };
    if val < 0 {
        val += 100;
        if start_at > 0 {
            full_loops += 1;
        }
    }
    if val > 100 {
        full_loops += val / 100;
    }
    val %= 100;
    if val == 0 {
        full_loops += 1;
    }
    (val, full_loops)
}