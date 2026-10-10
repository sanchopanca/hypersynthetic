//! `cargo bench -p hypersynthetic --bench render`
//!
//! - `render/*` measures `to_string()` on a fragment built beforehand.
//! - `build/*` measures running `html!` (and components) to build the fragment.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use hypersynthetic::prelude::*;
use hypersynthetic::{ElementData, Node};

#[component]
fn Row(id: usize, name: &str, email: &str) -> HtmlFragment {
    html! {
        <tr id="row-{id}" class="row">
            <td class="id">{id}</td>
            <td><a href="/users/{id}">{name}</a></td>
            <td>{email}</td>
        </tr>
    }
}

/// A realistic page: a table of rows built by a component.
fn table(rows: &[(usize, String, String)]) -> HtmlFragment {
    html! {
        <!DOCTYPE html>
        <html>
            <body>
                <table class="users">
                    <Row :for={(id, name, email) in rows} id={*id} name={name} email={email} />
                </table>
            </body>
        </html>
    }
}

fn rows(count: usize) -> Vec<(usize, String, String)> {
    (0..count)
        .map(|id| (id, format!("User <{id}>"), format!("user{id}@example.com")))
        .collect()
}

#[component]
fn Article(title: &str) -> HtmlFragment {
    html! {
        <article>
            <h2>{title}</h2>
            <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor."</p>
            <p>"Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip."</p>
            <p>"Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore."</p>
            <footer>"Posted in " <a href="/blog">"the blog"</a>"."</footer>
        </article>
    }
}

/// Mostly literal text, like a static page.
fn articles(titles: &[String]) -> HtmlFragment {
    html! {
        <main>
            <Article :for={title in titles} title={title} />
        </main>
    }
}

/// Deep nesting: the worst case for copying text once per level.
fn nested(depth: usize) -> HtmlFragment {
    let mut fragment = html! { <span>"A leaf with some text that has to reach the top"</span> };
    for _ in 0..depth {
        fragment = html! { <div class="level">{fragment}</div> };
    }
    fragment
}

/// The same tree as `nested`, built by moving each level into the next instead of
/// inserting it with `{fragment}`, which copies it. The difference to `nested` is
/// the cost of those copies.
fn nested_moved(depth: usize) -> HtmlFragment {
    let mut node = Node::Element({
        let mut span = ElementData::new("span");
        span.add_child(Node::Text(
            "A leaf with some text that has to reach the top".into(),
        ));
        span
    });
    for _ in 0..depth {
        let mut div = ElementData::new("div");
        div.set_attribute("class", "level");
        div.add_child(node);
        node = Node::Element(div);
    }
    HtmlFragment::new(vec![node])
}

#[component]
fn Card(children: HtmlFragment, title: &str) -> HtmlFragment {
    html! {
        <div class="card">
            <h3>{title}</h3>
            <table>{children}</table>
        </div>
    }
}

#[component]
fn Section(children: HtmlFragment, heading: &str) -> HtmlFragment {
    html! {
        <section>
            <h2>{heading}</h2>
            {children}
        </section>
    }
}

#[component]
fn Layout(children: HtmlFragment, title: &str) -> HtmlFragment {
    html! {
        <!DOCTYPE html>
        <html>
            <head><title>{title}</title></head>
            <body>
                <nav><a href="/">"Home"</a></nav>
                <main>{children}</main>
            </body>
        </html>
    }
}

/// A realistic page built from components with slots, three levels deep: each
/// level inserts its children with `{children}`.
fn layout(rows: &[(usize, String, String)]) -> HtmlFragment {
    html! {
        <Layout title="Users">
            <Section heading="All users">
                <Card :for={(id, name, email) in rows} title={name}>
                    <Row id={*id} name={name} email={email} />
                </Card>
            </Section>
        </Layout>
    }
}

/// The same HTML as `layout`, written inline: no slots, so nothing is copied.
fn layout_inline(rows: &[(usize, String, String)]) -> HtmlFragment {
    html! {
        <!DOCTYPE html>
        <html>
            <head><title>"Users"</title></head>
            <body>
                <nav><a href="/">"Home"</a></nav>
                <main>
                    <section>
                        <h2>"All users"</h2>
                        <div :for={(id, name, email) in rows} class="card">
                            <h3>{name}</h3>
                            <table>
                                <tr id="row-{id}" class="row">
                                    <td class="id">{id}</td>
                                    <td><a href="/users/{id}">{name}</a></td>
                                    <td>{email}</td>
                                </tr>
                            </table>
                        </div>
                    </section>
                </main>
            </body>
        </html>
    }
}

/// Lots of text that needs escaping.
fn text_heavy(paragraphs: usize) -> HtmlFragment {
    let text = "Fish & chips <cheap> \"quoted\" ".repeat(20);
    html! {
        <article>
            <p :for={_ in 0..paragraphs} title={&text}>{&text}</p>
        </article>
    }
}

fn render(c: &mut Criterion) {
    let mut group = c.benchmark_group("render");

    let page = table(&rows(100));
    group.bench_function("table_100", |b| b.iter(|| black_box(&page).to_string()));

    let deep = nested(100);
    group.bench_function("nested_100", |b| b.iter(|| black_box(&deep).to_string()));

    let article = text_heavy(50);
    group.bench_function("text_heavy_50", |b| {
        b.iter(|| black_box(&article).to_string())
    });

    group.finish();
}

fn build(c: &mut Criterion) {
    let mut group = c.benchmark_group("build");

    let data = rows(100);
    group.bench_function("table_100", |b| b.iter(|| table(black_box(&data))));
    group.bench_function("nested_100", |b| b.iter(|| nested(black_box(100))));
    // Only a fair comparison if both build the same tree
    assert_eq!(nested(100), nested_moved(100));
    group.bench_function("nested_100_moved", |b| {
        b.iter(|| nested_moved(black_box(100)))
    });
    group.bench_function("layout_100", |b| b.iter(|| layout(black_box(&data))));
    assert_eq!(layout(&data), layout_inline(&data));
    group.bench_function("layout_100_inline", |b| {
        b.iter(|| layout_inline(black_box(&data)))
    });

    let titles: Vec<String> = (0..100).map(|i| format!("Article {i}")).collect();
    group.bench_function("articles_100", |b| b.iter(|| articles(black_box(&titles))));

    group.finish();
}

criterion_group!(benches, render, build);
criterion_main!(benches);
