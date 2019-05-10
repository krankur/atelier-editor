#![recursion_limit = "128"]

use yew::prelude::*;

use editor::core::model::Model;

fn main() {
    web_logger::init();
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
