pub mod page {
    use yew::prelude::*;

    #[derive(Clone, PartialEq)]
    pub struct Page {
        pub name: &'static str,
        pub title: &'static str,
        pub body: Html
    }

    #[derive(Clone, Properties, PartialEq)]
    pub struct PageContentProps {
            pub page: Page,
    }

    #[function_component(PageContent)]
    pub fn page_content(PageContentProps { page }: &PageContentProps) -> Html {
        html! {
            <article class={classes!("animate__animated", "animate__zoomIn")}>
                <header><h2 class={classes!("no-margin")}>{page.title}</h2></header>
                {page.body.clone()}
            </article>
        }
    }
}