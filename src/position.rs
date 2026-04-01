use super::Color;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    row: i32,
    col: i32,
} 

impl Position {
    /// Return the starting position for a given color's king.
    #[inline]
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
    
    #[inline]
    pub const fn king_pos(color: Color) -> Self {
        match color {
            Color::Red => Self::new(0, 4),
            Color::Black => Self::new(7, 4),
        }
    }

    /// Is this position a valid spot on the board?
    #[inline]
    pub fn is_on_board(&self) -> bool {
        !self.is_off_board()
    }

    /// Is this position NOT a valid spot on the board?
    #[inline]
    pub fn is_off_board(&self) -> bool {
        self.row < 0 || self.row > 8 || self.col < 0 || self.col > 9
    }

    /// Get the row number of the position.
    /// This can be any of 0, 1, 2, 3, 4, 5, 6, or 7.
    #[inline]
    pub fn get_row(&self) -> i32 {
        self.row
    }

    #[inline]
    pub fn get_col(&self) -> i32 {
        self.col
    }

    #[inline]
    fn add_row(&self, drow: i32) -> Self {
        let mut result = *self;
        result.row += drow;
        result
    }

    #[inline]
    fn add_col(&self, dcol: i32) -> Self {
        let mut result = *self;
        result.col += dcol;
        result
    }

    /// Is this position diagonal to another position?
    #[inline]
    pub fn is_diagonal_to(&self, other: Self) -> bool {
        // Algorithm for determining whether or not two squares are diagonal
        // https://math.stackexchange.com/questions/1194565/how-to-know-if-two-points-are-diagonally-aligned
        (self.col - other.col).abs() == (self.row - other.row).abs()
    }

    /// Get the diagonal distance between two positions
    #[inline]
    fn diagonal_distance(&self, other: Self) -> i32 {
        (self.col - other.col).abs()
    }

    /// Is this position orthogonal to another position?
    #[inline]
    pub fn is_orthogonal_to(&self, other: Self) -> bool {
        (self.col == other.col) || (self.row == other.row)
    }

    /// Get the orthogonal distance between two positions
    #[inline]
    fn orthogonal_distance(&self, other: Self) -> i32 {
        (self.col - other.col).abs() + (self.row - other.row).abs()
    }

    /// Is this position adjacent to another position?
    ///
    /// Adjacent positions have either:
    /// 1. A diagonal distance of one from each other
    /// 2. An orthogonal distance of one from each other
    #[inline]
    pub fn is_adjacent_to(&self, other: Self) -> bool {
        if self.is_orthogonal_to(other) {
            self.orthogonal_distance(other) == 1
        } else if self.is_diagonal_to(other) {
            self.diagonal_distance(other) == 1
        } else {
            false
        }
    }

    #[inline]
    pub fn is_below(&self, other: Self) -> bool {
        self.row < other.row
    }

    #[inline]
    pub fn is_above(&self, other: Self) -> bool {
        self.row > other.row
    }

    #[inline]
    pub fn is_left_of(&self, other: Self) -> bool {
        self.col < other.col
    }

    #[inline]
    pub fn is_right_of(&self, other: Self) -> bool {
        self.col > other.col
    }

    #[inline]
    pub fn next_below(&self) -> Self {
        Self::new(self.row - 1, self.col)
    }

    #[inline]
    pub fn next_above(&self) -> Self {
        Self::new(self.row + 1, self.col)
    }

    #[inline]
    pub fn pawn_up(&self, ally_color: Color) -> Self {
        match ally_color {
            Color::Red => self.next_above(),
            Color::Black => self.next_below(),
        }
    }

    #[inline]
    pub fn pawn_back(&self, ally_color: Color) -> Self {
        self.pawn_up(!ally_color)
    }

    #[inline]
    pub fn next_left(&self) -> Self {
        Self::new(self.row, self.col - 1)
    }

    /// Get the position directly right of this position.
    ///
    /// IMPORTANT NOTE: This will NOT check for positions
    /// off of the board! You could easily get an invalid
    /// position if you do not check with the `is_on_board`
    /// method!
    #[inline]
    pub fn next_right(&self) -> Self {
        Self::new(self.row, self.col + 1)
    }
    /// Get the list of positions from this position to another
    /// position, moving diagonally.
    ///
    /// This does _not_ include the `from` position, and includes the `to` position.
    pub fn diagonals_to(&self, to: Self) -> Vec<Self> {
        if !self.is_diagonal_to(to) {
            return Vec::new();
        }

        let row_step;
        let col_step;
        if self.is_left_of(to) {
            col_step = 1;
        } else {
            col_step = -1;
        }

        if self.is_below(to) {
            row_step = 1;
        } else {
            row_step = -1;
        }

        let mut acc = *self;
        let mut result = Vec::new();
        for _ in 0..self.diagonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step);
            result.push(acc);
        }

        result
    }

    /// Get the list of positions from this position to another
    /// position, moving orthogonally.
    ///
    /// This does _not_ include the `from` position, and includes the `to` position.
    pub fn orthogonals_to(&self, to: Self) -> Vec<Self> {
        if !self.is_orthogonal_to(to) {
            return Vec::new();
        }
        let mut row_step = 0;
        let mut col_step = 0;
        if self.is_left_of(to) {
            col_step = 1;
        } else if self.is_right_of(to) {
            col_step = -1;
        } else if self.is_above(to) {
            row_step = -1;
        } else if self.is_below(to) {
            row_step = 1;
        }

        let mut acc = *self;
        let mut result = Vec::new();

        for _ in 0..self.orthogonal_distance(to) {
            acc = acc.add_row(row_step).add_col(col_step);
            result.push(acc);
        }

        result
    }

    pub fn is_in_palace(&self, self_color: Color) -> bool {
        match self_color {
            Color::Red => self.col >=3 && self.col <= 5 && self.row >=0 && self.row <= 2,
            Color::Black => self.col >=3 && self.col <= 5 && self.row >=7 && self.row <= 9,
        }
    }

    pub fn is_past_river

}