mod player;
mod utils;
use js_sys::JsString;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::player::player::*;
use wasm_bindgen::JsCast;
use std::rc::Rc;
use web_sys::{Document, Element, HtmlElement, Window};

/// Used for debugging to the console
pub fn exit(message: &str) {
    let v = wasm_bindgen::JsValue::from_str(&message.to_string());
    web_sys::console::exception_1(&v);
    std::process::abort();
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}

fn app(name: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let mut player_storage = PlayerSotorage::new("anime-clasher");
    let mut player_storage: PlayerSotorage = player_storage.unwrap();
    match PlayerSotorage::getPlayer(&player_storage) {
        Some(pl) => pl,
        None => {
            let section = match document.create_element("section") {
                Ok(expr) => expr,
                Err(err) => panic!("erro ao criar section"),
            };

            let span = match document.create_element("span") {
                Ok(expr) => expr,
                Err(err) => panic!("erro ao criar span"),
            };

            span.set_inner_html("Nome: ");

            let mut input = match document.create_element("input") {
                Ok(expr) => expr,
                Err(err) => panic!("erro ao criar input"),
            };

            input.set_attribute("type", "text");
            input.set_attribute("id", "player_name_input");

            let button = match document.create_element("button") {
                Ok(expr) => expr,
                Err(err) => panic!("erro ao criar input"),
            };
            button.set_inner_html("OK");
            button.set_attribute("id", "player_name_button");

            section.append_child(&span);
            section.append_child(&input);
            section.append_child(&button);

            body.append_child(&section);

            let player_name_input = document
                .get_element_by_id("player_name_input")
                .expect("should have #num-clicks on the page");

            let mut name_input_value = String::new();
            //

            let a = Closure::wrap(Box::new(move || {
                name_input_value = value(&mut Some(player_name_input));
                if name_input_value == String::new() {
                    alert("Please informe the name!");
                } else {
                    let mut p = Player {
                        name: name_input_value,
                    };
                    player_storage.insert(p);
                }
            }) as Box<dyn FnMut()>);
            document
                .get_element_by_id("player_name_button")
                .expect("should have #player_name_button on the page")
                .dyn_ref::<HtmlElement>()
                .expect("#player_name_button be an `HtmlElement`")
                .set_onclick(Some(a.as_ref().unchecked_ref()));

            // See comments in `setup_clock` above for why we use `a.forget()`.
            a.forget();

            &Player::new()
        }
    };

    /// Gets the value for the element in `self.el` (The element must be an input)
    pub fn value(el: &mut Option<web_sys::Element>) -> String {
        let mut v = String::new();
        if let Some(el) = el.take() {
            if let Some(el) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlInputElement>(&el) {
                v = el.value();
            }
        }
        v
    }

    // Manufacture the element we're gonna append
    // let val = match document.create_element("p") {
    //     Ok(expr) => expr,
    //     Err(err) => panic!("erro ao criar p"),
    // };
    // val.set_inner_html("Hello from Rust!");

    // body.append_child(&val);
    // body.set_inner_html("")

    // Ok(())
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    app("Anime Clasher");

    Ok(())
}
