use regex::Regex;
use std::io;

enum Status {
    Draw,
    Undefined,
    Win(usize),
}

fn main() {
    // Global
    let stdin = io::stdin();
    print!("\x1B[2J");

    // State
    let mark = vec![" \x1b[32m\x1b[0m ", " \x1b[31m\x1b[0m "];
    let mut grid = vec![
        ["┌", "───", "── ", "T", "bTT", "T", " ──", "───", "┐"],
        ["│", "   ", "   ", " ", "   ", " ", "   ", "   ", "│"],
        ["1", "   ", "   ", "║", "   ", "║", "   ", "   ", "│"],
        ["│", "   ", "═══", "╬", "═══", "╬", "═══", "   ", "│"],
        ["2", "   ", "   ", "║", "   ", "║", "   ", "   ", "│"],
        ["│", "   ", "═══", "╬", "═══", "╬", "═══", "   ", "│"],
        ["3", "   ", "   ", "║", "   ", "║", "   ", "   ", "│"],
        ["│", "   ", "   ", " ", "   ", " ", "   ", "   ", "│"],
        ["└", "───", "─A─", "─", "─B─", "─", "─C─", "───", "┘"],
    ];

    let mut turn = 0;
    while turn <= 9 {
        draw_grid(&grid);

        let status = if turn == 9 {
            Some(Status::Draw)
        } else {
            check_status(&grid)
        };

        match status {
            Some(Status::Draw) => {
                println!("That's a draw! 🫥");
                break;
            }
            Some(Status::Win(player)) => {
                println!("Player {} wins! 🎉", mark[player]);
                break;
            }
            _ => {}
        }

        // Read a line from stdin and parse to CHAR
        println!("{}'s turn: ", mark[turn & 1].trim());

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        let (row, col) = read_move(input);

        // Check if the cell is empty
        if grid[row][col] == "   " {
            grid[row][col] = mark[turn & 1];

            // Next turn
            turn += 1;
        } else {
            println!("\n> Input is invalid or cell is not empty!");
            std::thread::sleep(std::time::Duration::from_millis(1000));

            // Previous turn or keep at 0
            turn = turn.saturating_sub(1);
        }
    }
}

fn draw_grid(grid: &[[&str; 9]]) {
    print!("\x1B[2J"); // Clear screen

    // Grid
    grid.iter().for_each(|row| {
        row.iter().for_each(|col| print!("{}", col));
        println!()
    });

    println!() // Padding
}

fn read_move(input: String) -> (usize, usize) {
    let input = input.split_at(2).0.to_lowercase();

    // Check if input is valid
    let re = Regex::new(r"^[abc][123]").unwrap();
    if !re.is_match(&input) {
        return (0, 0);
    }

    let coord: Vec<usize> = input
        .trim()
        .split("")
        .map(|pos| match pos {
            "a" | "1" => 2usize,
            "b" | "2" => 4usize,
            "c" | "3" => 6usize,
            _ => 0usize,
        })
        .collect();

    (coord[2], coord[1])
}

fn check_status(grid: &[[&str; 9]]) -> Option<Status> {
    let mark = vec![" \x1b[32m\x1b[0m ", " \x1b[31m\x1b[0m "];
    let mut status = Some(Status::Undefined);

    for i in 0..3 {
        let row = [grid[i * 2 + 2][2], grid[i * 2 + 2][4], grid[i * 2 + 2][6]];
        let col = vec![grid[2][i * 2 + 2], grid[4][i * 2 + 2], grid[6][i * 2 + 2]];
        let diag = vec![grid[2][i * 2 + 2], grid[4][4], grid[6][6 - i * 2]];

        if (row.iter().all(|&x| x == row[2]) && mark.contains(&row[2]))
            || (col.iter().all(|&x| x == col[2]) && mark.contains(&col[2]))
            || (diag.iter().all(|&x| x == diag[2]) && mark.contains(&diag[2]))
        {
            status = Some(Status::Win(
                mark.iter().position(|&x| x == diag[2]).unwrap(),
            ));
        }
    }

    status
}
