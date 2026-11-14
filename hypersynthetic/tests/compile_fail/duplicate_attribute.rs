use hypersynthetic::prelude::*;

fn main() {
    let (extra, show) = ("b", true);
    // Browsers use the first and ignore the rest
    let _ = html! { <div class="a" class="b"></div> };
    let _ = html! { <div class="a" class={extra}></div> };
    let _ = html! { <input disabled disabled /> };
    let _ = html! { <div hidden hidden="until-found"></div> };
    // Not only when they're next to each other
    let _ = html! { <a href="/1" title="t" :if={show} href="/2"></a> };
    let _ = html! { <div data-x="1" data-x="2"></div> };
    let _ = html! { <button @click="a()" @click="b()"></button> };
    // Attribute names ignore case in HTML, in SVG too
    let _ = html! { <div ID="a" id="b"></div> };
    let _ = html! { <svg viewBox="0 0 1 1" viewbox="0 0 2 2"></svg> };
    // Inside other elements
    let _ = html! { <ul><li class="a" class="b">"x"</li></ul> };
}
