/// Essentially a straight copy of `TCOD_chars_t` from libtcod, but with Rustified names.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Chars {
    HLine = 196,
    VLine = 179,
    Ne = 191,
    Nw = 218,
    Se = 217,
    Sw = 192,
    TeeW = 180,
    TeeE = 195,
    TeeN = 193,
    TeeS = 194,
    Cross = 197,
    DHLine = 205,
    DVLine = 186,
    DNE = 187,
    DNW = 201,
    DSE = 188,
    DSW = 200,
    DTeeW = 185,
    DTeeE = 204,
    DTeeN = 202,
    DTeeS = 203,
    DCross = 206,
    Block1 = 176,
    Block2 = 177,
    Block3 = 178,
    ArrowN = 24,
    ArrowS = 25,
    ArrowE = 26,
    ArrowW = 27,
    Arrow2N = 30,
    Arrow2S = 31,
    Arrow2E = 16,
    Arrow2W = 17,
    DArrowH = 29,
    DArrowV = 18,
    CheckboxUnset = 224,
    CheckboxSet = 225,
    RadioUnset = 9,
    RadioSet = 10,
    SubpNW = 226,
    SubpNE = 227,
    SubpN = 228,
    SubpSE = 229,
    SubpDiag = 230,
    SubpE = 231,
    SubpSW = 232,
    Smilie = 1,
    SmilieInv = 2,
    Heart = 3,
    Diamond = 4,
    Club = 5,
    Spade = 6,
    Bullet = 7,
    BulletInv = 8,
    Male = 11,
    Female = 12,
    Note = 13,
    NoteDouble = 14,
    Light = 15,
    ExclamDouble = 19,
    Pilcrow = 20,
    Section = 21,
    Pound = 156,
    Multiplication = 158,
    Function = 159,
    Reserved = 169,
    Half = 171,
    OneQuarter = 172,
    Copyright = 184,
    Cent = 189,
    Yen = 190,
    Currency = 207,
    ThreeQuarters = 243,
    Division = 246,
    Grade = 248,
    Umlaut = 249,
    Pow1 = 251,
    Pow3 = 252,
    Pow2 = 253,
    BulletSquare = 254,
}

impl Chars {
    pub fn into_char(self) -> char {
        self as u8 as char
    }
    pub fn into_string(self) -> String {
        self.into_char().to_string()
    }
}
