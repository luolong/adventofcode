use std::fmt::{Display, Formatter};
use std::path::Path;

use common::{read_lines, Solution};
use common::problem::{FailedTo, Problem, problem};

pub struct Day4;

impl Solution for Day4 {
  type Result = u32;
  type Err = Problem;

  fn part1(input: &Path) -> Result<Self::Result, Self::Err> {
    let mut lines = read_lines(input)?;

    let numbers = lines.next().unwrap();
    lines.next(); // skip empty line

    let mut boards = parse_boards(&mut lines);

    let numbers = numbers.split(',')
      .map(|v| v.parse::<u32>().or_failed_to("Parse bingo number"))
      .collect();

    if let Some(first) = play_bingo(&mut boards, numbers).first() {
      Ok(*first)
    } else {
      problem!("Could not calculate result!")
    }
  }

  fn part2(input: &Path) -> Result<Self::Result, Self::Err> {
    let mut lines = read_lines(input)?;

    let numbers = lines.next().unwrap();
    lines.next(); // skip empty line

    let mut boards = parse_boards(&mut lines);

    let numbers = numbers.split(',')
      .map(|v| v.parse::<u32>().or_failed_to("Parse bingo number"))
      .collect();

    if let Some(last) = play_bingo(&mut boards, numbers).last() {
      Ok(*last)
    } else {
      problem!("Could not calculate result!")
    }
  }
}

fn play_bingo(boards: &mut Vec<Board>, numbers: Vec<u32>) -> Vec<u32> {
  let mut winning_scores = Vec::with_capacity(boards.len());
  for number in numbers {
    for board in & mut *boards {
      if let None = board.victory() {
        let result = &mut board.draw(number);

        if let Some(_victory) = result {
          let unmarked_numbers = board.unmarked_numbers();
          let sum: u32 = unmarked_numbers.iter().sum();
          winning_scores.push(sum * number);
        }
      }
    }
  }
  winning_scores
}

#[derive(Debug, Eq, PartialEq)]
struct Board([u32;50], Option<Victory>);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Victory {
  Row(usize),
  Column(usize),
}

impl Display for Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let numbers = &self.0[..25];
    let markings = &self.0[25..];
    for i in 0..25 {
      if i % 5 == 0 { writeln!(f)?; }
      if markings[i] == 0 {
        write!(f, " {:2}  ", numbers[i])?;
      } else {
        write!(f, "({:2}) ", numbers[i])?;
      }
    }
    writeln!(f, "", )
  }
}

fn parse_boards(iter: &mut dyn Iterator<Item=String>) -> Vec<Board> {
  let mut result = Vec::new();

  while let Some(board) = Board::parse_from(iter) {
    iter.next();
    result.push(board)
  }

  result
}

impl Board {

  fn new(slice: &[u32;25]) -> Board {
    let mut board: [u32;50] = [0;50];
    board[..25].copy_from_slice(slice);
    Board(board, None)
  }

  fn parse_from(iter: &mut dyn Iterator<Item=String>) -> Option<Board> {
    let mut board: [u32;25] = [0;25];

    let mut i = 0;
    for _ in 0..5 {
      if let Some(row) = iter.next() {
        let mut numbers = row.split_whitespace();
        for _ in 0..5 {
          if let Some(num) = numbers.next().and_then(|s| s.parse::<u32>().ok()) {
            board[i] = num;
            i += 1;
          } else {
            return None
          }
        }
      } else {
        return None
      }
    }

    Some(Board::new(&board))
  }

  fn unmarked_numbers(&self) -> Vec<u32> {
    let markings = &self.0[25..];
    self.0[..25].iter().enumerate().filter_map(|(i, v)| {
      if markings[i] == 0 { Some(*v) } else { None }
    }).collect::<Vec<u32>>()
  }
}

impl Board {
  fn victory(&self) -> Option<Victory> {
    self.1
  }

  fn draw(&mut self, number: u32) -> Option<Victory> {
    let numbers = &self.0[..25];
    if let Some(index) = numbers.iter().position(|e| *e == number) {
      let markings = &mut self.0[25..];
      markings[index] = 1;


      // Does row match?
      let row_index = index / 5;
      let row_sum: u32 = markings.chunks(5).nth(row_index).map(|chunk| {
        chunk.iter().sum()
      })?;
      if row_sum == 5 {
        self.1 = Some(Victory::Row(row_index));
        return self.1;
      }

      // Does column natch
      let col_index = index % 5;
      let col_sum: u32 = markings.chunks(5).map(|r| r[col_index]).sum();
      if col_sum == 5 {
        self.1 = Some(Victory::Column(col_index));
        return self.1;
      }
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use crate::{Board, play_bingo, Victory};

  const INPUT: &'static str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7
";

  #[test]
  fn it_can_pare_boards() {
    let boards: [Board;3] = [
      Board::new(&[
        22, 13, 17, 11,  0,
        8,  2, 23,  4, 24,
        21,  9, 14, 16,  7,
        6, 10,  3, 18,  5,
        1, 12, 20, 15, 19,
      ]),
      Board::new(&[
        3, 15,  0,  2, 22,
        9, 18, 13, 17,  5,
        19,  8,  7, 25, 23,
        20, 11, 10, 24,  4,
        14, 21, 16, 12,  6,
      ]),
      Board::new(&[
        14, 21, 17, 24,  4,
        10, 16, 15,  9, 19,
        18,  8, 23, 26, 20,
        22, 11, 13,  6,  5,
        2,  0, 12,  3,  7,
      ]),
    ];
    let mut index = 0;
    let mut lines = INPUT.lines().skip(2).map(|s| s.to_string());
    while let Some(board) = Board::parse_from(&mut lines) {
      assert_eq!(board, boards[index]);
      lines.next(); // skip the blank line
      index += 1;
    }
    assert_eq!(index, 3);
  }

  #[test]
  fn it_can_draw_until_victory_row() {
    let mut board = Board::new(&[
      22, 13, 17, 11,  0,
      8,  2, 23,  4, 24,
      21,  9, 14, 16,  7,
      6, 10,  3, 18,  5,
      1, 12, 20, 15, 19,
    ]);

    assert_eq!(board.draw(22), None);
    assert_eq!(board.draw(13), None);
    assert_eq!(board.draw(17), None);
    assert_eq!(board.draw(11), None);
    assert_eq!(board.draw(0), Some(Victory::Row(0)));
  }

  #[test]
  fn it_can_draw_until_victory_col() {
    let mut board = Board::new(&[
      22, 13, 17, 11,  0,
      8,  2, 23,  4, 24,
      21,  9, 14, 16,  7,
      6, 10,  3, 18,  5,
      1, 12, 20, 15, 19,
    ]);

    assert_eq!(board.draw(22), None);
    assert_eq!(board.draw(8), None);
    assert_eq!(board.draw(21), None);
    assert_eq!(board.draw(6), None);
    assert_eq!(board.draw(1), Some(Victory::Column(0)));
  }


  #[test]
  fn it_will_play_bingo() {
    let mut boards: Vec<Board> = vec![
      Board::new(&[
        22, 13, 17, 11,  0,
        8,  2, 23,  4, 24,
        21,  9, 14, 16,  7,
        6, 10,  3, 18,  5,
        1, 12, 20, 15, 19,
      ]),
      Board::new(&[
        3, 15,  0,  2, 22,
        9, 18, 13, 17,  5,
        19,  8,  7, 25, 23,
        20, 11, 10, 24,  4,
        14, 21, 16, 12,  6,
      ]),
      Board::new(&[
        14, 21, 17, 24,  4,
        10, 16, 15,  9, 19,
        18,  8, 23, 26, 20,
        22, 11, 13,  6,  5,
        2,  0, 12,  3,  7,
      ]),
    ];

    let result = play_bingo(&mut boards, vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]);
    assert_eq!(result.first(), Some(&4512u32));
    assert_eq!(result.last(), Some(&1924u32));
  }
}
