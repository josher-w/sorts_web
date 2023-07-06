use yew::prelude::*;
use num::{self, integer::Roots, pow};
use crate::graph::*;

type Step = u8; //number of comparisons per second
const MIN_SPEED: u64 = 1;
const MAX_SPEED: u64 = 100;
const MIN_STEP: Step = 1;
const DEFAULT_STEP: Step = 4;
const MAX_STEP: Step = 7;

const MIN_NEL: usize = 1000;
pub const DEFAULT_NEL: usize = 50;
const MAX_NEL: usize = 1000;

pub struct Controls {
    play_state: PlayState,
    speed: Step,
}

#[derive(Debug, PartialEq)]
pub enum PlayState {
    Playing,
    Paused,
    Stopped,
    Finished
}

pub enum Msg {
    PlayPause,
    Stop,
    SlowDown,
    SpeedUp,
    Finish
}

impl Component for Controls {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            play_state: PlayState::Stopped,
            speed: DEFAULT_STEP,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PlayPause => {
                self.play_state = match self.play_state {
                    PlayState::Playing => PlayState::Paused,
                    PlayState::Paused => PlayState::Playing,
                    _ => PlayState::Playing
                };
                true
            }
            Msg::Stop => {
                self.play_state = PlayState::Stopped;
                true
            }
            Msg::SlowDown => {
                change_speed(self, false);
                true
            }
            Msg::SpeedUp => {
                change_speed(self, true);
                true
            }
            Msg::Finish => {
                self.play_state = PlayState::Finished;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
                <div class="center-container">
                    <span id="controls">
                        <button id="slower" onclick={link.callback(|_| Msg::SlowDown)}>{'-'}</button>
                        <button id="play-pause" onclick={link.callback(|_| Msg::PlayPause)}>
                            {if self.play_state == PlayState::Playing {"\u{23F5}\u{FE0E}"} else {"\u{23F8}\u{FE0E}"} }  
                        </button>
                        <button id="stop" onclick={link.callback(|_| Msg::Stop)}>{"\u{23F9}\u{FE0E}"}</button>
                        <button id="faster" onclick={link.callback(|_| Msg::SpeedUp)}>{'+'}</button>
                    </span>
                </div>
                <p>{"speed = "}{self.speed}</p>
                <p>{"comparisons per second = "}{calc_actual_speed(self)}</p>
                <Graph on_finished={Callback::from(|_| Msg::Finish)}/>
            </>
        }
    }
    
}

fn change_speed(state: &mut Controls, is_incr: bool) {
    state.speed = ((state.speed as i8 + if is_incr {1} else {-1}).abs() as Step).clamp(MIN_STEP, MAX_STEP) as u8;
}

// const X_MULT: f64 = ((MAX_SPEED as f64).ln()) / (MAX_STEP as f64);
// const Y_MULT: f64 = pow(MAX_SPEED as f64, (1 / MAX_STEP) as usize);
fn calc_actual_speed(state: &Controls) -> u64 {
    let x_mult: f64 = ((MAX_SPEED as f64).ln()) / ((MAX_STEP - 1) as f64);
    let y_mult: f64 = (MAX_SPEED as f64).powf(-((1 / (MAX_STEP - 1)) as f64));
    return ((y_mult * (x_mult * ((state.speed - 1) as f64)).exp()).round() as u64).clamp(MIN_SPEED, MAX_SPEED) as u64;
}