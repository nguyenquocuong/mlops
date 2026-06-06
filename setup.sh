### Root account only

#!/bin/bash

REGION="ap-southeast-1"
ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"

TFSTATE_S3_BUCKET_NAME="tfstate-${ACCOUNT_ID}"
TFSTATE_DYNAMO_TABLE_NAME="tfstate-locks"

echo "Creating ${TFSTATE_S3_BUCKET_NAME} s3 bucket..."
aws s3 mb s3://${TFSTATE_S3_BUCKET_NAME}

echo "Creating ${TFSTATE_DYNAMO_TABLE_NAME} dynamo table..."
aws dynamodb create-table \
    --table-name ${TFSTATE_DYNAMO_TABLE_NAME} \
    --attribute-definitions AttributeName=LockID,AttributeType=S \
    --key-schema AttributeName=LockID,KeyType=HASH \
    --billing-mode PAY_PER_REQUEST \
    --region ${REGION}
