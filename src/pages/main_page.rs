use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn MainPage() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    spawn_local(async move {
        let args = to_value(&GreetArgs { name: &name.get() }).unwrap();
        let new_msg = invoke("greet", args).await.as_string().unwrap();
        set_greet_msg.set(new_msg);
    });
    view! {
        <div>
            <div class="bg-body-secondary sticky-top">
                <div class="d-block">
                    <h1 class="d-inline">{"tauritemplate"}</h1>
                    <div class="float-end bg-body-secondary d-inline">
                        <a href="/" class="btn btn-primary mx-1"><span><img src={"/public/some_icon.svg"} class="icon"/></span></a>
                    </div>
                </div>
            </div>
        </div>
    }
}
