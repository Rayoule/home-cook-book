use crate::app::{
    components::recipe_server_functions::*, elements::molecules::LoadingElem, ApplySaveFromJson,
    PopupColor,
};
use leptos::logging::*;
use leptos::*;
use web_sys::SubmitEvent;
extern crate chrono;

/// Download all recipes button
/// Renders the home page of your application.
#[component]
pub fn DownloadAll(has_been_backed_up: RwSignal<bool>) -> impl IntoView {
    let all_recipes = create_resource(
        || (),
        |_| async move {
            match get_all_recipes_as_json_string().await {
                Ok(content) => Some(content),
                Err(e) => {
                    error!("{:?}", e.to_string());
                    None
                }
            }
        },
    );

    view! {
        <Suspense
            fallback=move || view!{ <LoadingElem text="Loading Recipe...".to_owned() /> }
        >
            {move || {
                let all_recipes_fetched = all_recipes.get();
                if let Some(Some(data)) = all_recipes_fetched {
                    let encoded_data = format!("data:text/plain;charset=utf-8,{}", urlencoding::encode(&data));
                    let date_fmt = chrono::Local::now().format("%Y-%m-%d--%H:%M:%S").to_string();
                    let save_name =
                        "all_recipes_json".to_string()
                        + &date_fmt
                        + ".json";

                    view! {
                        <a
                            href =      {encoded_data}
                            download =  &save_name
                            on:click =  move |_| { has_been_backed_up.set(true) }
                            class=      "download-backup-button"
                        >
                            "Download All"
                        </a>
                    }.into_view()
                } else {
                    view!{
                        <p>"Fetched empty data :("</p>
                    }.into_view()
                }
            }}
        </Suspense>
    }
}

#[allow(unused)] // disable the warning on "ev"
#[component]
pub fn UploadAll(has_been_backed_up: RwSignal<bool>) -> impl IntoView {
    // Keep track if the save has be made
    let save_done = create_rw_signal(false);

    // Apply save action
    let upload_save_action = use_context::<ApplySaveFromJson>()
        .expect("Expected to find ApplyJsonSave in context")
        .0;
    let upload_pending = upload_save_action.pending();
    let save_action_value = upload_save_action.value();
    create_effect(move |_| {
        if let Some(true) = save_action_value.get() {
            save_done.set(true);
        }
    });

    // Textarea
    // setup for textarea autosize
    let textarea = create_node_ref::<html::Textarea>();

    #[cfg(feature = "hydrate")]
    let leptos_use::UseTextareaAutosizeReturn {
        content: _,
        set_content,
        trigger_resize: _,
    } = leptos_use::use_textarea_autosize(textarea);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = textarea()
            .expect("Expected testarea to be mounted.")
            .value();
        upload_save_action.dispatch(value);
    };

    let popup_color = create_rw_signal(PopupColor::random());
    create_effect(move |_| {
        let _ = upload_save_action.version().get();
        popup_color.set(PopupColor::random());
    });

    view! {
        <Show
            when = has_been_backed_up
            fallback = move || view! {
                <p class="backup-warning" >
                    "Download all before you can overwrite."
                </p>
            }
        >
            <Show
                when = move || { !save_done.get() }
                fallback = move || view! { <p>"Save has been done !"</p> }
            >
                <Show
                    when = move || { !upload_pending.get() }
                    fallback = move || view! {
                            <div
                                class="popup-window server-action"
                                style=popup_color.get().window_background_color()
                            >
                                <p class="wait-for-server" > "Wait for server ..." </p>
                            </div>
                    }
                >
                    <form
                        on:submit =     on_submit
                        class=          "upload-save-form"
                    >
                        <textarea
                            class=          "save-input"
                            node_ref=       textarea
                            id=             "text-input"
                            type=           "text"
                            placeholder=    "Paste JSON save here"
                            on:input=move |ev| {
                                // resize box to fit text
                                #[cfg(feature= "hydrate")]
                                set_content.set(event_target_value(&ev));
                            }
                        > {} </textarea>
                        <button class="upload-save-button" type="submit"> "Ok" </button>
                    </form>
                </Show>
            </Show>
        </Show>

    }
}
