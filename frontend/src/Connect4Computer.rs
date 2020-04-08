use crate::canvas::CanvasModel;
use crate::player::Player;
use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::{prelude::*, switch::AllowMissing};

pub struct Connect4ComputerModel {
    props: Props,
    player: Player,
    update_player_name: Callback<InputData>,
    start_game_callback: Callback<ClickEvent>,
    end_game_callback: Callback<i64>,
    is_game_on: bool,
    disabled: bool,
    display_state: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    // pub route: Option<ARoute>,
}

pub enum Msg {
    NewPlayer(InputData),
    StartGame,
    EndGame,
}

impl Component for Connect4ComputerModel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let player = Player {
            value: "".to_string(),
        };

        Connect4ComputerModel {
            props,
            player,
            update_player_name: link.callback(|e: InputData| Msg::NewPlayer(e)),
            start_game_callback: link.callback(|e| Msg::StartGame),
            end_game_callback: link.callback(|e: i64| Msg::EndGame),
            is_game_on: false,
            disabled: false,
            display_state: "none".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewPlayer(val) => self.player.value = val.value,
            Msg::StartGame => {
                self.is_game_on = true;
                self.disabled = true;
                self.display_state = "block".to_string();
            } 
            Msg::EndGame => {
                self.is_game_on = false;
                self.disabled = false;
                self.display_state = "none".to_string();
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        html! {
            <>
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"></hr>
            </div>
            <div class="col-md-offset-3 col-md-8">
                <div class="col-md-offset-3 col-md-8">
                    <input
                        id="player_name",
                        type="text",
                        placeholder="Your Name",
                        oninput = &self.update_player_name,
                    />
                    <button
                    id="startbutton",
                    onclick=&self.start_game_callback,
                    disabled={self.disabled},
                    title="Start Game",
                    >
                    { "Start Game" }
                    </button>
                </div>
            </div>
            <div style=format!("display: {}", self.display_state)>
                <br></br>
                <h4>{format!("New Game: {} Vs Computer", self.player.value)}</h4>
                <small>{format!("(Disc Colors: {} - ", self.player.value)} <b>{"Red"}</b> {"   and    Computer - "} <b>{"Yellow)"}</b></small>
                <br></br>
                <CanvasModel player1 = self.player.value.clone(), player2 = "Computer" game_done_cbk=&self.end_game_callback/>
            </div>
            </>
        }
    }
}
