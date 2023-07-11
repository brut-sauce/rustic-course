use crate::error::Error;
use crate::model::Course;
extern crate serde_json;
use aws_sdk_dynamodb::{Client, model::AttributeValue};
use lambda_http::Body;
use serde_json::{Value, json};

// set up DynamoDB client
async fn get_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

// Create a new course in DynamoDB
pub async fn create_course(req_body: &Body) -> Result<Value,Error>{
    let request_json = match req_body {
        Body::Text(json_string) => json_string,
        _ => "",
    };

    if request_json.is_empty() {
        return Err(Error::new(400, "Bad Request".to_string()));
    };

    let request_struct: Course = serde_json::from_str(request_json)?;

    let course_id_av = AttributeValue::S(request_struct.course_id.clone());
    let course_name_av = AttributeValue::S(request_struct.course_name.to_string());
    let course_category_av = AttributeValue::S(request_struct.course_category.to_string());

    let client = get_client().await;
    let query = client.put_item().table_name("Course")
    .item("course_id", course_id_av)
    .item("course_name", course_name_av)
    .item("course_category", course_category_av);

    let put_item_output = query.send().await;

    match put_item_output {
        Ok(put_item_output) => {
            let res = "Course Successfully Created!";
            Ok(res.into())
        }
        Err(err) => {
            Err(Error::new(500, format!("DB Error. Error = {}", err)))
        }
    }


}

// Get a course by ID from DynamoDB
pub async fn get_course_by_id(id:String) -> Result<Value, Error>{
    let client = get_client().await;
    let query = client.get_item()
    .table_name("Course")
    .key("course_id", AttributeValue::S(id));

    let get_item_output = query.send().await;

    match get_item_output {
        Ok(get_item_output) => {
            if let Some(item) = get_item_output.item{
                let c_id = item["course_id"].as_s().unwrap().to_string();
                let c_name = item["course_name"].as_s().unwrap().to_string();
                let c_category = item["course_category"].as_s().unwrap().to_string();

                let res = json!({
                    "course_id": c_id,
                    "course_name": c_name,
                    "course_category": c_category
                });

                Ok(res)

            } else{
                Err(Error::new(404, "Course Not Found!".to_string()))
            }
        }
        Err(err) => {
            Err(Error::new(500, format!("DB Error. Error = {}", err)))
        }
    }
}

// Get courses by category from DynamoDB
pub async fn get_courses_by_category(category: String) -> Result<Value, Error>{
    let client = get_client().await;

    let query = client.scan()
    .table_name("Course")
    .filter_expression("course_category = :category")
    .expression_attribute_values(":category", AttributeValue::S(category));

    let courses = query.send().await;
    match courses {
        Ok(courses) => {
            if let Some(items) = courses.items(){
                let mut res: Vec<Value> = Vec::new();
                for item in items.iter() {
                    let c_id = item["course_id"].as_s().unwrap().to_string();
                    let c_name = item["course_name"].as_s().unwrap().to_string();
                    let c_category = item["course_category"].as_s().unwrap().to_string();

                    let item_res = json!({
                        "course_id": c_id,
                        "course_name": c_name,
                        "course_category": c_category
                    });

                    res.push(item_res);
                }
                Ok(Value::Array(res))

            }else{
                Err(Error::new(404, "No Course found for given category!".to_string()))
            }
        }
        Err(err)  => {
            Err(Error::new(500, format!("DB Error. Error = {}", err)))
        }
    }
}