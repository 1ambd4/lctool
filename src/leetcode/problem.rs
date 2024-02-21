use anyhow::Result;

#[derive(Debug)]
pub struct Problem {
    pub id: i32,
    pub cn: String,
    pub en: String,
    pub slug: String,
    pub rate: f32,
    pub level: String,
}

impl Problem {
    pub fn slug(&self) -> Result<String> {
        Ok(self.slug.clone())
    }
}
