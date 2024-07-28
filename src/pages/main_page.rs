use leptos::*;

#[component]
pub fn MainPage() -> impl IntoView {
    view! {
        <div>
            <div class="bg-body-secondary sticky-top">
                <div class="d-block">
                    <h1 class="d-inline">{"tauri-template"}</h1>
                    <div class="float-end bg-body-secondary d-inline">
                        <a href="/" class="btn btn-primary mx-1"><span><img src={"/public/some_icon.svg"} class="icon"/></span></a>
                    </div>
                </div>
            </div>
        </div>
    }
}
