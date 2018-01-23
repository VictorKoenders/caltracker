#[derive(Default)]
pub struct Model {
    pub days: Vec<Day>,
    pub current_day: Option<usize>,
    pub current_entry: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct ModelFromServer {
    pub days: Vec<Day>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Day {
    pub date: Date,
    pub entries: Vec<Entry>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Day {
    pub fn label(&self) -> String {
        format!(
            "{}/{}: {} cals",
            self.date.month,
            self.date.day,
            self.total()
        )
    }

    pub fn total(&self) -> f32 {
        self.entries.iter().map(|e| e.value).sum()
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub name: String,
    pub value: f32,
}

