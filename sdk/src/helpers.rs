// helper to read catalog extras easily

#[derive(Debug, Clone, Default)]
pub struct Extra {
    pub search: Option<String>,
    pub genre: Option<String>,
    pub skip: Option<u32>
}

impl Extra {
    pub fn new(extras: Vec<(String, String)>) -> Result<Self, String> {
        let mut extra = Self::default();

        for (extra_type, value) in extras {
            match extra_type.as_str() {
                "search" => extra.search = Some(value),
                "skip" => {
                    let skip = value.parse().map_err(|_| {
                        format!("extra type `skip` has invalid value: {}", value)
                    })?;
                    extra.skip = Some(skip)
                }
                "genre"=> extra.genre = Some(value),
                unknown_type=>  Err(format!("extra type not supported: {}", unknown_type))?
            }
        }
        Ok(extra)
    }
}
