use crate::player::Player;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{MouseMoveEvent, ResizeEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::FillRule;
use stdweb::web::{document, window, CanvasRenderingContext2d};
use yew::{prelude::*, virtual_dom::VNode, Properties};
use yew_router::{prelude::*, switch::AllowMissing};

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

pub struct CanvasModel {
    props: Props,
    canvas_id: String,
    canvas: Option<CanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    cbk: Callback<ClickEvent>,
    animate_cbk: Callback<(usize, i64, usize, usize, bool)>,
    map: Vec<Vec<i64>>,
    current_move: i64,
    won: bool,
    paused: bool,
    reject_click: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub player1: Option<String>,
    pub player2: Option<String>,
    pub game_done_cbk: Callback<i64>,
}

pub enum Message {
    Click(ClickEvent),
    AnimateCallback((usize, i64, usize, usize, bool)),
}

impl CanvasModel {
    pub fn reset(&mut self) {
        self.map = vec![vec![0; 7]; 6];
        self.current_move = 0;
        self.paused = false;
        self.won = false;
        self.reject_click = false;
        self.clear();
        self.draw_mask();
    }

    pub fn check_state(&self, state: &Vec<Vec<i64>>) -> (i64, i64) {
        let mut winVal = 0;
        let mut chainVal = 0;
        let (mut temp_r, mut temp_b, mut temp_br, mut temp_tr) = (0, 0, 0, 0);
        for i in 0..6 {
            for j in 0..7 {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;
                for k in 0..=3 {
                    if (j + k < 7) {
                        temp_r += state[i][j + k];
                    }

                    if (i + k < 6) {
                        temp_b += state[i + k][j];
                    }

                    if (i + k < 6 && j + k < 7) {
                        temp_br += state[i + k][j + k];
                    }

                    if (i >= k && j + k < 7) {
                        temp_tr += state[i - k][j + k];
                    }
                }
                chainVal += temp_r * temp_r * temp_r;
                chainVal += temp_b * temp_b * temp_b;
                chainVal += temp_br * temp_br * temp_br;
                chainVal += temp_tr * temp_tr * temp_tr;

                if temp_r.abs() == 4 {
                    winVal = temp_r;
                } else if temp_b.abs() == 4 {
                    winVal = temp_b;
                } else if temp_br.abs() == 4 {
                    winVal = temp_br;
                } else if temp_tr.abs() == 4 {
                    winVal = temp_tr;
                }

            }
        }
        
        return (winVal, chainVal);
    }

    pub fn value(&self, ai_move_value: i64, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64) -> (i64, i64) {
        let val = self.check_state(state);
        if (depth >= 4) { // if slow (or memory consumption is high), lower the value
            let mut retValue = 0;

            // if win, value = +inf
            let winVal = val.0;
            let chainVal = val.1 * ai_move_value;
            retValue = chainVal;

            // If it lead to winning, then do it
            if (winVal == 4 * ai_move_value) { // AI win, AI wants to win of course
                retValue = 999999;
            } else if (winVal == 4 * ai_move_value * -1) { // AI lose, AI hates losing
                retValue = 999999 * -1;
            }
            retValue -= depth * depth;

            return (retValue, -1);
        }

        let win = val.0;
        // if already won, then return the value right away
        if (win == 4 * ai_move_value) { // AI win, AI wants to win of course
            return (999999 - depth * depth, -1);
        }
        if (win == 4 * ai_move_value * -1) { // AI lose, AI hates losing
            return (999999 * -1 - depth * depth, -1);
        }

        if (depth % 2 == 0) {
            return self.minState(ai_move_value, state, depth + 1, alpha, beta);
        }
        return self.maxState(ai_move_value, state, depth + 1, alpha, beta);
    }

    pub fn maxState(&self, ai_move_value: i64, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64) -> (i64, i64) {
        let mut v = -100000000007;
        let mut new_move: i64 = -1;
        let mut moveQueue = Vec::new();
        
        for j in 0..7 {
            let tempState = self.fill_map(state, j, ai_move_value);
            if (tempState[0][0] != 999) {

                let tempVal = self.value(ai_move_value, &tempState, depth, alpha, beta);
                if (tempVal.0 > v) {
                    v = tempVal.0;
                    new_move = j as i64;
                    moveQueue = Vec::new();
                    moveQueue.push(j);
                } else if (tempVal.0 == v) {
                    moveQueue.push(j);
                }

                // alpha-beta pruning
                if (v > beta) {
                    new_move = self.choose(&moveQueue);
                    return (v, new_move);
                }
                alpha = std::cmp::max(alpha, v);
            }
        }
        new_move = self.choose(&moveQueue);

        return (v, new_move);
    }

    pub fn minState(&self, ai_move_value: i64, state: &Vec<Vec<i64>>, depth: i64, mut alpha: i64, mut beta: i64) -> (i64, i64) {
        let mut v = 100000000007;
        let mut new_move: i64 = -1;
        let mut moveQueue = Vec::new();
        
        for j in 0..7 {
            let tempState = self.fill_map(state, j, ai_move_value * -1);
            if (tempState[0][0] != 999) {

                let tempVal = self.value(ai_move_value, &tempState, depth, alpha, beta);
                if (tempVal.0 < v) {
                    v = tempVal.0;
                    new_move = j as i64;
                    moveQueue = Vec::new();
                    moveQueue.push(j);
                } else if (tempVal.0 == v) {
                    moveQueue.push(j);
                }

                // alpha-beta pruning
                if (v < alpha) {
                    new_move = self.choose(&moveQueue);
                    return (v, new_move);
                }
                beta = std::cmp::min(beta, v);
            }
        }
        new_move = self.choose(&moveQueue);

        return (v, new_move);
    }

    // get random int 0 <= x <= val
    pub fn get_random_val(&self, val: usize) -> usize {
        let rand = js! { return Math.random(); };
        let base: f64 = rand.try_into().unwrap();
        let max_val = val as f64;

        return (base * max_val).floor() as usize;
    }

    pub fn choose(&self, choice: &Vec<usize>) -> i64 {
        let index = self.get_random_val(choice.len());
        return choice[index] as i64;
    }

    pub fn ai(&mut self, ai_move_value: i64) {
        let new_map = self.map.clone();
        let val_choice = self.maxState(ai_move_value, &new_map, 0, -100000000007, 100000000007);
        
        let val = val_choice.0;
        let choice = val_choice.1;
        
        // log::info!("AI move value {} choose column: {}, value: {}", ai_move_value, choice, val);

        self.paused = false;
        // TODO: Add rejectclick callback
        let mut done = self.action(choice as usize, true);

        // TODO: Add rejectclick callback
        while done < 0 {
            // log::info!("Using random agent");
            let random_choice = self.get_random_val(7);
            done = self.action(random_choice, true);
        }
    }

    pub fn fill_map(&self, new_state: &Vec<Vec<i64>>, column: usize, value: i64) -> Vec<Vec<i64>> {
        let mut tempMap = new_state.clone();
        if (tempMap[0][column] != 0 || column < 0 || column > 6) {
            tempMap[0][0] = 999; // error code
        }

        let mut done = false;
        let mut row = 0;

        for i in 0..5 {
            if (tempMap[i + 1][column] != 0) {
                done = true;
                row = i;
                break;
            }
        }
        if (!done) {
            row = 5;
        }

        tempMap[row][column] = value;
        return tempMap;
    }

    pub fn draw_circle(&self, x: u32, y: u32, fill: &str, stroke: &str) {
        self.ctx.as_ref().unwrap().save();
        self.ctx.as_ref().unwrap().set_fill_style_color(&fill);
        self.ctx.as_ref().unwrap().set_stroke_style_color(&stroke);
        self.ctx.as_ref().unwrap().begin_path();
        self.ctx
            .as_ref()
            .unwrap()
            .arc(x as f64, y as f64, 25.0, 0.0, 2.0 * 3.14159265359, false);
        self.ctx.as_ref().unwrap().fill(FillRule::NonZero);
        self.ctx.as_ref().unwrap().restore();
    }

    pub fn draw_mask(&self) {
        self.ctx.as_ref().unwrap().save();
        self.ctx.as_ref().unwrap().set_fill_style_color("#00bfff");
        self.ctx.as_ref().unwrap().begin_path();
        for y in 0..6 {
            for x in 0..7 {
                self.ctx.as_ref().unwrap().arc(
                    (75 * x + 100) as f64,
                    (75 * y + 50) as f64,
                    25.0,
                    0.0,
                    2.0 * 3.14159265359,
                    false,
                );
                self.ctx.as_ref().unwrap().rect(
                    (75 * x + 150) as f64,
                    (75 * y) as f64,
                    -100.0,
                    100.0,
                );
            }
        }
        self.ctx.as_ref().unwrap().fill(FillRule::NonZero);
        self.ctx.as_ref().unwrap().restore();
    }

    pub fn draw(&self) {
        for y in 0..6 {
            for x in 0..7 {
                let mut fg_color = "transparent";
                if (self.map[y][x] >= 1) {
                    fg_color = "#ff4136";
                } else if (self.map[y][x] <= -1) {
                    fg_color = "#ffff00";
                }
                self.draw_circle(
                    (75 * x + 100) as u32,
                    (75 * y + 50) as u32,
                    &fg_color,
                    "black",
                );
            }
        }
    }

    pub fn check(&mut self) {
        let (mut temp_r, mut temp_b, mut temp_br, mut temp_tr) = (0, 0, 0, 0);
        for i in 0..6 {
            for j in 0..7 {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;
                for k in 0..=3 {
                    if (j + k < 7) {
                        temp_r += self.map[i][j + k];
                    }

                    if (i + k < 6) {
                        temp_b += self.map[i + k][j];
                    }

                    if (i + k < 6 && j + k < 7) {
                        temp_br += self.map[i + k][j + k];
                    }

                    if (i >= k && j + k < 7) {
                        temp_tr += self.map[i - k][j + k];
                    }
                }
                if temp_r.abs() == 4 {
                    self.win(temp_r);
                } else if temp_b.abs() == 4 {
                    self.win(temp_b);
                } else if temp_br.abs() == 4 {
                    self.win(temp_br);
                } else if temp_tr.abs() == 4 {
                    self.win(temp_tr);
                }
                // log::debug!("values: {} {} {} {}", temp_r, temp_b, temp_br, temp_tr);
            }
        }
        // check if draw
        if ((self.current_move == 42) && (!self.won)) {
            self.win(0);
        }
    }

    pub fn clear(&self) {
        self.ctx.as_ref().unwrap().clear_rect(
            0.0,
            0.0,
            self.canvas.as_ref().unwrap().width() as f64,
            self.canvas.as_ref().unwrap().height() as f64,
        );
    }

    pub fn on_region(&self, coord: f64, x: f64, radius: f64) -> bool {
        return ((coord - x) * (coord - x) <= radius * radius);
    }

    pub fn player_move(&self) -> i64 {
        if (self.current_move % 2 == 0) {
            return 1;
        }
        return -1;
    }

    pub fn animate(&mut self, column: usize, current_move: i64, to_row: usize, cur_pos: usize, mode: bool) {
        let mut fg_color = "transparent";
        if (current_move >= 1) {
            fg_color = "#ff4136";
        } else if (current_move <= -1) {
            fg_color = "#ffff00";
        }

        if (to_row * 75 >= cur_pos) {
            self.clear();
            self.draw();
            self.draw_circle(
                (75 * column + 100) as u32,
                (cur_pos + 50) as u32,
                &fg_color,
                "black",
            );
            self.draw_mask();

            let cloned = self.animate_cbk.clone();
            window().request_animation_frame(enclose!((cloned) move |_| {
                cloned.emit((column, current_move, to_row, cur_pos+25, mode));
            }));
        } else {
            self.map[to_row][column] = self.player_move();
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

    pub fn action(&mut self, column: usize, mode: bool) -> i64 {
        if (self.paused || self.won) {
            return 0;
        }

        if (self.map[0][column] != 0 || column < 0 || column > 6) {
            return -1;
        }

        let mut done = false;
        let mut row = 0;
        for i in 0..5 {
            if (self.map[i + 1][column] != 0) {
                done = true;
                row = i;
                break;
            }
        }
        if (!done) {
            row = 5;
        }

        self.animate(column, self.player_move(), row, 0, mode);

        self.paused = true;
        return 1;
    }

    pub fn win(&mut self, player: i64) {
        self.paused = true;
        self.won = true;
        self.reject_click = false;

        // log::debug!(
        //     "players {} {}",
        //     self.props.player1.as_ref().unwrap(),
        //     self.props.player2.as_ref().unwrap()
        // );

        let mut msg = String::new();
        if (player > 0) {
            msg = format!("{} wins", self.props.player1.as_ref().unwrap());
        } else if (player < 0) {
            msg = format!("{} wins", self.props.player2.as_ref().unwrap());
        } else {
            msg = "It's a draw".to_string();
        }
        
        let to_print = format!("{} - Click on game board to reset", msg);
        
        self.ctx.as_ref().unwrap().save();
        self.ctx.as_ref().unwrap().set_font("14pt sans-serif");
        self.ctx.as_ref().unwrap().set_fill_style_color("#111");
        self.ctx
            .as_ref()
            .unwrap()
            .fill_text(&to_print, 150.0, 20.0, None);

        // TODO Some backend
        // postService.save($scope.newGame, function(){
        //     console.log("succesfully saved");
        // });

        self.ctx.as_ref().unwrap().restore();
    }
}

impl Component for CanvasModel {
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let canvas_id = String::from("canvas");

        let mut map: Vec<Vec<i64>> = vec![vec![0; 7]; 6];

        Self {
            props,
            canvas_id,
            canvas: None,
            ctx: None,
            cbk: link.callback(|e: ClickEvent| Message::Click(e)),
            animate_cbk: link.callback(|e: (usize, i64, usize, usize, bool)| Message::AnimateCallback(e)),
            map,
            current_move: 0,
            paused: false,
            won: false,
            reject_click: false,
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
                    if (self.on_region(x, (75 * j + 100) as f64, 25 as f64)) {
                        self.paused = false;
                        
                        let valid = self.action(j, false);
                        if valid == 1 {
                            self.reject_click = true;
                        };
                        
                        break;
                    }
                }
            }
            Message::AnimateCallback((a, b, c, d, e)) => {
                // log::info!("called that");
                self.animate(a, b, c, d, e);
            }
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
        true
    }
}

fn canvas(id: &str) -> CanvasElement {
    document()
        .query_selector(&format!("#{}", id))
        .unwrap()
        .expect(&format!("Failed to select canvas id #{}", id))
        .try_into()
        .unwrap()
}

fn context(id: &str) -> CanvasRenderingContext2d {
    canvas(id).get_context().unwrap()
}
