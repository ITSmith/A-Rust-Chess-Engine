use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct CastleRights {
    pub wk: bool,
    pub wq: bool,
    pub bk: bool,
    pub bq: bool,
}

impl From<&str> for CastleRights {
    fn from(value: &str) -> Self {
        CastleRights {
            wk: value.contains('K'),
            wq: value.contains('Q'),
            bk: value.contains('k'),
            bq: value.contains('q'),
        }
    }
}

impl Display for CastleRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wk = if self.wk { 'K' } else { '-' };
        let wq = if self.wq { 'Q' } else { '-' };
        let bk = if self.bk { 'k' } else { '-' };
        let bq = if self.bq { 'q' } else { '-' };
        write!(f, "{}{}{}{}", wk, wq, bk, bq)
    }
}
