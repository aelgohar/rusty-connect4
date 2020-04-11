use crate::canvas::CanvasModel;
use crate::player::Player;
use yew::{prelude::*, components::Select, virtual_dom::VNode, Properties};

pub struct Connect4ComputerModel {
    player: Player,
    difficulty: Difficulty,
    update_player_name: Callback<InputData>,
    update_difficulty: Callback<Difficulty>,
    start_game_callback: Callback<ClickEvent>,
    end_game_callback: Callback<i64>,
    is_game_on: bool,
    disabled: bool,
    display_state: String,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
use Difficulty::*;

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Easy => String::from("Easy"),
            Medium => String::from("Medium"),
            Hard => String::from("Hard"),
        }
    }
}

pub enum Msg {
    NewPlayer(InputData),
    ChangeDifficulty(Difficulty),
    StartGame,
    EndGame,
}

impl Component for Connect4ComputerModel {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let player = Player {
            value: "".to_string(),
        };

        Connect4ComputerModel {
            player,
            difficulty: Easy,
            update_player_name: link.callback(|e: InputData| Msg::NewPlayer(e)),
            update_difficulty: link.callback(|e: Difficulty| Msg::ChangeDifficulty(e)),
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
            Msg::ChangeDifficulty(data) => {
                self.difficulty = data
                // update canvas
            }
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
                        id="player_name",
                        type="text",
                        placeholder="Your Name",
                        oninput = &self.update_player_name,
                    />
                    <Select<Difficulty> 
                        disabled = { self.disabled }
                        selected = Some(Easy),
                        options = { vec![Easy, Medium, Hard] }
                        onchange = &self.update_difficulty />
                    <button
                        id="startbutton",
                        onclick=&self.start_game_callback,
                        disabled={self.disabled},
                        title="Start Game">
                    { "Start Game" }
                    </button>
                </div>
            </div>
            <div style=format!("display: {}", self.display_state)>
                <br></br>
                <h4>{format!("New Game: {} Vs Computer", self.player.value)}</h4>
                <small>{format!("(Disc Colors: {} - ", self.player.value)} <b>{"Red"}</b> {"   and    Computer - "} <b>{"Yellow)"}</b></small>
                <br></br>
                <CanvasModel  
                    canvas_id = "connect_computer" 
                    player1 = self.player.value.clone(), 
                    player2 = "Computer" 
                    difficulty = self.difficulty,
                    game_done_cbk=&self.end_game_callback/>
            </div>
            </>
        }
    }
}
