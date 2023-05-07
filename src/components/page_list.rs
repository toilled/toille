pub mod pages_list_mod {
    use yew::prelude::*;
    use crate::components::page::Page;

    #[derive(Properties, PartialEq)]
    pub struct PagesListProps {
        pub pages: Vec<Page>,
        pub on_click: Callback<Page>
    }

    #[function_component(PagesList)]
    pub fn pages_list(PagesListProps {pages, on_click}: &PagesListProps) -> Html {
        pages.iter().map(|page| {
            let on_page_select = {
                let on_click = on_click.clone();
                let page = page.clone();
                Callback::from(move |_| {
                    on_click.emit(page.clone())
                })
            };

            html! {
                <li>
                    <a onclick={on_page_select} class={classes!("pointer")}>{page.name}</a>
                </li>
            }
        }).collect()
    }
}