use crate::model::{PathParam, QueryParam};

pub fn parse_path(path:String, query_params:String) -> (PathParam, QueryParam) {
    let path_parts = path.split("/").skip(1).collect::<Vec<&str>>();
    let url=path_parts.get(0).unwrap_or(&"");

    let binding = url.to_string();
    let path_components = binding.split("?").collect::<Vec<&str>>();
    let path_key=path_components.get(0).unwrap_or(&"");

    match *path_key {
        "courses" => {
            if let Some(path_id) = path_parts.get(1) {
                print!("here1");
                (PathParam::Course(Some(path_id.to_string())), QueryParam::Query(None))
            } else{
                if query_params.is_empty(){
                    (PathParam::Course(None), QueryParam::Query(None))
                } else{
                    let params = query_params.split('&').collect::<Vec<&str>>();
                    if params.len() > 1 {
                        return (PathParam::Course(None), QueryParam::NotSupported);
                    }

                    let parts = params[0].split('=').collect::<Vec<&str>>();
                    if parts.len() == 2 && parts[0] == "category" {
                        return (PathParam::Course(None), QueryParam::Query(Some(parts[1].to_string())));
                    } else{
                        return (PathParam::Course(None), QueryParam::NotSupported);
                    }
                }
            }
        }
        _ => {
            (PathParam::NotSupported, QueryParam::NotSupported)
        }
    }
}