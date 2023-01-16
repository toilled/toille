pub mod page {
    use yew::prelude::*;

    #[derive(Clone, PartialEq)]
    pub struct Page {
        pub name: String,
        pub title: String,
        pub body: Vec<String>
    }

    #[derive(Clone, Properties, PartialEq)]
    pub struct PagesDetailsProps {
            pub page: Page,
    }

    #[function_component(PageDetails)]
    pub fn page_details(PagesDetailsProps { page }: &PagesDetailsProps) -> Html {
        html! {
            <article>
                <header>{ page.title.clone() }</header>
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
            let text = line.clone();
            html! {
                <p>{ text }</p>
            }
        }).collect()
    }
}