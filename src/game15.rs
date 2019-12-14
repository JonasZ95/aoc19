use aoc19::{AocResult, parse_file, FileType};
use aoc19::days::day05::Data;
use aoc19::days::day13::{Game, Input};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent, UpdateArgs, PressEvent, Button, Key};
use piston::window::{WindowSettings};
use glutin_window::GlutinWindow as Window;
use piston::event_loop::{Events, EventSettings};
use geo::Point;
use itertools::Itertools;


pub struct App {
    gl: GlGraphics,
    data: Data,
    game: Game,
    next_input: Input,
    dt: f64
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let to_pf = |p: Point<usize>| (p.x() as f64, p.y() as f64);
        //let to_tf = |p: (usize, usize)| (p.0 as f64, p.1 as f64);

        const BLACK: [f32; 4] = [0., 0., 0., 1.];
        const GREY: [f32; 4] = [0.5, 0.5, 0.5, 1.];
        const WHITE: [f32; 4] = [1., 1., 1., 1.];
        const RED: [f32; 4] = [1., 0.0, 0.0, 1.];
        const BLUE: [f32; 4] = [0., 1.0, 0.0, 1.];


        //let (w, h) = (args.window_size[0], args.window_size[1]);
        //let shape = to_tf(self.game.shape());
       // let zoom = (w / shape.1, h / shape.0);


        let p_ball =  to_pf(self.game.ball());
        let g_ball = rectangle::square(p_ball.0, p_ball.1, 1.);

        let p_paddle = to_pf(self.game.paddle());
        let g_paddle = rectangle::square(p_paddle.0, p_paddle.1, 1.);

       // let g_score = self.game.score().to_string();

        let g_walls = self.game.walls()
            .map(to_pf)
            .map(|p| rectangle::square(p.0, p.1, 1.))
            .collect_vec();

        let g_blocks = self.game.blocks()
            .map(to_pf)
            .map(|p| rectangle::square(p.0, p.1, 1.))
            .collect_vec();

        self.gl.draw(args.viewport(), move |c, gl| {
            clear(WHITE, gl);

            let transform = c
                .scale(12., 12.)
                .transform;

            ellipse(RED, g_ball, transform, gl);
            rectangle(BLUE, g_paddle, transform, gl);

            for wall in g_walls {
                rectangle(BLACK, wall, transform, gl);
            }

            for block in g_blocks {
                rectangle(GREY, block, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.dt += args.dt;

        if self.dt >= 0.005 {
           /* if self.game.update().unwrap() {
                self.game.set_input(self.next_input);
                self.next_input = Input::Neutral;
            }*/

            self.game.auto_play().unwrap();
            self.game.auto_play().unwrap();
            self.game.auto_play().unwrap();
            self.game.auto_play().unwrap();

            self.dt = 0.;
        }
    }

    fn reset(&mut self) {
        self.game = Game::create(self.data.clone(), true)
            .unwrap();
        self.next_input = Input::Neutral;
        self.game.set_input(self.next_input);
    }
}

fn main() -> AocResult<()> {
    let data: Data = parse_file(FileType::Input, 13, 01)?;
    let game = Game::create(data.clone(), true)?;

    let opengl = OpenGL::V2_1;

    let mut window: Window = WindowSettings::new("Day13 space balls", [1000, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut app = App {
        gl: GlGraphics::new(opengl),
        data,
        game,
        next_input: Input::Neutral,
        dt: 0.
    };

    app.game.set_input(Input::Neutral);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(e) = e.press_args() {
            let input = match e {
                Button::Keyboard(Key::Left) => Some(Input::Left),
                Button::Keyboard(Key::Right) => Some(Input::Right),
                Button::Keyboard(Key::Up) => Some(Input::Neutral),
                Button::Keyboard(Key::R) => {
                    app.reset();
                    None
                },
                _ => None
            };
            if let Some(input) = input {
                app.next_input = input;
            }
        }


        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }

    Ok(())
}