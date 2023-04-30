use yew::prelude::*;
pub mod components;
use crate::components::page::{Page, PageContent};
use crate::components::pages_list_mod::PagesList;

#[function_component(App)]
fn app() -> Html {
    let pages = vec![
        Page {
            name: "Home",
            title: "Home",
            body: html! {
                <>
                    <p>{"This is the home page of Elliot built using Rust."}</p>
                    <p>{"I'm still learning so it's very basic!"}</p>
                    <p>{"I will make the source code available "}<a href={"https://github.com/toilled/toille"}>{"here"}</a>{"."}</p>
                </>
            },
        },
        Page {
            name: "About",
            title: "About Me",
            body: html! {
                <>
                    <p>{"I am a BSc (Hons) graduate who has been confident working with computers in some way for most of my life."}</p>
                </>
            },
        },
        Page {
            name: "Interests",
            title: "My Interestes",
            body: html! {
                <>
                    <p>{"I develop new code ideas as a hobby in my spare time, actively keeping a check on new languages and new computing technologies on the internet."}</p>
                    <p>{"I keep up to date with music and play musical instruments including the guitar."}</p>
                </>
            },
        },
    ];

    let selected_page = use_state(|| Some(pages[0].clone()));

    let on_page_select = {
        let selected_page = selected_page.clone();
        Callback::from(move |page: Page| {
            selected_page.set(Some(page))
        })
    };

    let page_content = selected_page.as_ref().map(|page| html! {
        <PageContent page={page.clone()} />
    });

    html! {
        <main class={"container animate__animated animate__fadeInBottomRight"}>
            <nav>
                <ul class={"animate__animated animate__slideInLeft"}>
                    <li>
                        <hgroup>
                            <h1>{ "Elliot Dickerson" }</h1>
                            <h2>{ "A site to test things" }</h2>
                        </hgroup>
                    </li>
                </ul>
                <ul class={"animate__animated animate__slideInRight"}>
                    <PagesList pages={pages}  on_click={on_page_select.clone()} />
                </ul>
            </nav>
            { for page_content }
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
