use leptos::*;
use leptos::logging::log;
use server_fn::codec::{ MultipartData, MultipartFormData };
#[cfg(feature = "ssr")]
use std::sync::{
    atomic::{AtomicU8, Ordering},
    Mutex,
};
use wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement, SubmitEvent};

use crate::app::components::recipe_server_functions::*;

/// Download all recipes button
/// Renders the home page of your application.
#[component]
pub fn DownloadAll() -> impl IntoView {
    let all_recipes = create_resource(
        || (),
        |_| {
            async move {
                match get_all_recipes_as_json().await {
                    Ok(content) => Some(content),
                    Err(e) => {
                        log!("{:?}", e.to_string());
                        None
                    }
                }
            }
        }
    );

    view! {
        <Suspense
            fallback=move || view!{ <p>"Recipes loading."</p> <br/> <p>"Please wait..."</p> }
        >
            {move || {
                let all_recipes_fetched = all_recipes.get();
                if let Some(Some(data)) = all_recipes_fetched {
                    let encoded_data = format!("data:text/plain;charset=utf-8,{}", urlencoding::encode(&data));
                    view!{
                        <a href={encoded_data} download="all_recipes_json.txt">
                            "Download All Recipes"
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


#[component]
pub fn UploadAll() -> impl IntoView {
    /// A simple file upload function, which does just returns the length of the file.
    ///
    /// On the server, this uses the `multer` crate, which provides a streaming API.
    #[server(
        input = MultipartFormData,
    )]
    pub async fn file_length(
        data: MultipartData,
    ) -> Result<usize, ServerFnError> {
        // `.into_inner()` returns the inner `multer` stream
        // it is `None` if we call this on the client, but always `Some(_)` on the server, so is safe to
        // unwrap
        let mut data = data.into_inner().unwrap();

        // this will just measure the total number of bytes uploaded
        let mut count = 0;
        while let Ok(Some(mut field)) = data.next_field().await {
            println!("\n[NEXT FIELD]\n");
            let name = field.name().unwrap_or_default().to_string();
            println!("  [NAME] {name}");
            while let Ok(Some(chunk)) = field.chunk().await {
                let len = chunk.len();
                count += len;
                println!("      [CHUNK] {len}");
                // in a real server function, you'd do something like saving the file here
            }
        }

        Ok(count)
    }

    let upload_action = Action::new(|data: &FormData| {
        // `MultipartData` implements `From<FormData>`
        file_length(data.clone().into())
    });

    view! {
        <h3>File Upload</h3>
        <p>Uploading files is fairly easy using multipart form data.</p>
        <form on:submit=move |ev: SubmitEvent| {
            ev.prevent_default();
            let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
            let form_data = FormData::new_with_form(&target).unwrap();
            upload_action.dispatch(form_data);
        }>
            <input type="file" name="file_to_upload"/>
            <input type="submit"/>
        </form>
        <p>
            {move || {
                if upload_action.input().get().is_none() && upload_action.value().get().is_none()
                {
                    "Upload a file.".to_string()
                } else if upload_action.pending().get() {
                    "Uploading...".to_string()
                } else if let Some(Ok(value)) = upload_action.value().get() {
                    value.to_string()
                } else {
                    format!("{:?}", upload_action.value().get())
                }
            }}

        </p>
    }
}



/*
/// Encodes multipart form data.
///
/// You should primarily use this if you are trying to handle file uploads.
pub struct MultipartFormData;

impl Encoding for MultipartFormData {
    const CONTENT_TYPE: &'static str = "multipart/form-data";
    const METHOD: Method = Method::POST;
}

/// Describes whether the multipart data is on the client side or the server side.
#[derive(Debug)]
pub enum MultipartData {
    /// `FormData` from the browser.
    Client(BrowserFormData),
    /// Generic multipart form using [`multer`]. This implements [`Stream`](futures::Stream).
    Server(multer::Multipart<'static>),
}

impl MultipartData {
    /// Extracts the inner data to handle as a stream.
    ///
    /// On the server side, this always returns `Some(_)`. On the client side, always returns `None`.
    pub fn into_inner(self) -> Option<Multipart<'static>> {
        match self {
            MultipartData::Client(_) => None,
            MultipartData::Server(data) => Some(data),
        }
    }

    /// Extracts the inner form data on the client side.
    ///
    /// On the server side, this always returns `None`. On the client side, always returns `Some(_)`.
    pub fn into_client_data(self) -> Option<BrowserFormData> {
        match self {
            MultipartData::Client(data) => Some(data),
            MultipartData::Server(_) => None,
        }
    }
}

impl From<FormData> for MultipartData {
    fn from(value: FormData) -> Self {
        MultipartData::Client(value.into())
    }
}

impl<CustErr, T, Request> IntoReq<MultipartFormData, Request, CustErr> for T
where
    Request: ClientReq<CustErr, FormData = BrowserFormData>,
    T: Into<MultipartData>,
{
    fn into_req(
        self,
        path: &str,
        accepts: &str,
    ) -> Result<Request, ServerFnError<CustErr>> {
        let multi = self.into();
        Request::try_new_multipart(
            path,
            accepts,
            multi.into_client_data().unwrap(),
        )
    }
}

impl<CustErr, T, Request> FromReq<MultipartFormData, Request, CustErr> for T
where
    Request: Req<CustErr> + Send + 'static,
    T: From<MultipartData>,
    CustErr: 'static,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError<CustErr>> {
        let boundary = req
            .to_content_type()
            .and_then(|ct| multer::parse_boundary(ct).ok())
            .expect("couldn't parse boundary");
        let stream = req.try_into_stream()?;
        let data = multer::Multipart::new(
            stream.map(|data| data.map_err(|e| e.to_string())),
            boundary,
        );
        Ok(MultipartData::Server(data).into())
    }
}
*/