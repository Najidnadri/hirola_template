use hirola::prelude::*;
use crate::styling;


//--------- PAGE STATE -----------
pub struct PageState {
    _item: Signal<Blank>
}

#[derive(Default)]
pub struct Blank {
    _num: u32
}

impl Default for PageState {
    fn default() -> Self {
        Self { _item: Signal::new(Blank::default()) }
    }
}


//------- PAGE STATE IMPL ------






//------------ RENDERER -----------
#[component]
pub fn BlankComponent(_app: &HirolaApp) -> Dom {
    let _state = PageState::default();
    html!{
        <div>
            <Style />
        </div>
    }
}



//--------- STYLING ---------
styling! {Style, r##"

"##}