use crate::player::Player;
use crate::toot_canvas::TootCanvasModel;
use yew::html::InputData;
use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::{prelude::*, switch::AllowMissing};

pub struct TootOttoComputerModel {
    props: Props,
    player: Player,
    update_player_name: Callback<InputData>,
    start_game_callback: Callback<ClickEvent>,
    update_letter: Callback<InputData>,
    end_game_callback: Callback<i64>,
    is_game_on: bool,
    disabled: bool,
    display_state: String,
    letter: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    // pub route: Option<ARoute>,
}

#[derive(Debug)]
pub enum Msg {
    NewPlayer(InputData),
    StartGame,
    EndGame,
    UpdateLetter(InputData),
}

impl Component for TootOttoComputerModel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let player = Player {
            value: "".to_string(),
        };

        TootOttoComputerModel {
            props,
            player,
            update_player_name: link.callback(|e: InputData| Msg::NewPlayer(e)),
            start_game_callback: link.callback(|e| Msg::StartGame),
            end_game_callback: link.callback(|e: i64| Msg::EndGame),
            update_letter: link.callback(|e: InputData| Msg::UpdateLetter(e)),
            is_game_on: false,
            disabled: false,
            display_state: "none".to_string(),
            letter: "T".to_string(),
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
                // self.display_state = "none".to_string();
            }
            Msg::UpdateLetter(e) => {
                self.letter = e.value.to_string();
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> VNode {
        return html! {
            <>
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"></hr>
            </div>
            <div>
                <div>
                    <input
                        id="textbox1",
                        type="text",
                        placeholder="Player's Name",
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
            <br></br>
            <div style=format!("display: {}", self.display_state)>
            <h4>{format!("New Game: {} Vs Computer", self.player.value)}</h4>
                <small>{format!("(Winning Combination: {} - ", self.player.value)} <b>{"TOOT"}</b> {"   and    Computer - "} <b>{"OTTO)"}</b></small>
                <br></br>
                <h4>{"Select a Disc Type   :"}</h4>
                <input type="radio" id="T" name="gender" value="T" oninput=&self.update_letter/>
                <label for="T">{"T"}</label><br></br>
                <input type="radio" id="O" name="gender" value="O" oninput=&self.update_letter/>
                <label for="O">{"O"}</label>
                <br></br>
                <TootCanvasModel: canvas_id = "toot_computer" player1 = self.player.value.clone(), player2="Computer", letter=self.letter.clone(), game_done_cbk=&self.end_game_callback/>
            </div>
            <br></br>
            </>
        };
    }
}
