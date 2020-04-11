use crate::canvas::CanvasModel;
use yew::{prelude::*, virtual_dom::VNode, Properties};

use crate::player::Player;
use crate::Connect4Computer::Difficulty::Easy;

pub struct Connect4HumanModel {
    player1: Player,
    player2: Player,
    update_player1_name: Callback<InputData>,
    update_player2_name: Callback<InputData>,
    start_game_callback: Callback<ClickEvent>,
    end_game_callback: Callback<i64>,
    is_game_on: bool,
    disabled: bool,
    display_state: String,
}

#[derive(Debug)]
pub enum Msg {
    NewPlayer1(InputData),
    NewPlayer2(InputData),
    StartGame,
    EndGame,
}

impl Component for Connect4HumanModel {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let player1 = Player {
            value: "".to_string(),
        };

        let player2 = Player {
            value: "".to_string(),
        };

        Self {
            player1,
            player2,
            update_player1_name: link.callback(|e: InputData| Msg::NewPlayer1(e)),
            update_player2_name: link.callback(|e: InputData| Msg::NewPlayer2(e)),
            start_game_callback: link.callback(|e| Msg::StartGame),
            end_game_callback: link.callback(|e: i64| Msg::EndGame),
            is_game_on: false,
            disabled: false,
            display_state: "none".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewPlayer1(val) => self.player1.value = val.value,
            Msg::NewPlayer2(val) => self.player2.value = val.value,
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
        false
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
                        id="textbox1",
                        type="text",
                        placeholder="Player 1's Name",
                        oninput = &self.update_player1_name,
                    />
                    <input
                        id="textbox2",
                        type="text",
                        placeholder="Player 2's Name",
                        oninput = &self.update_player2_name,
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
                <h4>{format!("New Game: {} Vs {}", self.player1.value, self.player2.value)}</h4>
                <small disabled={!self.disabled}>{format!("(Disc Colors: {} - ", self.player1.value)} <b>{"Red"}</b> {format!("   and    {} - ", self.player2.value)} <b>{"Yellow)"}</b></small>
                <br></br>
                <CanvasModel: 
                    canvas_id = "connect_human" 
                    player1 = self.player1.value.clone(), 
                    player2=self.player2.value.clone(),
                    difficulty = Easy,
                    game_done_cbk=&self.end_game_callback/>
            </div>
            </>
        }
    }
}
