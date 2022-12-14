/// This macro will return a `Dom` just like any other Hirola component. However, it will wrap the content with `<style></style>`.
/// 
/// The first argument for this macro is the name of your style. Since this macro will return a component, you can give it a name. 
/// The second argument is the style itself in a `&str` format.
/// 
///  ## Example
/// ```rust, no_run
/// fn your_page(_app: &HirolaApp) -> Dom {
///     html! {
///         <div>
///             <MyStyle />
///             <div>
///                 <h1 class="headline">"Hello"</h1>
///             </div>
///         </div>
///     }
/// }
/// 
/// styling! {MyStyle, r##"
/// .headline {
///     color: red;
/// }
/// "##}
/// ```
#[macro_export(local_inner_macros)]
macro_rules! styling {
    ($name:ident, $style:expr) => {
        #[component]
        pub fn $name() -> Dom {
            html!{
                <style>
                    {$style}
                </style>
            }
        }
    };
}

/// Shorthand for `web_sys::console::log_1()` function from `web_sys` crate. Literally just `console.log()` version of wasm in rust.
/// 
/// ## Example
/// ```rust, no_run
/// fn your_page(_app: &HirolaApp) -> Dom {
///    let custom_callback = Box::new(move |e: Event| {
///         format_log!("{}", e.to_string());
///    });
///     
///     html! {
///         <div>
///             <MyStyle />
///             <div>
///                 <input type="text" oninput=custom_callback />
///             </div>
///         </div>
///     }
/// }
/// ```
/// 
/// ## Note
/// Debugging with this macro will be a bit hard since the value will be printed as a `String` instead of `JsValue`. To print out values as `JsValue`,
/// use [`log!`] instead.
#[macro_export]
macro_rules! format_log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

/// Shorthand for `web_sys::console::log_1()` function from `web_sys` crate. Literally just `console.log()` version of wasm in rust.
/// 
/// ## Example
/// ```rust, no_run
/// fn your_page(_app: &HirolaApp) -> Dom {
///    let custom_callback = Box::new(move |e: Event| {
///         log!(&e.into());
///    });
///     
///     html! {
///         <div>
///             <MyStyle />
///             <div>
///                 <input type="text" oninput=custom_callback />
///             </div>
///         </div>
///     }
/// }
/// ```
/// 
/// ## Note
/// [`log!`] macro will take a [JsValue](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html) as the argument. if you just
/// want to print with a `String`, consider [`format_log!`].
#[macro_export]
macro_rules! log {
    ($data:expr) => {
        web_sys::console::log_1($data);
    }
}

///shorthand for `web_sys::window().unwrap().document().unwrap().get_element_by_id()` function
#[macro_export]
macro_rules! get_element_by_id {
    ($id:expr) => {
        web_sys::window().unwrap().document().unwrap().get_element_by_id($id)
    };
}


///shorthand for `web_sys::window().unwrap().document().unwrap().get_elements_by_class_name()` function
#[macro_export]
macro_rules! get_elements_by_class {
    ($class:expr) => {
        web_sys::window().unwrap().document().unwrap().get_elements_by_class_name($class)
    };
}