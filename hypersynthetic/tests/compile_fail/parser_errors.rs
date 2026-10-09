use hypersynthetic::prelude::*;

fn main() {
    let _ = html! { <!DOCTYP html> };
    let _ = html! { <!DOCTYPE htm> };
    let _ = html! { <div> "text" };
    let _ = html! { <div> 5 </div> };
    let _ = html! { <p :if={true} :if={false}></p> };
    let _ = html! { <p :for={i in 0..1} :for={j in 0..1}></p> };
}
