resource "aws_default_vpc" "default" {}

data "aws_subnets" "default_vpc_subnets" {
  filter {
    name   = "vpc-id"
    values = [aws_default_vpc.default.id]
  }
}

resource "aws_iam_role" "sagemaker_execution_role" {
  name = "sagemaker-domain-execution-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "sagemaker.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "sagemaker_managed_attach" {
  for_each   = toset(["AmazonSageMakerFullAccess"])
  role       = aws_iam_role.sagemaker_execution_role.name
  policy_arn = "arn:aws:iam::aws:policy/${each.value}"
}

resource "aws_sagemaker_domain" "this" {
  domain_name = "customer-churn"
  auth_mode   = "IAM"

  vpc_id     = aws_default_vpc.default.id
  subnet_ids = data.aws_subnets.default_vpc_subnets.ids

  default_user_settings {
    execution_role = aws_iam_role.sagemaker_execution_role.arn
  }
}
