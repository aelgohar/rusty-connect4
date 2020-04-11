use anyhow::Error;
use serde_json::json;
use std::iter::FromIterator;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{MouseMoveEvent, ResizeEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::FillRule;
use stdweb::web::{document, window, CanvasRenderingContext2d};
use stdweb::web::Date;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{prelude::*, virtual_dom::VNode, Properties};

use crate::player::Player;
use crate::ScoreBoard::Game;
use crate::Connect4Computer::Difficulty::{self, *};

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub struct TootCanvasModel {
    props: Props,
    canvas_id: String,
    canvas: Option<CanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    cbk: Callback<ClickEvent>,
    animate_cbk: Callback<(usize, i64, char, usize, usize, bool)>,
    map: Vec<Vec<i64>>,
    dummy_map: Vec<Vec<char>>,
    current_move: i64,
    won: bool,
    paused: bool,
    reject_click: bool,
    letter: String,
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    link: ComponentLink<TootCanvasModel>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub player1: Option<String>,
    pub player2: Option<String>,
    pub difficulty: Difficulty,
    pub canvas_id: Option<String>,
    pub game_done_cbk: Callback<i64>,
    pub letter: String,
}

pub enum Message {
    Click(ClickEvent),
    AnimateCallback((usize, i64, char, usize, usize, bool)),
    Ignore
}

impl TootCanvasModel {
    pub fn reset(&mut self) {
        self.map = vec![vec![0; 7]; 6];
        self.dummy_map = vec![vec!['a'; 7]; 6];
        self.current_move = 0;
        self.paused = false;
        self.won = false;
        self.reject_click = false;
        self.clear();
        self.draw_mask();
    }

    #[inline]
    pub fn check_state(&self, state: &Vec<Vec<i64>>) -> (i64, i64) {
        let mut win_val = 0;
        let mut chain_val = 0;
        let (mut temp_r, mut temp_b, mut temp_br, mut temp_tr) = (0, 0, 0, 0);
        for i in 0..6 {
            for j in 0..7 {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;
                for k in 0..=3 {
                    // TODO may need to be flipped
                    let sign: i64 = if k == 0 || k == 3 { -1 } else { 1 };
                    if j + k < 7 {
                        temp_r += sign * state[i][j + k];
                    }
                    if i + k < 6 {
                        temp_b += sign * state[i + k][j];
                    }
                    if i + k < 6 && j + k < 7 {
                        temp_br += sign * state[i + k][j + k];
                    }

                    if i >= k && j + k < 7 {
                        temp_tr += sign * state[i - k][j + k];
                    }
                }
                chain_val += temp_r * temp_r * temp_r;
                chain_val += temp_b * temp_b * temp_b;
                chain_val += temp_br * temp_br * temp_br;
                chain_val += temp_tr * temp_tr * temp_tr;

                if temp_r.abs() == 4 {
                    win_val = temp_r;
                } else if temp_b.abs() == 4 {
                    win_val = temp_b;
                } else if temp_br.abs() == 4 {
                    win_val = temp_br;
                } else if temp_tr.abs() == 4 {
                    win_val = temp_tr;
                }
            }
        }
        return (win_val, chain_val);
    }

    pub fn value(
        &self,
        state: &Vec<Vec<i64>>,
        depth: i64,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64) {
        let val = self.check_state(state);
        let max_depth = match self.props.difficulty {
            Easy => 1,
            Medium => 2,
            Hard => 3,
        };
        if depth >= max_depth {
            // if slow (or memory consumption is high), lower the value
            let mut ret_val = 0;

            // if win, value = +inf
            let win_val = val.0;
            let chain_val = val.1;
            ret_val = chain_val;

            // If it lead to winning, then do it
            if win_val == 4 {
                // AI win, AI wants to win of course
                ret_val = 999999;
            } else if win_val == 4 {
                // AI lose, AI hates losing
                ret_val = 999999 * -1;
            }
            ret_val -= depth * depth;

            return (ret_val);
        }

        let win = val.0;
        // if already won, then return the value right away
        if win == 4 {
            // AI win, AI wants to win of course
            return (999999 - depth * depth);
        }
        if win == -4 {
            // AI lose, AI hates losing
            return (-999999 - depth * depth);
        }

        if depth % 2 == 0 {
            return self.min_state(state, depth + 1, alpha, beta).0;
        } else {
            return self.max_state(state, depth + 1, alpha, beta).0;
        }
    }

    // TODO - add letter to AI search
    pub fn max_state(
        &self,
        state: &Vec<Vec<i64>>,
        depth: i64,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64, (i64, char)) {
        let mut v = -100000000007;
        let mut new_move: (i64, char) = (-1, ' ');
        let mut move_queue = Vec::new();

        for letter in &['T', 'O'] {
            for j in 0..7 {
                let move_value = if *letter == 'T' { 1 } else { -1 };
                let temp_state = self.fill_map(state, j, move_value);
                if temp_state[0][0] != 999 {
                    let temp_val = self.value(&temp_state, depth, alpha, beta);
                    if temp_val > v {
                        v = temp_val;
                        new_move = (j as i64, *letter);
                        move_queue = Vec::new();
                        move_queue.push((j as i64, *letter));
                    } else if temp_val == v {
                        move_queue.push(((j as i64, *letter)));
                    }

                    // alpha-beta pruning
                    if v > beta {
                        new_move = self.choose(&move_queue);
                        return (v, new_move);
                    }
                    alpha = std::cmp::max(alpha, v);
                }
            }
        }
        new_move = self.choose(&move_queue);

        return (v, new_move);
    }

    // TODO - add letter choice to search
    pub fn min_state(
        &self,
        state: &Vec<Vec<i64>>,
        depth: i64,
        mut alpha: i64,
        mut beta: i64,
    ) -> (i64, (i64, char)) {
        let mut v = 100000000007;
        let mut new_move: (i64, char) = (-1, ' ');
        let mut move_queue = Vec::new();

        for letter in &['T', 'O'] {
            for j in 0..7 {
                let move_value = if *letter == 'T' { 1 } else { -1 };
                let temp_state = self.fill_map(state, j, move_value);
                if temp_state[0][0] != 999 {
                    let temp_val = self.value(&temp_state, depth, alpha, beta);
                    if temp_val < v {
                        v = temp_val;
                        new_move = (j as i64, *letter);
                        move_queue = Vec::new();
                        move_queue.push((j as i64, *letter));
                    } else if temp_val == v {
                        move_queue.push((j as i64, *letter));
                    }

                    // alpha-beta pruning
                    if v < alpha {
                        new_move = self.choose(&move_queue);
                        return (v, new_move);
                    }
                    beta = std::cmp::min(beta, v);
                }
            }
        }
        new_move = self.choose(&move_queue);

        return (v, new_move);
    }

    #[inline]
    pub fn get_random_val(&self, val: usize) -> usize {
        let rand = js! { return Math.random(); };
        let base: f64 = rand.try_into().unwrap();
        let max_val = val as f64;

        return (base * max_val).floor() as usize;
    }

    #[inline]
    pub fn choose<T: Copy>(&self, choice: &Vec<T>) -> T {
        let index = self.get_random_val(choice.len());
        return choice[index];
    }

    pub fn ai(&mut self, ai_move_value: i64) {
        let new_map = self.map.clone();
        let val_choice = self.max_state(&new_map, 0, -100000000007, 100000000007);

        let (val, (column, letter)) = val_choice;

        self.paused = false;
        let mut done = self.action(column as usize, letter, true);

        while done < 0 {
            log::info!("Using random agent");
            let random_choice = self.get_random_val(7);
            let letter = if self.get_random_val(2) == 0 { 'T' } else { 'O' };
            done = self.action(random_choice, letter, true);
        }
    }

    // TODO - add choice of letter
    #[inline]
    pub fn fill_map(&self, new_state: &Vec<Vec<i64>>, column: usize, value: i64) -> Vec<Vec<i64>> {
        let mut temp_map = new_state.clone();
        if temp_map[0][column] != 0 || column > 6 {
            temp_map[0][0] = 999; // error code
        }

        let mut done = false;
        let mut row = 0;

        for i in 0..5 {
            if temp_map[i + 1][column] != 0 {
                done = true;
                row = i;
                break;
            }
        }
        if !done {
            row = 5;
        }

        temp_map[row][column] = value;
        return temp_map;
    }

    #[inline]
    pub fn draw_circle(&self, x: u32, y: u32, fill: &str, stroke: &str, text: &str) {
        let context = self.ctx.as_ref().unwrap();

        context.save();
        context.set_fill_style_color(&fill);
        context.set_stroke_style_color(&stroke);
        context.begin_path();
        context.arc(x as f64, y as f64, 25.0, 0.0, 2.0 * 3.14159265359, false);
        context.fill(FillRule::NonZero);
        context.restore();

        context.set_font("bold 30px serif");
        context.restore();
        context.fill_text(text, x as f64 - 12.0, y as f64 + 12.0, None);
    }

    #[inline]
    pub fn draw_mask(&self) {
        let context = self.ctx.as_ref().unwrap();

        context.save();
        context.set_fill_style_color("#00bfff");
        context.begin_path();
        for y in 0..6 {
            for x in 0..7 {
                context.arc(
                    (75 * x + 100) as f64,
                    (75 * y + 50) as f64,
                    25.0,
                    0.0,
                    2.0 * 3.14159265359,
                    false,
                );
                context.rect((75 * x + 150) as f64, (75 * y) as f64, -100.0, 100.0);
            }
        }
        context.fill(FillRule::NonZero);
        context.restore();
    }

    #[inline]
    pub fn draw(&self) {
        for y in 0..6 {
            for x in 0..7 {
                let mut text = "";
                let mut fg_color = "transparent";
                if self.map[y][x] >= 1 && self.dummy_map[y][x] == 'T' {
                    fg_color = "#99ffcc";
                    text = "T";
                } else if self.map[y][x] >= 1 && self.dummy_map[y][x] == 'O' {
                    fg_color = "#99ffcc";
                    text = "O";
                } else if self.map[y][x] <= -1 && self.dummy_map[y][x] == 'T' {
                    fg_color = "#ffff99";
                    text = "T";
                } else if self.map[y][x] <= -1 && self.dummy_map[y][x] == 'O' {
                    fg_color = "#ffff99";
                    text = "O";
                }

                self.draw_circle(
                    (75 * x + 100) as u32,
                    (75 * y + 50) as u32,
                    &fg_color,
                    "black",
                    text,
                );
            }
        }
    }

    #[inline]
    pub fn check(&mut self) {
        let (mut temp_r, mut temp_b, mut temp_br, mut temp_tr) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for i in 0..6 {
            for j in 0..7 {
                temp_r = vec!['a'; 4];
                temp_b = vec!['a'; 4];
                temp_br = vec!['a'; 4];
                temp_tr = vec!['a'; 4];
                for k in 0..=3 {
                    if j + k < 7 {
                        temp_r[k] = self.dummy_map[i][j + k];
                    }

                    if i + k < 6 {
                        temp_b[k] = self.dummy_map[i + k][j];
                    }

                    if i + k < 6 && j + k < 7 {
                        temp_br[k] = self.dummy_map[i + k][j + k];
                    }

                    if i >= k && j + k < 7 {
                        temp_tr[k] = self.dummy_map[i - k][j + k];
                    }
                }
                let toot = "TOOT";
                let otto = "OTTO";
                if String::from_iter(temp_r.clone()) == toot {
                    self.win(1);
                } else if String::from_iter(temp_r) == otto {
                    self.win(-1);
                } else if String::from_iter(temp_b.clone()) == toot {
                    self.win(1);
                } else if String::from_iter(temp_b) == otto {
                    self.win(-1);
                } else if String::from_iter(temp_br.clone()) == toot {
                    self.win(1);
                } else if String::from_iter(temp_br) == otto {
                    self.win(-1);
                } else if String::from_iter(temp_tr.clone()) == toot {
                    self.win(1);
                } else if String::from_iter(temp_tr) == otto {
                    self.win(-1);
                }
            }
        }
        // check if draw
        if self.current_move == 42 && !self.won {
            self.win(0);
        }
    }

    #[inline]
    pub fn clear(&self) {
        self.ctx.as_ref().unwrap().clear_rect(
            0.0,
            0.0,
            self.canvas.as_ref().unwrap().width() as f64,
            self.canvas.as_ref().unwrap().height() as f64,
        );
    }

    #[inline]
    pub fn on_region(&self, coord: f64, x: f64, radius: f64) -> bool {
        return ((coord - x) * (coord - x) <= radius * radius);
    }

    #[inline]
    pub fn player_move(&self) -> i64 {
        match self.letter.as_str() {
            "T" => 1,
            "O" => -1,
            _ => 0, // unreachable
        }
    }

    pub fn animate(
        &mut self,
        column: usize,
        current_move: i64,
        letter: char,
        to_row: usize,
        cur_pos: usize,
        mode: bool,
    ) {
        let mut fg_color = "transparent";
        if current_move >= 1 {
            fg_color = "#99ffcc";
        } else if current_move <= -1 {
            fg_color = "#ffff99";
        }

        if to_row * 75 >= cur_pos {
            self.clear();
            self.draw();
            self.draw_circle(
                (75 * column + 100) as u32,
                (cur_pos + 50) as u32,
                &fg_color,
                "black",
                &letter.to_string(),
            );
            self.draw_mask();

            let cloned = self.animate_cbk.clone();
            window().request_animation_frame(enclose!((cloned) move |_| {
                cloned.emit((column, current_move, letter, to_row, cur_pos+25, mode));
            }));
        } else {
            self.map[to_row][column] = if letter == 'T' { 1 } else { -1 };
            self.dummy_map[to_row][column] = letter;
            self.current_move += 1;
            self.draw();
            self.check();
            if mode == false && self.props.player2.as_ref().unwrap() == "Computer" {
                self.ai(-1);
            } else {
                self.reject_click = false;
            }
        }
    }

    #[inline(always)]
    pub fn action(&mut self, column: usize, letter: char, mode: bool) -> i64 {
        if self.paused || self.won {
            return 0;
        }

        if self.map[0][column] != 0 || column > 6 {
            return -1;
        }

        let mut done = false;
        let mut row = 0;
        for i in 0..5 {
            if self.map[i + 1][column] != 0 {
                done = true;
                row = i;
                break;
            }
        }
        if !done {
            row = 5;
        }

        self.animate(column, self.player_move(), letter, row, 0, mode);

        self.paused = true;
        return 1;
    }

    #[inline]
    pub fn win(&mut self, player: i64) {
        self.paused = true;
        self.won = true;
        self.reject_click = false;

        let mut msg = String::new();
        if player > 0 {
            msg = format!("{} wins", self.props.player1.as_ref().unwrap());
        } else if player < 0 {
            msg = format!("{} wins", self.props.player2.as_ref().unwrap());
        } else {
            msg = "It's a draw".to_string();
        }

        let to_print = format!("{} - Click on game board to reset", msg);

        let context = self.ctx.as_ref().unwrap();
        context.save();
        context.set_font("14pt sans-serif");
        context.set_fill_style_color("#111");
        context.fill_text(&to_print, 150.0, 20.0, None);

        // construct game to post
        let game = Game {
            gameNumber: String::new(),
            gameType: String::from("TOOT-OTTO"),
            Player1Name: self.props.player1.as_ref().unwrap().clone(),
            Player2Name: self.props.player2.as_ref().unwrap().clone(),
            WinnerName: if player > 0 {
                self.props.player1.as_ref().unwrap().clone()
            } else if player < 0 {
                self.props.player2.as_ref().unwrap().clone()
            } else {
                String::from("Draw")
            },
            GameDate: Date::now() as u64,
        };

        // construct callback
        let callback = self.link.callback(
            move |response: Response<Result<String, Error>>| {
                info!("successfully saved!");
                Message::Ignore
            }
        );

        // construct request
        let request = Request::post("/games")
            .header("Content-Type", "application/json")
            .body(Json(&game)).unwrap();

        // send the request
        self.fetch_task = self.fetch_service.fetch(request, callback).ok();

        context.restore();
    }
}

impl Component for TootCanvasModel {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let canvas_id = props.canvas_id.clone().unwrap();
        let letter = props.letter.clone();

        let mut map: Vec<Vec<i64>> = vec![vec![0; 7]; 6];
        let mut dummy_map: Vec<Vec<char>> = vec![vec!['a'; 7]; 6];

        Self {
            props,
            canvas_id,
            canvas: None,
            ctx: None,
            cbk: link.callback(|e: ClickEvent| Message::Click(e)),
            animate_cbk: link.callback(
                |e: (usize, i64, char, usize, usize, bool)| Message::AnimateCallback(e)),
            map,
            dummy_map,
            current_move: 0,
            paused: false,
            won: false,
            reject_click: false,
            letter,
            fetch_service: FetchService::new(),
            fetch_task: None,
            link,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::Click(e) => {
                if self.reject_click {
                    return false;
                }

                if self.won {
                    self.reset();
                    self.props.game_done_cbk.emit(0);
                    return true;
                }

                let rect = self.canvas.as_ref().unwrap().get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.get_left();

                for j in 0..7 {
                    if self.on_region(x, (75 * j + 100) as f64, 25 as f64) {
                        self.paused = false;

                        // TODO
                        let valid = self.action(j, self.letter.chars().next().unwrap(), false);
                        if valid == 1 {
                            self.reject_click = true;
                        };

                        break;
                    }
                }
            }
            Message::AnimateCallback((a, b, c, d, e, f)) => {
                self.animate(a, b, c, d, e, f);
            }
            Message::Ignore => (),
        };

        true
    }

    fn view(&self) -> Html {
        html! {
            <canvas id={&self.canvas_id} height="480" width="640"></canvas>
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.canvas = Some(canvas(self.canvas_id.as_str()));
        self.ctx = Some(context(self.canvas_id.as_str()));

        let ctx = self.ctx.as_ref().unwrap();
        let cloned_cbk = self.cbk.clone();

        self.canvas.as_ref().unwrap().add_event_listener(enclose!(
            (ctx) move | event: ClickEvent | {
                cloned_cbk.emit(event);
            }
        ));

        // clears and draws mask
        self.reset();

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        self.letter = self.props.letter.clone();
        true
    }
}

#[inline(always)]
fn canvas(id: &str) -> CanvasElement {
    document()
        .query_selector(&format!("#{}", id))
        .unwrap()
        .expect(&format!("Failed to select canvas id #{}", id))
        .try_into()
        .unwrap()
}

#[inline(always)]
fn context(id: &str) -> CanvasRenderingContext2d {
    canvas(id).get_context().unwrap()
}
