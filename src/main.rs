use yew::prelude::*;
use std::fmt;
use duplicate::duplicate_item;

mod controls;
mod graph;

const DEFAULT_SORT: SortType = SortType::Bubble;
const DEFAULT_NEL: u128 = 100;
const DEFAULT_ORDER: Order = Order::Ascending;

struct Sort {
    sort_type: SortType,
    nel: u128,
    order: Order,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SortType {
    Bogo, 
    Bubble,
    Insertion,
    Merge,
    Quick
}

pub fn sort_colour(sort_type: SortType) -> &'static str {
    match sort_type {
        SortType::Bogo => todo!(),
        SortType::Bubble => "orangered",
        SortType::Insertion => todo!(),
        SortType::Merge => todo!(),
        SortType::Quick => todo!()
    }
}

#[derive(Debug)]
enum Order {
    Ascending,
    Descending
}

#[duplicate_item(name; [SortType]; [Order]; [controls::PlayState];)]
impl fmt::Display for name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <header>
                <h1>{"\u{25B6} Sorts"}</h1> 
            </header>

            <controls::Controls/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}