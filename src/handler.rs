use crate::model::NoteModel;
use crate::{

    schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;
#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, Postgres,and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}
// #[get("/notes/{id}")]
// async fn get_note_handler(
//     path: web::Path<uuid::Uuid>,
//     data: web::Data<AppState>,
// ) -> impl Responder {
//     let note_id = path.into_inner();
//     let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM items WHERE id = $1", note_id)
//         .fetch_one(&data.db)
//         .await;

//     match query_result {
//         Ok(note) => {
//             let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                 "note": note
//             })});

//             return HttpResponse::Ok().json(note_response);
//         }
//         Err(_) => {
//             let message = format!("Note with ID: {} not found", note_id);
//             return HttpResponse::NotFound()
//                 .json(serde_json::json!({"status": "fail","message": message}));
//         }
//     }
// }
#[get("/notes/{barcode}")]
async fn get_note_handler(
    path: web::Path<i64>,
    data: web::Data<AppState>,
) -> impl Responder {
    let barcode = path.into_inner();
    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT * FROM items WHERE barcodestring = $1",
        barcode
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({
                "status": "success",
                "data": {
                    "note": note
                }
            });

            HttpResponse::Ok().json(note_response)
        }
        Err(_) => {
            let message = format!("Note with barcode: {} not found", barcode);
            HttpResponse::NotFound().json(json!({
                "status": "fail",
                "message": message
            }))
        }
    }
}


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
#[post("/notes/")]
async fn create_note_handler(
    body: web::Json<CreateNoteSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO items (barcodestring,title,price) VALUES ($1,$2,$3) RETURNING *",
        body.barcodestring,
        body.title.to_string(),
        body.price,
        
        
        
        
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Note with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
         .service(health_checker_handler)
        // .service(note_list_handler)
        .service(create_note_handler)
          .service(get_note_handler);
        // .service(edit_note_handler)
        // .service(delete_note_handler);

    conf.service(scope);
}
