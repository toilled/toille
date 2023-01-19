pub mod page {
    use yew::prelude::*;

    #[derive(Clone, PartialEq)]
    pub struct Page {
        pub name: String,
        pub title: String,
        pub body: Vec<String>
    }

    #[derive(Clone, Properties, PartialEq)]
    pub struct PageContentProps {
            pub page: Page,
    }

    #[function_component(PageContent)]
    pub fn page_content(PageContentProps { page }: &PageContentProps) -> Html {
        html! {
            <article class={"animate__animated animate__zoomIn"}>
                <header><strong>{ page.title.clone() }</strong></header>
                <Paragraphs lines={page.body.clone()} />
            </article>
        }
    }

    #[derive(Properties, PartialEq)]
    pub struct ParagraphProps {
        pub lines: Vec<String>
    }

    #[function_component(Paragraphs)]
    pub fn paragraphs(ParagraphProps { lines }: &ParagraphProps) -> Html {
        lines.iter().map(|line| {
            html! {
                <p>{ line.clone() }</p>
            }
        }).collect()
    }
}