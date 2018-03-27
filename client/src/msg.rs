use shared::{Day, Date, Entry};
use failure::Error;

#[derive(Debug)]
pub enum Msg {
    Load,
    Loaded(Result<Vec<Day>, Error>),
    LoadedEntry {
        day: Date,
        entry_index: usize,
        result: Result<Entry, Error>,
    },
    SelectDay(Date),
    EditEntry(usize),
    UpdateEntryName(String),
    UpdateEntryValue(String),
    SaveEntry,
    NewEntry,
    Nop,
}
