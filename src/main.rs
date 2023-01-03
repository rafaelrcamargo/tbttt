use std::io;

fn main() {
    // Global
    let stdin = io::stdin();
    print!("\x1B[2J");

    // State
    let mark = vec![" \x1b[32m\x1b[0m ", " \x1b[31m﯇\x1b[0m "];
    let mut grid = vec![
        ["┌", "───", "───", "T", "bTT", "T", "───", "───", "┐"],
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
        // Check if there is a winner
        for i in 0..3 {
            // get the row
            let row = [grid[i * 2 + 2][2], grid[i * 2 + 2][4], grid[i * 2 + 2][6]];

            // get the column
            let col = vec![grid[2][i * 2 + 2], grid[4][i * 2 + 2], grid[6][i * 2 + 2]];

            // get the diagonal
            let diag = vec![grid[2][i * 2 + 2], grid[4][4], grid[6][6 - i * 2]];

            // Check if there is a winner, present in the mark vector
            if row.iter().all(|&x| x == row[2]) && mark.contains(&row[2]) {
                println!("\n{} \x1b[33mWINS!\x1b[0m 🥳", row[2].trim());
                return;
            } else if col.iter().all(|&x| x == col[2]) && mark.contains(&col[2]) {
                println!("\n{} \x1b[33mWINS!\x1b[0m 🥳", col[2].trim());
                return;
            } else if diag.iter().all(|&x| x == diag[2]) && mark.contains(&diag[2]) {
                println!("\n{} \x1b[33mWINS!\x1b[0m 🥳", diag[2].trim());
                return;
            }
        }

        draw(&grid);

        if turn == 9 {
            break;
        }

        // Read a line from stdin and parse to CHAR
        println!("\n{}'s turn: ", mark[turn & 1].trim());
        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        let mut input = input.trim().chars();

        let col = match input.next().unwrap() {
            'a' => 2,
            'b' => 4,
            'c' => 6,
            _ => {
                panic!("Invalid input")
            }
        };

        let row = match input.next().unwrap() {
            '1' => 2,
            '2' => 4,
            '3' => 6,
            _ => {
                panic!("Invalid input")
            }
        };

        // Check if the cell is empty
        if grid[row][col] == "   " {
            grid[row][col] = mark[turn & 1];
        } else {
            println!("Cell is not empty");
            turn -= 1;
        }

        // Next turn
        turn += 1;
    }
}

fn draw(grid: &[[&str; 9]]) {
    print!("\x1B[2J"); // Clear screen

    // Grid
    grid.iter().for_each(|row| {
        row.iter().for_each(|col| print!("{}", col));
        println!()
    });

    println!() // Padding
}
