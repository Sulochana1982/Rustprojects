use std::str::FromStr;
use std::io::{Error, ErrorKind};

 use warp::{Filter, reject::Reject, Rejection, Reply, http::StatusCode};
 use serde::Serialize;




#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags:Option<Vec<String>>,
}



impl Question {
    fn new (id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Question{
        Question{
            id,
            title,
            content,
            tags,
        }
    }
}

#[derive(Debug, Serialize)]

struct QuestionId(String);

impl FromStr for QuestionId{

    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() { 
            false => Ok(QuestionId(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput, "No id Provided")
            )
            
        }
    }
}

#[derive(Debug)]

struct InvalidId;

impl Reject for InvalidId {}

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection>{
    let question = Question::new(
        QuestionId::from_str("1").expect("No id Provided"), 
        "First Question".to_string(),
        "Content of the question".to_string(), 
        Some(vec!("faq".to_string())),
    );

    match question.id.0.parse::<i32>() {
       Err(_) => {
        Err(warp::reject::custom(InvalidId))
       }
        
        Ok(_) => {
            Ok(warp::reply::json(&question))
        }
    }
       

}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {

    if let Some(_invalid_id) = r.find::<InvalidId>() {
        Ok(warp::reply::with_status(
            "No Valid Id Presented", StatusCode::UNPROCESSABLE_ENTITY,
        ))
        
    } else {
        Ok(warp::reply::with_status(
            "Route Not Found", StatusCode::NOT_FOUND,
        ))
    }
}




#[tokio::main]
async fn main() {

  
    let get_items = warp::get()
    .and(warp::path("questions"))
    .and(warp::path::end())
    .and_then(get_questions)
    .recover(return_error);


    let routes = get_items;

    warp::serve(routes)
    .run(([127, 0, 0, 1], 3030))
    .await;
}