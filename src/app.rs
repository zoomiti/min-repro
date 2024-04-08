use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use thiserror::Error;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/min-repro.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Debug, Error)]
#[error("id is invalid")]
pub struct ServerError;

#[server]
pub async fn server(id: i32) -> Result<String, ServerFnError> {
    if id > 5 {
        Err(ServerError)?
    }
    
    Ok(String::from("Success"))
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (id, set_id): (Memo<Option<i32>>, _) = create_query_signal("id");

    let resource = create_resource(id, |id| async move{
        match id {
            Some(id) => server(id).await,
            None => Ok(
            String::from("Not yet")),
        }
    });

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <input
            type="number"
            on:change=move |ev| {
                set_id(event_target_value(&ev).parse().ok());
            }

            prop:value=id
        />
        <Suspense>
            <ErrorBoundary fallback=|_| view! { "Error" }>{resource}</ErrorBoundary>
        </Suspense>
    }
}
