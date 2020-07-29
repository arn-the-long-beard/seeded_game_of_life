use crate::universe::Cell;
use crate::{Model};
use seed::prelude::wasm_bindgen::__rt::std::cmp;
use seed::{prelude::*, *};

/// Get the canvas and write grid on it depending
pub fn draw_grid(model: &mut Model) {
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
/// Update the canvas depending of the universe state
pub fn draw_cells(model: &mut Model) {
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

pub fn find_cell_from_click(model: &mut Model, event: web_sys::MouseEvent) -> (u32, u32) {
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

    (row, col)
}
