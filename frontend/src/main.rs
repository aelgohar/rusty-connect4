#[macro_use]
extern crate log;

use sidebar::{CanvasModel, Page, Sidebar};
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{MouseMoveEvent, ResizeEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::FillRule;
use stdweb::web::{document, window, CanvasRenderingContext2d};
use yew::prelude::*;
use yew::virtual_dom::VNode;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main() {
    yew::initialize();
    web_logger::init();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self) -> VNode {
        html! {
            <Sidebar>
                <Page
                    uri="Connect4Computer"
                    page_url="/#HowToConnect4"
                    title="How to Play Connect4"
                />
                <Page
                    uri="Connect4Computer"
                    page_url="/#Connect4Computer"
                    title="Play Connect4 With Computer"
                />
                <Page
                    uri="Connect4Human"
                    page_url="/#Connect4Human"
                    title="Play Connect4 with Another Human"
                />
                <Page
                     uri="HowToToot"
                     page_url="/#HowToToot"
                     title="How to Play TOOT-OTTO"
                />
                <Page
                    uri="TootOttoComputer"
                    page_url="/#TootOttoComputer"
                    title="Play Toot-Otto With Computer"
                />
                <Page
                    uri="TootOttoHuman"
                    page_url="/#TootOttoHuman"
                    title="Play Toot-Otto With Another Human"
                />
                <Page
                    uri="ScoreBoard"
                    page_url="/#ScoreBoard"
                    title="View Game History"
                />
                <Page
                    uri="Scores"
                    page_url="/#Scores"
                    title="Score Board"
                />
            </Sidebar>
        }
    }
}
