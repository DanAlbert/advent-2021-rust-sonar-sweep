use advent::columniterator::ColumnIterator;

const MY_INPUT: &str = include_str!("inputs/problem4.txt");

#[cfg(test)]
const SAMPLE_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

#[derive(Debug, PartialEq)]
struct BingoBoard {
    cells: [[u8; 5]; 5],
    marks: [[bool; 5]; 5],
}

impl BingoBoard {
    fn new(cells: [[u8; 5]; 5]) -> BingoBoard {
        BingoBoard {
            cells,
            marks: [[false; 5]; 5],
        }
    }

    fn from_lines<'a>(lines: &mut impl Iterator<Item = &'a str>) -> BingoBoard {
        let mut cells = [[0u8; 5]; 5];
        // Clippy seems to be just wrong here. The index is important, and I'm not iterating over
        // the whole iterator, just the first 5 rows. next_chunk would make this go away, but is
        // still experimental. Rewriting to iterated and break after 5 is not more readable IMO.
        #[allow(clippy::needless_range_loop)]
        for i in 0..5 {
            let line = lines.next().unwrap();
            let row: [u8; 5] = line
                .split_whitespace()
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            cells[i] = row;
        }
        BingoBoard::new(cells)
    }

    fn mark(&mut self, number: u8) {
        for x in 0..self.cells.len() {
            for y in 0..self.cells[x].len() {
                if self.cells[x][y] == number {
                    self.marks[x][y] = true
                }
            }
        }
    }

    fn won(&self) -> bool {
        self.winning_set().is_some()
    }

    fn winning_set(&self) -> Option<[u8; 5]> {
        for row in 0..self.marks.len() {
            if self.marks[row].iter().all(|v| *v) {
                return Some(self.cells[row]);
            }
        }
        for (idx, column) in ColumnIterator::new(self.marks).enumerate() {
            if column.iter().all(|v| *v) {
                return Some(ColumnIterator::new(self.cells).collect::<Vec<[u8; 5]>>()[idx]);
            }
        }
        None
    }

    fn sum_of_unmarked_cells(&self) -> u32 {
        let mut sum = 0u32;
        for x in 0..self.marks.len() {
            for y in 0..self.marks[x].len() {
                if !self.marks[x][y] {
                    sum += self.cells[x][y] as u32;
                }
            }
        }
        sum
    }
}

#[test]
fn test_board_from_lines() {
    assert_eq!(
        [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ],
        BingoBoard::from_lines(
            &mut "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19"
                .lines()
        )
        .cells
    )
}

#[test]
fn test_mark() {
    let mut cells = [[0u8; 5]; 5];
    cells[0][0] = 1;
    let mut board = BingoBoard::new(cells);
    board.mark(1u8);
    for row in 0..board.cells.len() {
        for column in 0..board.cells[row].len() {
            if row == 0 && column == 0 {
                assert!(board.marks[row][column]);
            } else {
                assert!(!board.marks[row][column]);
            }
        }
    }
}

struct Game {
    boards: Vec<BingoBoard>,
    winners: Vec<(usize, u8)>,
}

impl Game {
    fn new(boards: Vec<BingoBoard>) -> Game {
        Game {
            boards,
            winners: vec![],
        }
    }

    fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Result<(Vec<u8>, Game), String> {
        let mut stripped = lines.filter(|l| !l.is_empty()).peekable();
        let draws_line = stripped.next().unwrap();
        let draws = draws_line
            .split(',')
            .map(|s| {
                s.parse::<u8>()
                    .map_err(|e| format!("Failed to parse game draw {}: {}", s, e))
            })
            .collect::<Result<Vec<u8>, String>>()?;
        let mut boards = Vec::<BingoBoard>::new();
        loop {
            if stripped.peek().is_none() {
                break;
            }
            boards.push(BingoBoard::from_lines(&mut stripped));
        }
        Ok((draws, Game::new(boards)))
    }

    fn call(&mut self, number: u8) {
        for (i, board) in self.boards.iter_mut().enumerate() {
            if !board.won() {
                board.mark(number);
                if board.won() {
                    self.winners.push((i, number))
                }
            }
        }
    }

    fn winner(&self) -> Option<usize> {
        Some(self.winners.first()?.0)
    }

    fn winner_last_draw(&self) -> Option<u8> {
        Some(self.winners.first()?.1)
    }

    fn biggest_loser(&self) -> Option<usize> {
        Some(self.winners.last()?.0)
    }

    fn biggest_loser_last_draw(&self) -> Option<u8> {
        Some(self.winners.last()?.1)
    }

    #[cfg(test)]
    fn winning_set(&self) -> Option<[u8; 5]> {
        Some(self.boards[self.winner()?].winning_set().unwrap())
    }

    fn winning_score(&self) -> Option<u32> {
        let last_draw = self.winner_last_draw()? as u32;
        let sum = self.boards[self.winner()?].sum_of_unmarked_cells();
        Some(sum * last_draw)
    }

    fn loserest_score(&self) -> Option<u32> {
        let last_draw = self.biggest_loser_last_draw()? as u32;
        let sum = self.boards[self.biggest_loser()?].sum_of_unmarked_cells();
        Some(sum * last_draw)
    }
}

#[test]
fn test_game_from_lines() {
    let (draws, game) = Game::from_lines(&mut SAMPLE_INPUT.lines()).unwrap();

    assert_eq!(
        draws,
        vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1
        ]
    );

    assert_eq!(
        game.boards,
        vec![
            BingoBoard::new([
                [22, 13, 17, 11, 0],
                [8, 2, 23, 4, 24],
                [21, 9, 14, 16, 7],
                [6, 10, 3, 18, 5],
                [1, 12, 20, 15, 19],
            ]),
            BingoBoard::new([
                [3, 15, 0, 2, 22],
                [9, 18, 13, 17, 5],
                [19, 8, 7, 25, 23],
                [20, 11, 10, 24, 4],
                [14, 21, 16, 12, 6],
            ]),
            BingoBoard::new([
                [14, 21, 17, 24, 4],
                [10, 16, 15, 9, 19],
                [18, 8, 23, 26, 20],
                [22, 11, 13, 6, 5],
                [2, 0, 12, 3, 7],
            ]),
        ]
    )
}

#[test]
fn test_sample() {
    let (draws, mut game) = Game::from_lines(&mut SAMPLE_INPUT.lines()).unwrap();

    for draw in &draws[..5] {
        game.call(*draw);
    }

    assert!(game.winner().is_none());

    for draw in &draws[5..11] {
        game.call(*draw);
    }

    assert!(game.winner().is_none());

    game.call(draws[11]);

    // The result here is the index of the winning board. Player 3 wins, which is board "2".
    assert_eq!(Some(2), game.winner());
    assert_eq!(Some([14, 21, 17, 24, 4]), game.winning_set());
    assert_eq!(Some(4512), game.winning_score());

    for draw in &draws[12..] {
        game.call(*draw);
    }

    assert_eq!(Some(1), game.biggest_loser());
    assert_eq!(Some(1924), game.loserest_score());
}

fn main() {
    match Game::from_lines(MY_INPUT.lines()) {
        Ok((draws, mut game)) => {
            for draw in draws {
                game.call(draw);
            }

            println!("Winning score: {}", game.winning_score().unwrap());
            println!("Winning score: {}", game.loserest_score().unwrap());
        }
        Err(error) => {
            eprintln!("{}", error);
        }
    }
}
