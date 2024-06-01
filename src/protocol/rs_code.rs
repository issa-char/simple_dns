#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rs_Code{
    NOERROR = 0,
    FORMERR = 1,
    SERVFAIL = 2,
    NXDOMAIN = 3,
    NOTIMP = 4,
    REFUSED = 5,
    }

impl Rs_Code {
    pub fn from_num(num: u8) -> Rs_code{
        match num {
            1 => RsCode::FORMERR,
            2 => RsCode::SERVFAIL,
            3 => RsCode::NXDOMAIN,
            4 => RsCode::NOTIMP,
            5 => RsCode::REFUSED,
            0 | _ => RsCode::NOERROR,
        }
    }
}


