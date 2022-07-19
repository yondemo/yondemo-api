struct UserName<'a> {
    nickname: &'a str,
}

impl<'a> UserName<'a> {
    pub fn new(nickname: &'a str) -> Self {
        Self { nickname }
    }
}

pub struct User<'a> {
    name: UserName<'a>,
}

impl<'a> User<'a> {
    pub fn new(nickname: &'a str) -> Self {
        Self { name: UserName::new(nickname) }
    }

    pub fn nickname(&self) -> &str {
        self.name.nickname
    }
}

struct Reference<'a> {
    name: &'a str
}

struct Reading<'a> {
    user: User<'a>,
    reference: Reference<'a>,
    understanding: Understanding<'a>,
}

struct Understanding<'a> {
    user: User<'a>,
    reference: Reference<'a>,
    level: UnderstandingLevel<'a>,
}

struct UnderstandingLevel<'a> {
    name: &'a str,
    value: i8
}