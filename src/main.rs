mod pages;
mod components;
mod macros;

use hirola::prelude::*;
use pages::{home::home, about::about};

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();


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
                {router.render(&app)}
            </div>
        }
    });

}