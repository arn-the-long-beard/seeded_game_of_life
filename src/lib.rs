// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod universe;
mod utils;

use crate::universe::{Cell, Universe};
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::PrepareCanvas);

    Model {
        cell_size: 5,
        grid_color: "#CCCCCC".to_string(),
        dead_color: "#FFFFFF".to_string(),
        alive_color: "#000000".to_string(),
        pause: false,
        universe: Universe::new(),
        canvas_height: 0,
        canvas_width: 0,
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
}
// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    PrepareCanvas,
    Play,
    Pause,
    Draw,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::PrepareCanvas => {
            model.canvas_width = (model.cell_size + 1) * model.universe.width() + 1;
            model.canvas_height = (model.cell_size + 1) * model.universe.height() + 1;
            orders.after_next_render(|_| Msg::Draw);
        }
        Msg::Draw => {
            if model.pause {
            } else {
                model.universe.tick();
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

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    if model.canvas_width == 0 && model.canvas_height == 0 {
        div!["Loading canvas"]
    } else {
        section![
            button![
                id!("play-pause"),
                if model.pause {
                    ev(Ev::Click, |_| Msg::Play)
                } else {
                    ev(Ev::Click, |_| Msg::Pause)
                },
                if model.pause { "▶" } else { "⏸" }
            ],
            canvas![id!("game-of-life-canvas")],
        ]
    }
}

//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
