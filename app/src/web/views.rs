use maud::{DOCTYPE, Markup, html};

pub fn layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                title { (title) }
            }
            body {
                nav {
                    a href="/" { "Live" }
                    " "
                    a href="/history" { "History" }
                }
                main { (content) }
            }
        }
    }
}
