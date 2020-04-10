use crate::Connect4Computer::Connect4ComputerModel;
use crate::Connect4Human::Connect4HumanModel;
use crate::HowToConnect4::HowToConnect4Model;
use crate::HowToToot::HowToTootModel;
use crate::ScoreBoard::ScoreBoardModel;
use crate::Scores::ScoresModel;
use crate::TootOttoComputer::TootOttoComputerModel;
use crate::TootOttoHuman::TootOttoHumanModel;
use crate::Welcome::WelcomeModel;

use yew::{prelude::*, virtual_dom::VNode};

pub struct DisplayWindow {
    props: DisplayWindowProps,
}

#[derive(Properties, Debug, Clone)]
pub struct DisplayWindowProps {
    pub uri: Option<String>,
}

pub enum Msg {}

impl Component for DisplayWindow {
    type Message = Msg;
    type Properties = DisplayWindowProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        DisplayWindow { props }
    }

    fn mounted(&mut self) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        if let Some(uri) = &self.props.uri {
            match uri.as_str() {
                "" => return html! {<WelcomeModel/>},
                "HowToConnect4" => return html! {<HowToConnect4Model/>},
                "HowToToot" => return html! {<HowToTootModel/>},
                "Connect4Computer" => return html! {<Connect4ComputerModel/>},
                "Connect4Human" => return html! {<Connect4HumanModel/>},
                "TootOttoComputer" => return html! {<TootOttoComputerModel/>},
                "TootOttoHuman" => return html! {<TootOttoHumanModel/>},
                "ScoreBoard" => return html! {<ScoreBoardModel/>},
                "Scores" => return html! {<ScoresModel/>},
                _ => {
                    return html! {"Page not found"};
                }
            }
        } else {
            html! {<WelcomeModel/>}
        }
    }
}
