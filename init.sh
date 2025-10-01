### Root account only

#!/bin/bash

ACCOUNT_ID="$(aws sts get-caller-identity --query Account --output text)"

IAM_USER="terraform"

IAM_ROLE="terraform"
IAM_POLICY="terraform"

S3_BUCKET_NAME="mla-c01-tf"

aws s3 mb s3://${S3_BUCKET_NAME}

aws iam create-user --user-name ${IAM_USER}
IAM_USER_ARN="$(aws iam get-user --user-name ${IAM_USER} --query User.Arn --output text)"

aws iam create-role --role-name ${IAM_ROLE} --assume-role-policy-document "$(cat <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "AWS": "${IAM_USER_ARN}"
      },
      "Action": "sts:AssumeRole"
    }
  ]
}
EOF
)"

aws iam create-policy --policy-name ${IAM_POLICY} --policy-document "$(cat <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "iam:CreateRole"
      ],
      "Resource": "*"
    },
    {
      "Effect": "Allow",
      "Action": [
        "s3:ListBucket"
      ],
      "Resource": "arn:aws:s3:::${S3_BUCKET_NAME}"
    },
    {
      "Effect": "Allow",
      "Action": [
        "s3:GetObject",
        "s3:PutObject"
      ],
      "Resource": "arn:aws:s3:::${S3_BUCKET_NAME}/*"
    }
  ]
}
EOF
)"

aws iam attach-role-policy --role-name ${IAM_ROLE} --policy-arn arn:aws:iam::${ACCOUNT_ID}:policy/${IAM_POLICY}
