use yew::html::{Html, InputData};
use super::Renderable;
use shared::Entry;
use {Model, Msg};

impl Renderable for Entry {
    fn render(&self, model: &Model, index: usize) -> Html<Msg> {
        match model.current_entry {
            Some(i) if i == index => html! {
                <dt>
                    <input type="text",
                        value={&self.name},
                        oninput=|e: InputData| Msg::UpdateEntryName(e.value),
                    />
                </dt>
                <dd>
                    <input type="text",
                        value={&self.value},
                        oninput=|e: InputData| Msg::UpdateEntryValue(e.value),
                    />
                </dd>
                <input type="button",
                    value={"Save"},
                    onclick=|_| Msg::SaveEntry, />
            },
            _ => html! {
                <dt ondoubleclick=move|_| Msg::EditEntry(index), >{&self.name}</dt>
                <dd ondoubleclick=move|_| Msg::EditEntry(index), >{&self.value}</dd>
            },
        }

    }
}

