use shared::{Day, Entry};

pub enum Msg {
    Load,
    Loaded(Result<Vec<Day>, ()>),
    LoadedEntry {
        day_index: usize,
        entry_index: usize,
        result: Entry,
    },
    SelectDay(usize),
    EditEntry(usize),
    UpdateEntryName(String),
    UpdateEntryValue(String),
    SaveEntry,
    NewEntry,
    Nop,
}
