use hirola::prelude::*;
use web_sys::{Event, window};
use crate::components::molecule::header::Header;
use crate::{styling, log, get_element_by_id};


/// WELCOME TO ONE OF THE PAGE IN HIROLA APP!
/// First thing first, if you want a blank template of this page, go to blank_page.rs
/// 
/// One page divided into 3 parts, STATE, RENDERER, and STYLE
/// STATE: Hold the state of the page, you can make it reactive by wrap the value with a Signal wrapper
/// RENDERER: a function that will passed on to the renderer, the function return a Dom
/// STYLE: Hold the style, please note that the style is not scoped and may leak to the child component


//--------- PAGE STATE -----------

pub struct PageState {
    state: Signal<HomeState>,
}

#[derive(Clone)]
pub struct HomeState {
    counter: i32
}

impl Default for HomeState {
    fn default() -> Self {
        HomeState { counter: 0 }
    }
}

impl Default for PageState {
    fn default() -> Self {
        PageState { state: Signal::new(HomeState::default()) }
    }
}


//------- PAGE STATE IMPL ------
impl PageState {
    fn plus_one(&self) -> Box<dyn Fn(Event)> {
        self.state.callback(|st, e: Event| {
            let state = st.get();
            st.set(HomeState{ counter: state.counter + 1});
        })
    }

    fn minus_one(&self) -> Box<dyn Fn(Event)> {
        self.state.callback(|st, _| {
            let state = st.get();
            st.set(HomeState{ counter: state.counter - 1})
        })
    }
}





//------------ RENDERER -----------
pub fn home(app: &HirolaApp) -> Dom {
    let router = app.data::<Router>().unwrap().clone();

    let state = PageState::default();
    let plus_one = state.plus_one();
    let minus_one = state.minus_one();


    html!{
        <div>
            <Style />
            <div class="homebody">
                <Header router={(&router).clone()} />
                <div class="intro">
                    <h1>"What's next?"</h1>
                    <p>"Try reading the official documentation "<a href="https://hirola-docs.vercel.app/" target="blank">"here"</a></p>
                    <div class="counter">
                        <h2>"Meanwhile..."</h2>
                        <p>"Check out this counter I made"</p>
                        <p class="output">{state.state.get().counter}</p>
                        <button on:click=plus_one class="increase" id="inc">"Increment One"</button>
                        <button on:click=minus_one class="decrease">"Decrease One"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}



//--------- STYLING ---------

styling!{Style, r##"
.homebody {
    padding: 2em 5em;
    width: 100%;
    height: 100vh;
    background-color: #191919;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 30px;
}

.intro {
    padding: 1em 2em;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.intro h1 {
    color: #C84B31;
    font-size: 35px;
}

.intro p {
    color: #ECDBBA;
}

.intro a {
    cursor: pointer;
    color: #C84B31;
}

.intro a:hover {
    background-color: rgba(236, 219, 186, 0.1);
    transition: background-color 1s;
}

.counter {
    text-align: center;
    background-color: #2D4263;
    width: 70%;
    margin-top: 40px;
    border-radius: 10px;
    padding: 1em;
}

.counter h2 {
    font-size: 20px;
    font-weight: bold;
    color: #ECDBBA;
}

.output {
    margin: 8px auto;
    color: black !important;
    background-color: #ECDBBA;
    width: 60%;
    border-radius: 5px;
}

.increase {
    background-color: #191919;
    color: #ECDBBA;
    width: 60%;
    margin: 3px auto;
    border-radius: 5px;
    padding: 5px 0;
}

.increase:active {
    transform: scale(1.1);
}

.decrease {
    background-color: #960018;
    color: #ECDBBA;
    width: 60%;
    margin: 3px auto;
    border-radius: 5px;
    padding: 5px 0;
}

.decrease:active {
    transform: scale(1.1);
}
"##}
