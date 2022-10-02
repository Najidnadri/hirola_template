
#[macro_export(local_inner_macros)]
macro_rules! styling {
    ($style:expr) => {
        #[component]
        pub fn Style() -> Dom {
            html!{
                <style>
                    {$style}
                </style>
            }
        }
    };
}

