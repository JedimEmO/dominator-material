use dominator::{html, Dom};
use wasm_bindgen::__rt::std::rc::Rc;

use dominator_material::components::{Button, ButtonType, Card};

pub struct ButtonDemo {}

impl ButtonDemo {
    pub fn new() -> Rc<ButtonDemo> {
        Rc::new(ButtonDemo {})
    }

    pub fn render(self: Rc<Self>) -> Dom {
        Card::new()
            .apply(|v| v.class("demo-cards"))
            .body(html!("div", {
                .class("demo-cards")
                .class("demo-card")
                .children(&mut[
                    Card::new()
                        .header(html!("div", { .text("ButtonType::Contained") }))
                        .body(Button::new()
                            .text("Click me!")
                            .on_click(|_| {
                                web_sys::window().unwrap().alert_with_message("You clicked?").unwrap();
                            })
                            .render())
                        .render(),
                    Card::new()
                        .header(html!("div", { .text("ButtonType::Text") }))
                        .body(Button::new()
                            .text("Click me!")
                            .button_type(ButtonType::Text)
                            .render())
                        .render(),
                    Card::new()
                        .header(html!("div", { .text("ButtonType::Outlined") }))
                        .body(Button::new()
                            .text("Click me!")
                            .button_type(ButtonType::Outlined)
                            .render())
                        .render(),
                    Card::new()
                        .header(html!("div", { .text("Button with Dom content") }))
                        .body(Button::new()
                            .dom_content(html!("h1", { .text("H1 Dom element")}))
                            .render())
                        .render()
                ])
            }))
            .render()
    }
}
