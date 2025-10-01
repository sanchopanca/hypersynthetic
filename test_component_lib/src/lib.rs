use hypersynthetic::prelude::*;

#[component]
pub fn Card() -> HtmlFragment {
    html! {
        <div class="card">
            <h2>"Card Title"</h2>
            <p>"This is a card component"</p>
        </div>
    }
}
