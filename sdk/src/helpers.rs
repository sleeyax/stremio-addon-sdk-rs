// helper to read catalog extras easily

#[derive(Clone)]
pub struct Extra {
    pub search: Option<String>,
    pub genre: Option<String>,
    pub skip: Option<u32>
}
impl Extra {
    pub fn empty() -> Self {
        Self {
            genre: None,
            search: None,
            skip: None
        }
    }
    pub fn parse(&mut self, extras: Vec<(String, String)>) -> Self {
        for extra in extras {
            let (extra_type, value) = extra;

            match extra_type.as_str() {
                "search" =>  self.search = Some(value),
                "skip"   =>  self.skip = Some(value.parse().unwrap()),
                "genre"  =>  self.genre = Some(value),
                _x       =>  panic!("{} not supported!", _x)
            }
        }

        self.clone()
    }
}