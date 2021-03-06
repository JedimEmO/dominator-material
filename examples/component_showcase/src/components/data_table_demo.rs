use dominator::{clone, html, Dom};
use dominator_material::components::{Card, DataTable};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use wasm_bindgen::__rt::std::rc::Rc;

pub struct DataTableDemo {}

impl DataTableDemo {
    pub fn new() -> DataTableDemo {
        DataTableDemo {}
    }

    pub fn render(self) -> Dom {
        let data: Rc<MutableVec<usize>> = Rc::new(MutableVec::new_with_values((0..10).collect()));

        let current_top = Mutable::new(0);

        let table = DataTable::new(data.clone())
            .row_render_func(|v| {
                html!("tr", {
                    .children(&mut [
                        html!("td", {
                            .text(format!("{}", v).as_str())
                        }),
                        html!("td", {
                            .text(lipsum::lipsum_words_from_seed(5, *v as u64).as_str())
                        })
                    ])
                })
            })
            .headers(vec!["Column 1".to_string(), "Column 2".to_string()])
            .page_meta(
                Mutable::new(10),
                Mutable::new(100000),
                current_top.clone(),
                clone!(data, current_top => move |v, w| {
                    data.lock_mut().replace_cloned((v..(v+w)).collect());
                    current_top.replace(v);
                }),
                Some(vec![10, 20, 50]),
            )
            .render();

        Card::new()
            .title(
                "Data table with pagination",
                Some("Page change triggers data regeneration"),
            )
            .apply(|v| v.class("demo-card"))
            .body(table)
            .render()
    }
}
