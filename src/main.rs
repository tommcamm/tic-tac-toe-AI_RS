use std::io;
use std::io::Write;
use std::cmp::{max, min};


#[derive(Eq, PartialEq)]
enum WinStatus {
    Human,
    Computer,
    Draw,
    InProgress,
}

fn main() {
    println!("Welcome to tic-tac-toe game against AI!");
    // Initialize a new board
    let mut board = [[0u8; 3]; 3];


    loop {
        print_board(board);
        board = player_move(board);
        board = computer_move(board);
        match check_win(board) {
            WinStatus::Human  => {
                println!("[WIN] Congratulations! you beat minimax!");
                break;
            },
            WinStatus::Computer => {
                println!("[LOSE] minimax won...");
                break;
            },
            WinStatus::Draw => {
                println!("Draw!");
                break;
            },
            _ => {},
        }

    }
}

fn computer_move(mut board: [[u8; 3]; 3]) -> [[u8; 3]; 3] {
    // Here we apply the minimax algorithm
    let mut best_val = -1000;
    let mut best_move :(i32, i32) = (-1,-1);

    for i in 0..3 {
        for j in 0..3 {
            if board[i][j] == 0 {
                board[i][j] = 2;

                let move_val = minimax(board, 0, false);

                board[i][j] = 0;

                if move_val > best_val {
                    best_move = (i as i32, j as i32);
                    best_val = move_val;
                }
            }
        }
    }

    // We try to make the best move
    if best_move.0 != -1 {
        board[best_move.0 as usize][best_move.1 as usize] = 2;
    }

    board
}



fn minimax(mut board: [[u8; 3]; 3], depth: i32, is_max: bool) -> i32{
    let score = evaluate_score(board);

    // If Minimizer (human) or Maximizer (computer) won the game return his score.
    if score == 10 || score == -10 {
        return score;
    }

    // If there are no moves left and no winnder is a Draw.
    if check_win(board) == WinStatus::Draw {
        return 0;
    }

    // Maximizer move
    if is_max {
        let mut best = -1000;

        for i in 0..3 {
            for j in 0..3 {

                // If there is a empty spot we make the move
                if board[i][j] == 0 {
                    board[i][j] = 2;

                    best = max(best, minimax(board, depth + 1, !is_max));

                    // Undo the move
                    board[i][j] = 0;
                }
            }
        }
        best
    } else {
        // Minimizer move
        let mut best = 1000;

        for i in 0..3 {
            for j in 0..3 {
                if board[i][j] == 0 {
                    board[i][j] = 1;

                    best = min(best, minimax(board, depth + 1, !is_max));

                    // Undo the move
                    board[i][j] = 0;
                }
            }
        }
        best
    }
}

fn evaluate_score (board: [[u8; 3]; 3]) -> i32 {
    return match check_win(board) {
        WinStatus::Human => -10,
        WinStatus::Computer => 10,
        _ => 0,
    }
}

fn check_win(board: [[u8; 3]; 3]) -> WinStatus {

    // Check if human wins
    if board[0][0] == 1 && board[1][1] == 1 && board[2][2] == 1 {
        return WinStatus::Human;
    } else if board[0][2] == 1 && board[1][1] == 1 && board[2][0] == 1 {
        return WinStatus::Human;
    } else if board[0][0] == 1 && board[0][1] == 1 && board[0][2] == 1 {
        return WinStatus::Human;
    } else if board[1][0] == 1 && board[1][1] == 1 && board[1][2] == 1 {
        return WinStatus::Human;
    } else if board[2][0] == 1 && board[2][1] == 1 && board[2][2] == 1 {
        return WinStatus::Human;
    } else if board[0][0] == 1 && board[1][0] == 1 && board[2][0] == 1 {
        return WinStatus::Human;
    } else if board[0][1] == 1 && board[1][1] == 1 && board[2][1] == 1 {
        return WinStatus::Human;
    } else if board[0][2] == 1 && board[1][2] == 1 && board[2][2] == 1 {
        return WinStatus::Human;
    }

    // Check if computer wins
    if board[0][0] == 2 && board[1][1] == 2 && board[2][2] == 2 {
        return WinStatus::Computer;
    } else if board[0][2] == 2 && board[1][1] == 2 && board[2][0] == 2 {
        return WinStatus::Computer;
    } else if board[0][0] == 2 && board[0][1] == 2 && board[0][2] == 2 {
        return WinStatus::Computer;
    } else if board[1][0] == 2 && board[1][1] == 2 && board[1][2] == 2 {
        return WinStatus::Computer;
    } else if board[2][0] == 2 && board[2][1] == 2 && board[2][2] == 2 {
        return WinStatus::Computer;
    } else if board[0][0] == 2 && board[1][0] == 2 && board[2][0] == 2 {
        return WinStatus::Computer;
    } else if board[0][1] == 2 && board[1][1] == 2 && board[2][1] == 2 {
        return WinStatus::Computer;
    } else if board[0][2] == 2 && board[1][2] == 2 && board[2][2] == 2 {
        return WinStatus::Computer;
    }

    // Check if it isn't a Draw
    for (row_pos, row) in board.iter().enumerate() {
        for (col_pos, _) in row.iter().enumerate() {
            if board[row_pos][col_pos] == 0 {
                return WinStatus::InProgress;
            }
        }
    }

    WinStatus::Draw
}

fn player_move(board: [[u8; 3]; 3]) -> [[u8; 3]; 3]{
    println!();
    println!("== PLAYER TURN ==");
    println!("Please select position to apply the cross (from zero)");
    let mut updated_board = board;
    loop {
        let mut x_input = String::new();
        let mut y_input = String::new();

        // X - ROW INPUT
        println!();
        print!("Row[0-2] >> ");
        let _ = io::stdout().flush();
        let _ =io::stdin()
            .read_line(&mut x_input);

        let x_input: usize = match x_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Y - COLUMN INPUT
        println!();
        print!("Column[0-2] >> ");
        let _ = io::stdout().flush();
        let _ = io::stdin()
            .read_line(&mut y_input);
        let y_input: usize = match y_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if board[x_input][y_input] == 0 {
            updated_board[x_input][y_input] = 1;
            break;
        } else {
            println!("Position not valid! please try again")
        }
    }
    updated_board
}

fn print_board(board: [[u8; 3]; 3]) {
    let mut iter = 0;
    for row in board.iter() {
        if iter != 0 {
            println!("-----");
        }
        let mut el = 0;
        for _ in row.iter() {
            if board[iter][el] == 1 {
                print!("X");
            } else if board[iter][el] == 2 {
                print!("O");
            } else {
                print!(" ");
            }

            if el != 2 {
                print!("|")
            } else if el == 2 {
                println!();
            }

            el = el + 1;
        }

        iter = iter + 1;
    }
}
