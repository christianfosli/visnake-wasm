extern crate gloo_timers;
extern crate wasm_bindgen;
extern crate web_sys;
use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlElement};

mod snake;
use crate::snake::*;

// Called by our JS entry point
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    add_canvas()?;

    let mut snake = Snake::new();
    draw_snake(&snake)?;

    Interval::new(500, move || {
        let (moved_snake, old_tail) = snake.move_along();
        draw_snake(&moved_snake).unwrap();
        clear_tail(&old_tail, moved_snake.thickness).unwrap();
        snake = moved_snake;
    })
    .forget();

    Ok(())
}

fn add_canvas() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let main_section = match document.query_selector("main")? {
        Some(v) => v.dyn_into::<HtmlElement>()?,
        None => document.body().unwrap(),
    };

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlElement>()?;
    canvas.set_id("canvas");

    main_section.append_child(&canvas)?;

    Ok(())
}

fn get_canvas_context() -> Result<CanvasRenderingContext2d, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .expect("no canvas element could be found")
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    Ok(context)
}

fn draw_snake(snake: &Snake) -> Result<(), JsValue> {
    let context = get_canvas_context()?;
    for pos in snake.body.iter() {
        context.fill_rect(pos.x, pos.y, snake.thickness, snake.thickness);
    }
    Ok(())
}

fn clear_tail(tail: &Position, thickness: f64) -> Result<(), JsValue> {
    let context = get_canvas_context()?;
    context.clear_rect(tail.x, tail.y, thickness, thickness);
    Ok(())
}
