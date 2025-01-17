pub enum Reason {
    OK200,
    BADREQUEST400,
    INTERNAL500,
}

impl Reason {
    pub fn code(&self) -> i32 {
        match self {
            Reason::BADREQUEST400 => 400,
            Reason::INTERNAL500 => 500,
            Reason::OK200 => 200,
        }
    }

    pub fn reason(&self) -> String {
        String::from(match self {
            Reason::BADREQUEST400 => "Bad Request",
            Reason::INTERNAL500 => "Internal Server Error",
            Reason::OK200 => "Ok",
        })
    }
}
