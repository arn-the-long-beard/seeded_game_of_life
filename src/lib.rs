// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod fps;
pub mod universe;

use crate::fps::FpsCounter;
use crate::universe::{Cell, Universe};
use seed::browser::util::get_value;
use seed::{prelude::*, *};
use std::cmp;

// ------ ------
//     Init
// ------ ------

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
        fps: FpsCounter::new(),
    }
}

// ------ ------
//     Model
// ------ ------

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
// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
// #[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Play,
    Pause,
    Draw,
    Destroy,
    Random,
    CellClick(web_sys::MouseEvent),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Draw => {
            if model.pause {
            } else {
                let fps = document().get_element_by_id("fps").unwrap();
                let stats = model.fps.render();

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
                let tick_number = tick_frequency.parse::<i32>().unwrap();
                for i in 0..tick_number {
                    model.universe.tick();
                }

                draw_grid(model);
                draw_cells(model);
                orders.after_next_render(|_| Msg::Draw);
            }
        }
        Msg::Play => {
            model.pause = false;
            orders.after_next_render(|_| Msg::Draw);
        }
        Msg::Pause => model.pause = true,
        Msg::Destroy => model.universe = Universe::death(),
        Msg::Random => model.universe = Universe::random(),
        Msg::CellClick(event) => {
            let canvas = canvas("game-of-life-canvas").unwrap();
            let bounding_rect = canvas.get_bounding_client_rect();

            let scale_x: f64 = f64::from(canvas.width()) / bounding_rect.width();
            let scale_y: f64 = f64::from(canvas.height()) / bounding_rect.height();

            let canvas_left: f64 = (f64::from(event.client_x()) - bounding_rect.left()) * scale_x;
            let canvas_top: f64 = (f64::from(event.client_y()) - bounding_rect.top()) * scale_y;

            let row_pos: f64 = (canvas_top / f64::from(model.cell_size + 1)).floor();
            let col_pos: f64 = (canvas_left / f64::from(model.cell_size + 1)).floor();

            let row: u32 = cmp::min(row_pos as u32, model.universe.height() - 1);
            let col: u32 = cmp::min(col_pos as u32, model.universe.width() - 1);

            model.universe.toggle_cell(row, col);
        }
    }
}

fn draw_grid(model: &mut Model) {
    let canvas = canvas("game-of-life-canvas").unwrap();
    canvas.set_width(model.canvas_width);
    canvas.set_height(model.canvas_height);
    let ctx = seed::canvas_context_2d(&canvas);
    ctx.begin_path();
    ctx.set_stroke_style(&JsValue::from_str(model.grid_color.as_str()));

    // Vertical lines.
    for i in 0..model.universe.width() {
        ctx.move_to((i * (model.cell_size + 1) + 1).into(), 0.);
        ctx.line_to(
            (i * (model.cell_size + 1) + 1).into(),
            ((model.cell_size + 1) * model.universe.height() + 1).into(),
        );
    }
    // Horizontal lines.
    for j in 0..model.universe.height() {
        ctx.move_to(0., (j * (model.cell_size + 1) + 1).into());
        ctx.line_to(
            ((model.cell_size + 1) * model.universe.width() + 1).into(),
            (j * (model.cell_size + 1) + 1).into(),
        )
    }

    ctx.stroke();
}

fn draw_cells(model: &mut Model) {
    let canvas = canvas("game-of-life-canvas").unwrap();
    let ctx = seed::canvas_context_2d(&canvas);
    ctx.begin_path();

    // Alive cells.
    ctx.set_fill_style(&JsValue::from_str(model.alive_color.as_str()));
    for row in 0..model.universe.height() {
        for col in 0..model.universe.width() {
            let idx = model.universe.get_index(row, col);
            if model.universe.cell_at_index(idx) != Cell::Alive {
                continue;
            }

            ctx.fill_rect(
                (col * (model.cell_size + 1) + 1).into(),
                (row * (model.cell_size + 1) + 1).into(),
                (model.cell_size).into(),
                (model.cell_size).into(),
            );
        }
    }

    // Dead cells.
    ctx.set_fill_style(&JsValue::from_str(model.dead_color.as_str()));
    for row in 0..model.universe.height() {
        for col in 0..model.universe.width() {
            let idx = model.universe.get_index(row, col);
            if model.universe.cell_at_index(idx) != Cell::Dead {
                continue;
            }

            ctx.fill_rect(
                (col * (model.cell_size + 1) + 1).into(),
                (row * (model.cell_size + 1) + 1).into(),
                (model.cell_size).into(),
                (model.cell_size).into(),
            );
        }
    }

    ctx.stroke();
}

fn fps() {}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
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

//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
