use dotenv_codegen::dotenv;
use serde_json::{Value, json};
use yew::format::Json;
use yew::Callback;
use yew::services::fetch::{Response, Request,FetchTask,FetchService};
use cynic::GraphQlResponse;
use cynic::impl_scalar;
use cynic::DecodeError;
use anyhow::Error;

// use uuid::Uuid;
const API_GPL: &str = dotenv!("API_GPL");
// const TOKEN_KEY: &str = dotenv!("TOKEN_KEY");

// Define the possible messages which can be sent to the component
pub enum Msg {
    ReceiveResponse(Result<GraphQlResponse<FilmDirectorQuery>, DecodeError>),
}

mod schema {
    cynic::use_schema!("src/schema.graphql");
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "src/schema.graphql",
    query_module = "schema"
)]
struct Film {
    title: Option<String>,
    director: Option<String>,
}

#[derive(cynic::FragmentArguments)]
struct FilmArguments {
    id: Option<cynic::Id>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "src/schema.graphql",
    graphql_type = "Root",
    argument_struct = "FilmArguments"
)]
struct FilmDirectorQuery {
    #[arguments(id = &args.id)]
    film: Option<Film>,
}

pub(crate) fn run_query(&self) -> bool {
    let operation = build_query(); // You'd need to provide this build_query function, similar to the cynic examples
    let request = Request::post(API_GPL)
        .body(Json(json!(operation)))
        .expect("Could not build request.");

    // construct a callback to handle the response
    let callback =
        self.link
            .callback(|response: Response<Json<Result<GraphQlResponse<serde_json::Value>, anyhow::Error>>>| {
                let Json(data) = response.into_body();
                let decoded_data = operation.decode_response(data);
                Msg::ReceiveResponse(decoded_data)
            });

    // pass the request and callback to the fetch service
    let task = FetchService::fetch(request, callback).expect("failed to start request");
    // store the task so it isn't canceled immediately
    let fetch_task = Some(task);

    dbg!(format!("Fetch_task: {:?}", fetch_task));
    // we want to redraw so that the page displays a 'fetching...' message to the user
    // so return 'true'
    true
}

fn build_query() -> cynic::Operation<'static, FilmDirectorQuery> {
    use cynic::QueryBuilder;

    FilmDirectorQuery::build(&FilmArguments {
        id: Some("ZmlsbXM6MQ==".into()),
    })
}
