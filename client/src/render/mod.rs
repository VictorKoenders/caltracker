pub mod title;
pub mod day;
pub mod entry;

use yew::html::Html;
use {Msg, Model};

pub trait Renderable {
    fn render(&self, model: &Model, index: usize) -> Html<Msg>;
}
