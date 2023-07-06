use yew::prelude::*;
use yew::Callback;

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::*;

const MIN_HEIGHT_PERCENT : f32 = 1.5;
const MAX_HEIGHT_PERCENT : f32 = 97.5;

pub struct Graph {
    values: Vec<u64>
}

#[derive(Properties, PartialEq)]
pub struct GraphProps {
    #[prop_or(controls::DEFAULT_NEL)]
    pub nel: usize,
    #[prop_or(SortType::Bubble)]
    pub sort_type: SortType,
    pub on_finished: Callback<(), controls::Msg>
}

pub enum Msg {
    Sort,
    Resize(usize),
    Finish
}

impl Component for Graph {
    type Message = Msg;
    type Properties = GraphProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            values: gen_values(ctx.props().nel),
        }
    }   

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Sort => {
                match ctx.props().sort_type {
                    SortType::Bubble => todo!(),
                    SortType::Bogo => todo!(),
                    SortType::Insertion => todo!(),
                    SortType::Merge => todo!(),
                    SortType::Quick => todo!(), //...
                };
                true
            }
            Msg::Resize(new_size) => {
                self.values.clear();
                self.values = gen_values(new_size);
                true
            }
            Msg::Finish => {
                ctx.props().on_finished.emit(()); 
                true
            }
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="center-container" style="width: 100%;">
                <p>{"nel = "}{ctx.props().nel.clone()}</p>
                <p>{"sort_type = "}{ctx.props().sort_type.clone()}</p>
                <div id="graph">
                    <div id="box">
                        { self.values.clone().into_iter().map(|value| render_line(value, ctx.props().nel, ctx.props().sort_type.clone()))
                            .collect::<Html>() }
                    </div>
                </div>
            </div>
        }
    }
}


fn gen_values(nel: usize) -> Vec<u64> {
    let mut values = (1..nel as u64).collect::<Vec<u64>>();
    shuffle(&mut values);
    return values;
}

fn shuffle(values: &mut Vec<u64>) {
    let mut rng = thread_rng();
    values.shuffle(&mut rng);
}

fn render_line(value: u64, nel: usize, sort_type: SortType) -> Html {
    html! {
        <div
            class="line"
            key={format!("line{}", value)}
            style={format!("height: {h}%; border-color: {c}", 
                h = (value as f32 / nel as f32) * 100f32,
                c = crate::sort_colour(sort_type)
                )}
            >
        </div>
    }
}

async fn bogo_sort() {
}

async fn bubble_sort() {

}