# rustic-course
A rust application using AWS Lambda, DynamoDB and AWS CDK

### problem-statement
Your task is to design and build a scalable serverless RESTful API that performs Create and Read operations for a "Course" entity. 
Each Course could have attributes 
{  course_id
  course_name, 
  course_category
}

1. Create a serverless API using AWS Lambda, written in Rust. The API should have 3 methods. 
Create Course
getCourseById
getCoursesByCategory
2. Use Amazon DynamoDB, serverless as your database.
3. Infrastructure as Code: Use AWS CDK

### Result
The application is deployed using AWS CDK as a lambda function and with DynamoDB used as a serverless datasource
the endpoint for invoking the lambda function: https://m5wojhikstftzqo3o4pnxc3xny0qtlhb.lambda-url.ap-south-1.on.aws/

#### Sample curls

##### Method: GET
##### API: /courses?category={course_category}
```
curl --location 'https://m5wojhikstftzqo3o4pnxc3xny0qtlhb.lambda-url.ap-south-1.on.aws/courses?category=11'
```

##### Method: GET
##### API: /courses/{course_id}
```
curl --location 'https://m5wojhikstftzqo3o4pnxc3xny0qtlhb.lambda-url.ap-south-1.on.aws/courses/2'
```

##### Method: POST
##### API: /courses
```
curl --location 'https://m5wojhikstftzqo3o4pnxc3xny0qtlhb.lambda-url.ap-south-1.on.aws/courses' \
--header 'Content-Type: application/json' \
--data '{
    "course_id": "44",
    "course_name": "c2",
    "course_category": "11"
}'
```

### Improvements for iteration 2
- Separate service and dao layers
- "course_id" is currently required to be sent in the POST API call that creates a course, needs to be autogenerated
- need to create global_secondary_index on "course_category"
- add authentication
- overall code quality (first time using rust :p)

