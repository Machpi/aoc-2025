struct Mino {
    squares: usize,
    _shape: Vec<Vec<bool>>,
}
struct Board {
    width: usize,
    height: usize,
    minos: Vec<usize>,
}

fn get_mino(lines: Vec<&str>) -> Mino {
    let mut squares = 0;
    let _shape = lines
        .iter()
        .map(
            |line| line
                .chars()
                .map(|c| {
                    if c == '#' {
                        squares += 1;
                        true
                    } else {
                        false
                    }
                }) .collect::<Vec<bool>>(),
        )
        .collect::<Vec<Vec<bool>>>();
    Mino { squares, _shape }
}

fn get_minos(lines: Vec<&str>) -> Vec<Mino> {
    let mut minos = Vec::new();
    let lines_clean = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| *line)
        .enumerate()
        .filter(|(i, _)| i % 4 != 0)
        .map(|(_, line)| line)
        .collect::<Vec<&str>>();
    for chunk in lines_clean.chunks(3) {
        minos.push(get_mino(chunk.to_vec()));
    }
    minos
}

fn get_board(line: &str) -> Board {
    let parts = line.split(':').collect::<Vec<&str>>();
    let dims = parts[0].split('x').collect::<Vec<&str>>();
    let width = dims[0].parse::<usize>().unwrap();
    let height = dims[1].parse::<usize>().unwrap();
    let minos = parts[1]
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    Board { width, height, minos }
}

fn get_boards(lines: Vec<&str>) -> Vec<Board> {
    lines
        .iter()
        .map(|line| get_board(line))
        .collect::<Vec<Board>>()
}

fn get_input() -> (Vec<Mino>, Vec<Board>) {
    let lines = include_str!("../../inputs/day12.txt")
        .lines()
        .collect::<Vec<&str>>();
    let lines_minos = lines[..29].to_vec();
    let lines_boards = lines[30..].to_vec();
    (get_minos(lines_minos), get_boards(lines_boards))
}

fn is_possible(board: &Board, minos: &Vec<Mino>) -> bool {
    let total_squares: usize = board.minos
        .iter()
        .enumerate()
        .map(|(i, &c)| c*minos[i].squares)
        .sum();
    total_squares <= board.width * board.height
}

fn main() {
    let (minos, boards) = get_input();
    let mut count = 0;
    for board in boards.iter() {
        if is_possible(board, &minos) {
            count += 1;
        }
    }
    println!("{}", count);
}
