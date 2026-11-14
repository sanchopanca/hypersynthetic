use hypersynthetic::prelude::*;

struct Page {
    title: String,
}

impl Page {
    #[component]
    fn Header(&self, subtitle: &str) -> HtmlFragment {
        html! { <h1>{&self.title}</h1><p>{subtitle}</p> }
    }

    #[component]
    fn Consuming(mut self: Box<Self>) -> HtmlFragment {
        html! { <h1>{std::mem::take(&mut self.title)}</h1> }
    }
}

fn main() {}
