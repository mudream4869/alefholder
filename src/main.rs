use web_sys::HtmlTextAreaElement;

use yew::{
    events::Event,
    html,
    // Need to import TargetCast
    Component, Context, Html, TargetCast,Properties,

    function_component
};

use pulldown_cmark::{Parser, Options};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(SafeHtml)]
pub fn safe_html(props: &Props) -> Html {
    let div = gloo_utils::document().create_element("div").unwrap();
    div.set_inner_html(&props.html.clone());

    Html::VRef(div.into())
}

enum Msg {
    SetText(String),
}

#[derive(Default)]
struct Model {
    raw_md_text: String,
    html_md: String
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetText(text) => {
                self.raw_md_text = text;
                self.html_md.clear();
                let mut options = Options::empty();
                options.insert(Options::ENABLE_STRIKETHROUGH);
                let parser = Parser::new_ext(self.raw_md_text.as_str(), options);
                pulldown_cmark::html::push_html(&mut self.html_md, parser);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        let onchange = link.batch_callback(|e: Event| {
            let input = e.target_dyn_into::<HtmlTextAreaElement>();
            input.map(|input| Msg::SetText(input.value()))
        });

        html! {
            <div class="container mh-100" style="height: 99vh;">
                <div class="row gx-5 h-100">
                    <div class="col h-100">
                        <textarea class="h-100 w-100" onchange={onchange} ></textarea>
                    </div>
                    <div class="col" id="showHTML">
                        <SafeHtml html={ self.html_md.clone() }/>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}

