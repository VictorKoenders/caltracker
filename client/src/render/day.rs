use super::Renderable;
use yew::html::Html;
use { Msg, Model };
use shared::Day;

impl Renderable for Day {
    fn render(&self, model: &Model, _: usize) -> Html<Msg> {
        html! {
            <div>
                <b>
                    {"Day "}
                    {self.label()}
                </b>
                <dl>
                    {for self.entries.iter().enumerate().map(|(idx, e)| e.render(model, idx))}
                    <dt><a onclick=|_| Msg::NewEntry, >{"New"}</a></dt>
                    <dd></dd>
                </dl>
            </div>
        }
    }
}
