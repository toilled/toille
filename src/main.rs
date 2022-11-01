use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct Page {
    id: usize,
    name: String,
    title: String,
    body: String
}

#[derive(Properties, PartialEq)]
struct PagesListProps {
    pages: Vec<Page>,
    on_click: Callback<Page>
}

#[function_component(PagesList)]
fn pages_list(PagesListProps { pages, on_click }: &PagesListProps) -> Html {
    let on_click = on_click.clone();
    pages.iter().map(|page| {
        let on_page_select = {
            let on_click = on_click.clone();
            let page = page.clone();
            Callback::from(move |_| {
                on_click.emit(page.clone())
            })
        };

        html! {
            <li class="menuItem" onclick={on_page_select}>{format!("{}", page.name)}</li>
        }
    }).collect()
}

#[derive(Clone, Properties, PartialEq)]
struct PagesDetailsProps {
    page: Page,
}

#[function_component(PageDetails)]
fn page_details(PagesDetailsProps { page }: &PagesDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ page.title.clone() }</h3>
            <p>{ page.body.clone() }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let pages = vec![
        Page {
            id: 1,
            name: "Home".to_string(),
            title: "Home".to_string(),
            body: "This is the home page of Elliot built using Rust.".to_string(),
        },
        Page {
            id: 2,
            name: "About".to_string(),
            title: "About Me".to_string(),
            body: "I am a BSc (Hons) graduate who has been confident working with computers in some way for most of my life.".to_string(),
        },
        Page {
            id: 3,
            name: "Interests".to_string(),
            title: "My Interestes".to_string(),
            body: "I develop new code ideas as a hobby in my spare time, actively keeping a check on new languages and new computing technologies on the internet.".to_string(),
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