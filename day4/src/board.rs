#[derive(Clone, Debug)]
pub struct Board {
    grid: [[u8; 5]; 5],
}

impl Board {
    pub fn new(numbers: Vec<u8>) -> Self {
        if numbers.len() != 25 {
            panic!("A bingo grid requires precisely 25 numbers!")
        }

        let grid =
            numbers
                .into_iter()
                .enumerate()
                .fold([[0_u8; 5]; 5], |mut grid, (index, value)| {
                    grid[index / 5][index % 5] = value;
                    grid
                });

        Board { grid }
    }

    fn is_column_complete(&self, numbers: &[u8]) -> bool {
        for index in 0..5 {
            if self.grid.iter().all(|row| numbers.contains(&row[index])) {
                return true;
            }
        }

        false
    }

    fn is_row_complete(&self, numbers: &[u8]) -> bool {
        self.grid
            .iter()
            .any(|row| row.iter().all(|number| numbers.contains(number)))
    }

    pub fn is_won(&self, numbers: &[u8]) -> bool {
        self.is_row_complete(numbers) || self.is_column_complete(numbers)
    }

    pub fn score(&self, numbers: &[u8]) -> u16 {
        let unmarked_number_sum: u16 = self
            .grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|number| !numbers.contains(number))
            .map(|number| *number as u16)
            .sum();

        let final_number = *numbers
            .last()
            .expect("At least five numbers should have been called.")
            as u16;

        unmarked_number_sum * final_number
    }
}
