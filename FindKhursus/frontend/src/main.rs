use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct App {
    link: ComponentLink<Self>,
    courses: Vec<Course>,
    category_filter: Option<String>,
}

struct Course {
    id: u64,
    title: String,
    description: String,
    price: u64,
    category: String,
}

enum Msg {
    FetchCourses,
    SetCategoryFilter(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            link: ctx.link().clone(),
            courses: Vec::new(),
            category_filter: None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchCourses => {
                // Simulasikan pengambilan data kursus dari backend
                self.courses = vec![]; // Ganti dengan fetch data sebenarnya
                true
            }
            Msg::SetCategoryFilter(category) => {
                self.category_filter = Some(category);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Find Kursus" }</h1>
                <div>
                    <input type="text" placeholder="Kategori" oninput={ctx.link().callback(|e: InputData| Msg::SetCategoryFilter(e.value))}/>
                    <button onclick={ctx.link().callback(|_| Msg::FetchCourses)}>{ "Cari Kursus" }</button>
                </div>
                <ul>
                    { for self.courses.iter().map(|course| html! {
                        <li>{ format!("{} - {} - Rp{}/kursus - {}", course.title, course.description, course.price, course.category) }</li>
                    }) }
                </ul>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<App>::new().mount_to_body();
}
