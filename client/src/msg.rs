use shared::{Day, Entry};
use failure::Error;

#[derive(Debug)]
pub enum Msg {
    Load,
    Loaded(Result<Vec<Day>, Error>),
    LoadedEntry {
        day_index: usize,
        entry_index: usize,
        result: Result<Entry, Error>,
    },
    SelectDay(usize),
    EditEntry(usize),
    UpdateEntryName(String),
    UpdateEntryValue(String),
    SaveEntry,
    NewEntry,
    Nop,
}
