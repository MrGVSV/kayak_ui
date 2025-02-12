use kayak_core::{context::KayakContext, styles::Style, Children, Index};
use kayak_core::{derivative::*, Fragment, Widget};
use kayak_render_macros::rsx;

#[derive(Derivative)]
#[derivative(Clone, Default, Debug, PartialEq)]
#[allow(dead_code)]
struct Test {
    id: Index,
    #[derivative(Default(value = "None"))]
    styles: Option<Style>,
    foo: u32,
    #[derivative(Debug = "ignore", PartialEq = "ignore", Default(value = "None"))]
    children: Children,
    #[derivative(Debug = "ignore", PartialEq = "ignore", Default(value = "None"))]
    pub on_event: Option<kayak_core::OnEvent>,
}

impl Widget for Test {
    fn get_id(&self) -> Index {
        todo!()
    }

    fn focusable(&self) -> Option<bool> {
        None
    }

    fn set_id(&mut self, _id: Index) {
        todo!()
    }

    fn get_styles(&self) -> Option<Style> {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn on_event(&mut self, _context: &mut KayakContext, _event: &mut kayak_core::Event) {
        todo!()
    }

    fn render(&mut self, _context: &mut KayakContext) {
        todo!()
    }
}

fn main() {
    let mut context = KayakContext::new();
    {
        let context = &mut context;
        let foo = 10;
        let test_styles = Style::default();
        let parent_id: Option<Index> = None;
        let children: Option<kayak_core::Children> = None;
        let tree = kayak_core::WidgetTree::new();
        rsx! {
            <Fragment>
                <Test foo={10}>
                    <Test foo={1}>
                        <Test foo={5}>
                            <Test foo={foo} styles={Some(test_styles)}>
                                {}
                            </Test>
                        </Test>
                    </Test>
                </Test>
            </Fragment>
        };

        let foo = 10;
        let test_styles = Style::default();

        let parent_id: Option<Index> = None;
        let children: Option<kayak_core::Children> = None;
        let tree = kayak_core::WidgetTree::new();

        rsx! {
            <Fragment>
                <Test foo={foo} styles={Some(test_styles)}>
                    {}
                </Test>
                <Test foo={5} styles={Some(test_styles)}>
                    {}
                </Test>
            </Fragment>
        }
    }
}
