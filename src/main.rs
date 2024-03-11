use yew::prelude::*;

#[function_component(App)]
fn app() -> Html { 
    let b: &str = "diman";
    let s: String = format!("{}",b);
    html! {
        <h1 class="Petro">{ "Hello " } { b}</h1>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}