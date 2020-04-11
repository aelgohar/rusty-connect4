use std::collections::HashMap;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use stdweb::web::Date;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use crate::ScoreBoard::Game;

pub enum Msg {
    FetchReady(Result<Vec<Game>, Error>),
    Ignore,
}

impl From<()> for Msg {
    fn from(parameter: ()) -> Self {
        error!("Tried to create message from unit type!");
        Msg::Ignore
    }
}

pub struct ScoresModel {
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<ScoresModel>,
    data: Option<Vec<Game>>,
}

impl ScoresModel {
    fn view_total_games(&self) -> Html {
        if let Some(ref games) = self.data {
            html! {
                <tr>
                    <td>{ games.len() }</td>
                    <td>{ games.iter().filter(|game| game.Player2Name == "Computer").count() }</td>
                    <td>{ games.iter().filter(|game| game.WinnerName == "Computer").count() }</td>
                </tr>
            }
        } else {
            html! {}
        }
    }

    fn view_computer_wins(&self) -> Html {
        if let Some(ref games) = self.data {
            html! {
                { games.iter().filter(|game| game.WinnerName == "Computer").enumerate().map(|(i, game)| {
                    let date = Date::from_time(game.GameDate as f64);
                    html! {
                        <tr>
                        <td>{ i + 1 }</td>
                        <td>{ game.gameType.as_str() }</td>
                        <td>{ game.WinnerName.as_str() }</td>
                        <td>{ game.Player1Name.as_str() }</td>
                        <td>{ &Date::from_time(game.GameDate as f64).to_string()[0..24] }</td>
                        </tr>
                    }
                }).collect::<Html>() }
            }
        } else {
            html! {}
        }
    }

    fn view_total_wins(&self) -> Html {
        if let Some(ref games) = self.data {
            // aggregate total wins for each player
            let mut counts = HashMap::new();
            for game in games {
                *counts.entry(game.WinnerName.as_str()).or_insert(0) += 1;
            }

            // sort by win count
            let mut counts: Vec<_> = counts.iter().collect();
            counts.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

            html! {

                { counts.iter().enumerate().map(|(i, (player, count))| {
                    html! {
                        <tr>
                        <td>{ i + 1 }</td>
                        <td>{ player }</td>
                        <td>{ count }</td>
                        </tr>
                    }
                }).collect::<Html>() }
            }
        } else {
            html! {}
        }
    }

    fn fetch_games(&mut self) -> FetchTask {
        let callback =
            self.link
                .callback(move |response: Response<Json<Result<Vec<Game>, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    if meta.status.is_success() {
                        Msg::FetchReady(data)
                    } else {
                        error!("Failed to fetch games");
                        Msg::Ignore
                    }
                });
        let request = Request::get("/games").body(Nothing).unwrap();
        self.fetch_service.fetch(request, callback).unwrap()
    }
}

impl Component for ScoresModel {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = ScoresModel {
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
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Score Board"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round"></hr>
            <div><h4>{"Games Won by Computer"}</h4></div>
                <table>
                    <tr>
                        <th>{"Total Games Played"}</th>
                        <th>{"Games Against Computer"}</th>
                        <th>{"Games Computer Won"}</th>
                    </tr>
                    { self.view_total_games() }
                </table>
            <br></br>
            <div><h4>{"Details of Games Won by Computer"}</h4></div>
                <div id="game-stream">
                <table>
                    <tr>
                        <th>{"Sl. No."}</th>
                        <th>{"Game Type"}</th>
                        <th>{"Winner"}</th>
                        <th>{"Played Against"}</th>
                        <th>{"When Played"}</th>
                    </tr>
                    { self.view_computer_wins() }
                 </table>
            </div>
            <br></br>
            <div><h4>{"Details of Games Won by All Players"}</h4></div>
            <div id="game-stream">
                <table>
                    <tr>
                        <th>{"Sl. No."}</th>
                        <th>{"Winner or Draw"}</th>
                        <th>{"No. of Wins"}</th>
                    </tr>
                    { self.view_total_wins() }
                </table>
            </div>
            </div>
        }
    }
}

//NOTE: Backend work here i think
