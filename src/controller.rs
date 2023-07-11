use lambda_http::{http::Method, Body, Request, RequestExt};
use crate::{error, model::{ResponseWrapper, PathParam,QueryParam}, dao, utils};


pub async fn router(req:Request) -> Result<ResponseWrapper, error::Error>{
    let path = req.raw_http_path().to_string();

    let response = match *req.method(){
        Method::GET => get(path, req.query_string_parameters().to_query_string()).await?,
        Method::POST => post(path, req.body()).await?,
        _ => return Err(error::Error::new(405, "Method Not Allowed".to_string())),
    };

    Ok(response)
}

async fn get(path:String, query_params:String) -> Result<ResponseWrapper, error::Error>{
    let body = match utils::parse_path(path, query_params){
        //unsupported urls for get
        (PathParam::NotSupported, QueryParam::NotSupported) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::NotSupported, QueryParam::Query(None)) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::NotSupported, QueryParam::Query(Some(_))) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::Course(None), QueryParam::Query(None)) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::Course(None), QueryParam::NotSupported) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::Course(Some(_)), QueryParam::NotSupported) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::Course(Some(_)), QueryParam::Query(Some(_))) => return Err(error::Error::new(404,"Not Found".to_string())),

        //supported urls for get
        (PathParam::Course(Some(id)), QueryParam::Query(None)) => dao::get_course_by_id(id).await?,
        (PathParam::Course(None), QueryParam::Query(Some(category))) => dao::get_courses_by_category(category).await?,
    };

    Ok(ResponseWrapper::new(200,body))
}

async fn post(path:String, req_body: &Body) -> Result<ResponseWrapper, error::Error>{
    let res = match utils::parse_path(path, "".to_string()){
        //unsupported urls for post
        (PathParam::NotSupported, QueryParam::NotSupported) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::NotSupported, QueryParam::Query(None)) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::NotSupported, QueryParam::Query(Some(_))) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::Course(Some(_)), QueryParam::Query(None)) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::Course(Some(_)), QueryParam::NotSupported) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::Course(Some(_)), QueryParam::Query(Some(_))) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::Course(None), QueryParam::NotSupported) => return Err(error::Error::new(404,"Not Found".to_string())),
        (PathParam::Course(None), QueryParam::Query(Some(_))) => return Err(error::Error::new(404,"Not Found".to_string())),
        
        //supported urls for post
        (PathParam::Course(None), QueryParam::Query(None)) => dao::create_course(req_body).await?,
    };

    Ok(ResponseWrapper::new(201,res))
}
