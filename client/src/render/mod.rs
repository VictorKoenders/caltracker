pub mod title;
pub mod day;
pub mod entry;

use yew::html::Html;
use {Context, Model};

pub trait Renderable {
    fn render(&self, model: &Model, index: usize) -> Html<Context, Model>;
}
