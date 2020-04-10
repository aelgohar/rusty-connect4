use crate::player::Player;
use yew::html::InputData;
use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::{prelude::*, switch::AllowMissing};

pub struct TootOttoComputerModel {
    player: Player,
    update_player_name: Callback<InputData>,
    start_game_callback: Callback<ClickEvent>,
    is_game_on: bool,
    disabled: bool,
}

#[derive(Debug)]
pub enum Msg {
    NewPlayer(InputData),
    StartGame,
}

impl Component for TootOttoComputerModel {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let player = Player {
            value: "".to_string(),
        };

        TootOttoComputerModel {
            player,
            update_player_name: link.callback(|e: InputData| Msg::NewPlayer(e)),
            start_game_callback: link.callback(|e| Msg::StartGame),
            is_game_on: false,
            disabled: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewPlayer(val) => self.player.value = val.value,
            Msg::StartGame => {
                self.is_game_on = true;
                self.disabled = true;
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> VNode {
        return html! {
            <>
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"></hr>
            </div>
            <div>
                <div>
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
            <br></br>
            <h4>{format!("New Game: {} Vs Computer", self.player.value)}</h4>
            <small>{format!("(Winning Combination: {} - ", self.player.value)} <b>{"TOOT"}</b> {"   and    Computer - "} <b>{"OTTO)"}</b></small>
            <br></br>
            </>
        };
    }
}
