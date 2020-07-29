use crate::universe::{ Universe};
use seed::{prelude::*, *};
use seed::util::get_value;
use crate::fps::FpsCounter;

mod canvas;
mod fps;
pub mod universe;
// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Draw);

    let universe = Universe::new();
    let cell_size = 5;
    let canvas_width = (cell_size + 1) * universe.width() + 1;
    let canvas_height = (cell_size + 1) * universe.height() + 1;

    Model {
        cell_size,
        grid_color: "#CCCCCC".to_string(),
        dead_color: "#FFFFFF".to_string(),
        alive_color: "#000000".to_string(),
        pause: false,
        universe,
        canvas_height,
        canvas_width,
        fps:FpsCounter::new()
    }
}

// `Model` describes our app state.
pub struct Model {
    cell_size: u32,
    grid_color: String,
    dead_color: String,
    alive_color: String,
    universe: Universe,
    pause: bool,
    canvas_height: u32,
    canvas_width: u32,
    fps: FpsCounter,
}
// `Msg` describes the different events you can modify state with.
enum Msg {
    /// We need to play the game
    Play,
    /// We need to pause
    Pause,
    /// We need to draw stuff
    Draw,
    /// We need to Destroy the universe
    Destroy,
    /// We need to Generate a random Universe
    Random,
    /// We need to click o9n a cell
    CellClick(web_sys::MouseEvent),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Play => {
            model.pause = false;
            orders.after_next_render(|_| Msg::Draw);
        }
        Msg::Pause => model.pause = true,
        Msg::Draw => {
            if model.pause {
            } else {
                let fps = document().get_element_by_id("fps").unwrap();
                let stats = model.fps.calculate();
                let text = format!(
                    "\
                Frames per Second:
         latest = {:?}
avg of last 100 = {:?}
min of last 100 = {:?}
max of last 100 = {:?}
                \
                ",
                    stats.fps, stats.mean, stats.min, stats.max
                );

                fps.set_text_content(Some(text.as_str()));


                let tick_input = document().get_element_by_id("ticks").unwrap();
                let tick_frequency = get_value(tick_input.as_ref()).unwrap();
                let tick_number = tick_frequency.parse::<u32>().unwrap();

                for i in 0..tick_number {
                    model.universe.tick();
                }
                canvas::draw_grid(model);
                canvas::draw_cells(model);
                orders.after_next_render(|_| Msg::Draw);
            }
        }
        Msg::Destroy => {
            model.universe = Universe::death();
        }
        Msg::Random => {
            model.universe = Universe::random();
        }
        Msg::CellClick(event) => {
            let position = canvas::find_cell_from_click(model, event);

            model.universe.toggle_cell(position.0, position.1);
        }
    }
}

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    section![
    div![id!["fps"]],
        p!["Ticks settings :"],
        div![
            input![
                id!("ticks"),
                1,
                attrs! {
                    At::Name => "ticks",
                    At::Type => "range",
                    At::Min =>"1",
                    At::Max =>"10"
                }
            ],
            label![attrs! { At::For => "ticks"}, "ticks"]
        ],
        button![
            id!("random"),
            ev(Ev::Click, |_| Msg::Random),
            "Random Reset"
        ],
        button![
            id!("destroy"),
            ev(Ev::Click, |_| Msg::Destroy),
            "Ultimate Death"
        ],
        button![
            id!("play-pause"),
            if model.pause {
                ev(Ev::Click, |_| Msg::Play)
            } else {
                ev(Ev::Click, |_| Msg::Pause)
            },
            if model.pause { "▶" } else { "⏸" }
        ],
        canvas![
            id!("game-of-life-canvas"),
            ev(Ev::Click, |event| {
                let mouse_event: web_sys::MouseEvent = event.unchecked_into();
                Msg::CellClick(mouse_event)
            })
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".

    App::start("app", init, update, view);
}
