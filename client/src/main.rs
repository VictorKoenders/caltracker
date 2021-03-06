#![recursion_limit = "128"]

extern crate serde_json;
#[macro_use]
extern crate failure;
extern crate shared;
extern crate stdweb;
#[macro_use]
extern crate yew;

mod msg;
mod render;

use yew::html::{Component, ComponentUpdate, Env, Html, Scope, Renderable};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::console::ConsoleService;
use render::Renderable as CustomRenderable;
use shared::{Day, Entry};
use render::title::Title;
use yew::format::Json;
use msg::Msg;

#[derive(Default)]
pub struct Model {
    pub days: Vec<Day>,
    pub current_day: Option<usize>,
    pub current_entry: Option<usize>,
    pub fetch_task: Option<FetchTask>,
}

pub struct Context {
    pub console: ConsoleService,
    pub fetch: FetchService,
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService,
        fetch: FetchService::new(),
    };
    let mut app = Scope::<_, Model>::new(context);

    app.get_env().sender().send(
        ComponentUpdate::Message(Msg::Load),
    );

    app.mount_to_body();
    yew::run_loop();
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model::default()
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> bool {
        match msg {
            Msg::Load => {
                let callback = context.send_back(Msg::Loaded);
                let handler = move |r: Response<Json<Result<Vec<Day>, failure::Error>>>| {
                    let (meta, Json(r)) = r.into_parts();
                    if meta.status.is_success() {
                        callback.emit(r);
                    } else {
                        callback.emit(Err(format_err!("Could not load data: {:?}", meta)));
                    }
                };
                let task = context.fetch.fetch(
                    Request::get("/api/list").body(None).unwrap(),
                    handler.into(),
                );
                self.fetch_task = Some(task);
            }
            Msg::Loaded(days) => {
                match days {
                    Ok(d) => {
                        self.days = d;
                    }
                    Err(e) => {
                        context.console.error(&format!("{:?}", e));
                    }
                }
            }
            Msg::LoadedEntry {
                day_index,
                entry_index,
                result,
            } => {
                match result {
                    Ok(entry) => {
                        let day = &mut self.days[day_index];
                        day.entries[entry_index] = entry;
                    }
                    Err(e) => {
                        context.console.error(
                            &format!("Could not load entry: {:?}", e),
                        );
                    }
                }
            }
            Msg::SelectDay(index) => {
                self.current_day = Some(index);
            }
            Msg::EditEntry(index) => {
                if let Some(_) = self.current_entry {
                    save_model(context, self);
                }
                self.current_entry = Some(index);
            }
            Msg::NewEntry => {
                let day = &mut self.days[self.current_day.unwrap()];
                self.current_entry = Some(day.entries.len());
                day.entries.push(Default::default());
            }
            Msg::UpdateEntryName(name) => {
                let day = &mut self.days[self.current_day.unwrap()];
                let entry = &mut day.entries[self.current_entry.unwrap()];
                entry.name = name;
            }
            Msg::UpdateEntryValue(value) => {
                let day = &mut self.days[self.current_day.unwrap()];
                let entry = &mut day.entries[self.current_entry.unwrap()];
                entry.value = value.parse().unwrap_or(0f32);
            }
            Msg::SaveEntry => {
                save_model(context, self);
                self.current_entry = None;
            }
            Msg::Nop => {
                return false;
            }
        };
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <ul class="test", >
                    {for self.days.iter().enumerate().map(|(i, day)| Title(day).render(self, i))}
                </ul>
                { if let Some(idx) = self.current_day {
                    self.days[idx].render(self, idx)
                } else { html! { <div /> } } }
            </div>
        }
    }
}

fn save_model(context: &mut Env<Context, Model>, model: &Model) {
    let day_index = model.current_day.unwrap();
    let entry_index = model.current_entry.unwrap();
    let day = &model.days[day_index];
    let entry = &day.entries[entry_index];

    let url = format!(
        "/api/entry/{}/{}/{}",
        day.date.year,
        day.date.month,
        day.date.day
    );
    let json: yew::format::Storable = Json(&entry).into();
    let callback = context.send_back(move |data| {
        Msg::LoadedEntry {
            day_index,
            entry_index,
            result: data,
        }
    });
    let handler = move |response: Response<Json<Result<Entry, failure::Error>>>| {
        let (meta, Json(data)) = response.into_parts();
        if meta.status.is_success() {
            callback.emit(data);
        } else {
            callback.emit(Err(
                format_err!("Could not finish request: {:?}", meta.status),
            ));
        }
    };
    context.fetch.fetch(
        Request::post(url.as_str()).body(json).unwrap(),
        handler.into(),
    );
}
