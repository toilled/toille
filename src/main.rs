use yew::prelude::*;
use yew::{classes, html};
pub mod components;
use components::page::{Page, PageContent};
use components::pages_list_mod::PagesList;

#[function_component(App)]
fn app() -> Html {
    let title = "Elliot Dickerson";
    let subtitle = "A site to test things";
    let pages = vec![
        Page {
            name: "Home",
            title: "Home",
            body: html! {
                <>
                    <p>{"This is my home page for testing ideas built using Rust."}</p>
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
                    <p>{"I am currently based in the Gloucestershire area"}</p>
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

    let first_page = pages[0].clone();

    let selected_page = use_state(|| Some(first_page.clone()));

    let on_title_click = {
        let selected_page = selected_page.clone();
        Callback::from(move |_e: MouseEvent| {
            selected_page.set(Some(first_page.clone()))
        })
    };

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
        <main class={classes!("container", "animate__animated", "animate__fadeInBottomRight")}>
            <nav>
                <ul class={classes!("animate__animated", "animate__slideInLeft")}>
                    <li>
                        <hgroup onclick={on_title_click} class={classes!("pointer")}>
                            <h1>{title}</h1>
                            <h2>{subtitle}</h2>
                        </hgroup>
                    </li>
                </ul>
                <ul class={classes!("animate__animated", "animate__slideInRight")}>
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
