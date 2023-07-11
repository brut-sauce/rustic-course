import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as path from "path";
import { Code, Function, Runtime, FunctionUrlAuthType } from "aws-cdk-lib/aws-lambda";
import { CfnOutput } from "aws-cdk-lib";
import { AttributeType, BillingMode, Table } from "aws-cdk-lib/aws-dynamodb";

export class DeployStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    //DynamoDB table
    let table = Table.fromTableName(this, "Course", "Course");
    if (table==null){
      const new_table = new Table(this, "Course", {
        tableName: "Course",
        partitionKey: {
          name: "course_id",
          type: AttributeType.STRING,
        },
        billingMode: BillingMode.PAY_PER_REQUEST,
      });

      table = new_table;
    }
    

    const handler = new Function(this, "MyFunction", {
      code: Code.fromAsset(path.join(__dirname, "..", "..", "target/lambda/rustic-course")),
      runtime: Runtime.PROVIDED_AL2,
      handler: "anything",
      functionName: "rustic-course",
      environment: {
        TABLE_NAME: table.tableName,
      },
    });

    const fnUrl = handler.addFunctionUrl({
      authType: FunctionUrlAuthType.NONE,
    });

    new CfnOutput(this, 'TheUrl', {
      value: fnUrl.url,
    });

    table.grantReadWriteData(handler);
  }
}
