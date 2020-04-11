use anyhow::Error;
use serde::{Deserialize, Serialize};
use stdweb::web::Date;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    FetchReady(Result<Vec<Game>,Error>),
    Ignore,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Game {
    pub gameNumber: String,
    pub gameType: String,
    pub Player1Name: String,
    pub Player2Name: String,
    pub WinnerName: String,
    pub GameDate: u64,
}

pub struct ScoreBoardModel {
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<ScoreBoardModel>,
    data: Option<Vec<Game>>,
}

impl ScoreBoardModel {
    fn view_data(&self) -> Html {
        if let Some(ref games) = self.data {
            html! {
                { games.iter().enumerate().map(|(i, game)| {
                    let date = Date::from_time(game.GameDate as f64);
                    html! {
                        <tr>
                        <td>{ i + 1 }</td>
                        <td>{ game.gameType.as_str() }</td>
                        <td>{ game.Player1Name.as_str() }</td>
                        <td>{ game.Player2Name.as_str() }</td>
                        <td>{ game.WinnerName.as_str() }</td>
                        <td>{ &Date::from_time(game.GameDate as f64).to_string() }</td>
                        </tr>
                    }
                }).collect::<Html>() }
            }
        }
        else {
            html! {
                <tr><td colspan="6">{"Loading..."}</td></tr>
            }
        }
    }

    fn fetch_games(&mut self) -> FetchTask {
        let callback = self.link.callback(
            move |response: Response<Json<Result<Vec<Game>, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    error!("Failed to fetch games");
                    Msg::Ignore
                }
            }
        );
        let request = Request::get("/games").body(Nothing).unwrap();
        self.fetch_service.fetch(request, callback).unwrap()
    }
}

impl Component for ScoreBoardModel {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = ScoreBoardModel {
            fetch_service: FetchService::new(),
            fetch_task: None,
            link,
            data: None,
        };
        model.fetch_task = Some(model.fetch_games());
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(response) => {
                self.data = response.map(|data| data).ok();
                self.fetch_task = None;
            }
            Msg::Ignore => (),
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"> </hr>
            <div id="game-stream">
            <table>
                <tr>
                    <th>{"Game-ID"}</th>
                    <th>{"Game Type"}</th>
                    <th>{"Player1"}</th>
                    <th>{"Player2"}</th>
                    <th>{"Winner"}</th>
                    <th>{"When Played"}</th>
                </tr>
                { self.view_data() }
            </table>
            </div>
            </div>
        }
    }
}
