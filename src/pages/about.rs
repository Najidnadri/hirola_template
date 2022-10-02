use hirola::prelude::*;
use crate::styling;
use crate::components::molecule::header::Header;


//--------- PAGE STATE -----------


//------- PAGE STATE IMPL ------






//------------ RENDERER -----------
pub fn about(app: &HirolaApp) -> Dom {
    let router = app.data::<Router>().unwrap().clone();

    html!{
        <>  
            <div>
                <Style />
                <div class="homebody">
                    <Header router={(&router).clone()} />
                    <div class="about-container">
                        <h1>"This is an About Page"</h1>
                    </div>
                </div>
            </div>
        </>
    }
}



//--------- STYLING ---------
styling! {r##"
.homebody {
    padding: 2em 5em;
    width: 100%;
    height: 100vh;
    background-color: #191919;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 30px;
}

.about-container {
    padding: 1em 2em;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.about-container h1 {
    color: #C84B31;
    font-size: 35px;
}
"##}