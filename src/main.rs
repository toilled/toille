use yew::prelude::*;
pub mod components;
use crate::components::page::{Page, PageDetails};
use crate::components::pages_list_mod::PagesList;

#[function_component(App)]
fn app() -> Html {
    let pages = vec![
        Page {
            name: "Home".to_string(),
            title: "Home".to_string(),
            body: vec![
                "This is the home page of Elliot built using Rust.".to_string(),
                "I'm still learning so it's very basic!".to_string(),
            ],
        },
        Page {
            name: "About".to_string(),
            title: "About Me".to_string(),
            body: vec![
                "I am a BSc (Hons) graduate who has been confident working with computers in some way for most of my life.".to_string(),
            ],
        },
        Page {
            name: "Interests".to_string(),
            title: "My Interestes".to_string(),
            body: vec![
                "I develop new code ideas as a hobby in my spare time, actively keeping a check on new languages and new computing technologies on the internet.".to_string(),
                "I keep up to date with music and play musical instruments including the guitar.".to_string(),
            ],
        },
    ];

    let selected_page = use_state(|| Some(pages[0].clone()));

    let on_page_select = {
        let selected_page = selected_page.clone();
        Callback::from(move |page: Page| {
            selected_page.set(Some(page))
        })
    };

    let details = selected_page.as_ref().map(|page| html! {
            <PageDetails page={page.clone()} />
    });

    html! {
        <>
            <header>
                <h1 class="title">{ "Elliot Dickerson" }</h1>
            </header>
            <nav>
                <ul class="menu">
                    <PagesList pages={pages}  on_click={on_page_select.clone()} />
                </ul>
            </nav>
            <main>
                <div id="content">
                    { for details }
                </div>
            </main>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
