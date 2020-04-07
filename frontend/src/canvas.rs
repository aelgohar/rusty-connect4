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
    canvas_id: String,
    canvas: Option<CanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    cbk: Callback<ClickEvent>,
    map: Vec<Vec<i32>>,
    current_move: i32,
    won: bool,
    paused: bool,
}

impl CanvasModel {
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
                log::debug!("values: {} {} {} {}", temp_r, temp_b, temp_br, temp_tr);
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

    pub fn player_move(&self) -> i32 {
        if (self.current_move % 2 == 0) {
            return 1;
        }
        return -1;
    }

    pub fn animate<F: FnMut()>(
        &mut self,
        column: usize,
        current_move: i32,
        to_row: usize,
        cur_pos: usize,
        mut callback: F,
    ) {
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

            // stdweb::web::window().request_animation_frame(move |_| {});
            self.animate(column, current_move, to_row, cur_pos + 25, callback);
        // window.requestAnimationFrame(function () {
        //     that.animate(column, current_move, to_row, cur_pos + 25, callback);
        // });
        } else {
            self.test(to_row, column);
        }
    }

    pub fn action(&mut self, column: usize) -> i32 {
        // if (self.paused || self.won) {
        //     return 0;
        // }
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

        // self.animate(column, self.player_move(), row, 0, || {
        //     self.map[row][column] = self.player_move();
        //     self.current_move += 1;
        //     self.draw();
        //     // self.check();
        //     // self.print();
        // });
        self.animate(column, self.player_move(), row, 0, || {});
        // self.paused = true;
        return 1;
    }
    pub fn test(&mut self, row: usize, column: usize) {
        self.map[row][column] = self.player_move();
        self.current_move += 1;
        // self.draw();
        self.check();
        // self.print();
    }

    pub fn win(&mut self, player: i32) {
        self.paused = true;
        self.won = true;
        // self.rejectClick = false;

        let mut msg = "won";
        // if (player > 0) {
        //     msg = $scope.newGame.Player1Name + " wins";
        //     $scope.newGame.WinnerName=$scope.newGame.Player1Name;
        // } else if (player < 0) {
        //     msg = $scope.newGame.Player2Name + " wins";
        //     $scope.newGame.WinnerName=$scope.newGame.Player2Name;
        // } else {
        //     msg = "It's a draw";
        //     $scope.newGame.WinnerName='Draw';
        // }
        // msg += " - Click on game board to reset";
        // self.context.save();
        // self.context.font = '14pt sans-serif';
        // self.context.fillStyle = "#111";
        // self.context.fillText(msg, 150, 20);

        // postService.save($scope.newGame, function(){

        //     console.log("succesfully saved");
        // });

        // self.canvas = document.getElementsByTagName("canvas")[0];
        // self.canvas.addEventListener('click', function (e) {
        //     location.reload();
        // });
        // //this.context.restore();
        // button.disabled = false;

        log::debug!("won: {}", msg);
    }
}

pub enum Message {
    SwitchColor(ClickEvent),
}

impl Component for CanvasModel {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let canvas_id = String::from("canvas");

        let mut map: Vec<Vec<i32>> = vec![vec![0; 7]; 6];
        let mut dummyMap: Vec<Vec<i32>> = vec![vec![0; 7]; 6];

        Self {
            canvas_id,
            canvas: None,
            ctx: None,
            cbk: link.callback(|e: ClickEvent| Message::SwitchColor(e)),
            map,
            current_move: 0,
            paused: false,
            won: false,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Message::SwitchColor(e) => {
                log::debug!("received event: {:?}", e);
                let rect = self.canvas.as_ref().unwrap().get_bounding_client_rect();
                let x = e.client_x() as f64 - rect.get_left();

                for j in 0..7 {
                    if (self.on_region(x, (75 * j + 100) as f64, 25 as f64)) {
                        // console.log("clicked region " + j);
                        self.paused = false;
                        self.action(j);
                        break;
                    }
                }
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
        log::trace!("mounted");

        let ctx = self.ctx.as_ref().unwrap();
        let cloned_cbk = self.cbk.clone();

        self.canvas.as_ref().unwrap().add_event_listener(enclose!(
            (ctx) move | event: ClickEvent | {
                // event is handled in a message
                cloned_cbk.emit(event);
            }
        ));

        self.draw_mask();

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
