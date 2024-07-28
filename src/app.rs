use leptos::*;
use leptos_meta::{provide_meta_context, Meta};
use leptos_router::*;

use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::main_page::MainPage,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="tauritemplate"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
        }>
            <Routes>
                <Route path="/" view=MainPage/>
            </Routes>
        </Router>
    }
}
