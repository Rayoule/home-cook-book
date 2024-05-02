use leptos::*;


// Will display on top of each page in the header
#[component]
pub fn PageName(
    page_name: String,
)  -> impl IntoView {
    view! {
        <div class="page-name">
            <h1>{page_name}</h1>
        </div>
    }
}