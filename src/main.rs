mod pages;
mod components;
mod macros;

use std::panic;

use hirola::prelude::*;
use pages::{home::home, about::about};

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    //error logging, dont delete if you want your life to be easier
    panic::set_hook(Box::new(console_error_panic_hook::hook));


    //empty app instance
    let mut app = HirolaApp::new();



    //Add routers
    let mut router = Router::new();
    router.add("/", home);
    router.add("/about", about);


    //register router to app instance
    app.extend(router);



    //start the app
    app.mount(&body, |app: &HirolaApp| {
        let router = app.data::<Router>().unwrap().clone();
        let app = app.clone();

        html!{
            <div>
                <style>r##"
                body {
                    margin: 0;
                    box-sizing: border-box;
                }
                "##</style>
                {router.render(&app)}
            </div>
        }
    });

}