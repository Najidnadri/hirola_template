use hirola::prelude::*;

use crate::styling;







//------------ RENDERER -----------
#[component]
pub fn Header(router: Router) -> Dom {  
    html!{ 
        <div>
            <Style />
            <div class="header">
                <h1>"Hirola starts here!"</h1>
                <p>"You've successfully created a project with "
                    <span><a href="https://github.com/geofmureithi/hirola" target="blank">
                        "Hirola"
                    </a></span>
                </p>
                <nav class="nav">
                    <a mixin::route=&router.link() href="/">"Home"</a>
                    <a mixin::route=&router.link() href="/about" class="about">"About Us"</a>
                </nav>
            </div>
        </div> 
    }
}


//--------- STYLING ---------
styling!{Style, r##"
.header {
    padding: 1em 2em;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
}

.header h1 {
    color: #C84B31;
    font-size: 35px;
    font-weight: semibold;  
    margin-bottom: 13px;
}

.header p {
    color: #ECDBBA;
    font-size: 18px;
}

.header a {
    cursor: pointer;
    color: #C84B31;
}

.header a:hover {
    background-color: rgba(236, 219, 186, 0.1);
    transition: background-color 1s;
}

.nav {
    margin-top: 27px;
    display: flex;
    flex-direction: row;
}

.nav a {
    padding: 0 1em;
}

.about {
    border-left: 1px solid grey;
}
"##}
