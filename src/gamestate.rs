#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ParsedGameState {
    pub piece_position: [[char; 8]; 8],
    pub active_player: Player,
    pub castling_rights: CastlingRights,
    pub en_passant_target: Option<(File, Rank)>,
    pub half_turn_clock: u8,
    pub full_turn_clock: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CastlingRights {
    pub black_kingside: bool,
    pub black_queenside: bool,
    pub white_kingside: bool,
    pub white_queenside: bool,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl File {
    pub fn from_square(square: u8) -> File{
        debug_assert_eq!(square & 0b11000000, 0);
        unsafe {std::mem::transmute(square % 8)}
    }

    pub fn to_u8(&self) -> u8 {
        unsafe {std::mem::transmute(*self)}
    }
}

impl TryFrom<char> for File {
    type Error = char;
    fn try_from(file: char) -> Result<File, Self::Error> {
        match file {
            'a'..='h' => Ok(unsafe { std::mem::transmute((file as u8) - b'a') }),
            x => Err(x),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Rank {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
}

impl Rank{
    pub fn from_square(square: u8) -> Rank{
        debug_assert_eq!(square & 0b11000000, 0);
        unsafe {std::mem::transmute(square / 8)}
    }

    pub fn to_u8(&self) -> u8 {
        unsafe {std::mem::transmute(*self)}
    }
}

impl TryFrom<char> for Rank {
    type Error = char;
    fn try_from(rank: char) -> Result<Rank, Self::Error> {
        match rank {
            '1'..='8' => Ok(unsafe { std::mem::transmute((rank as u8) - b'1') }),
            x => Err(x),
        }
    }
}