resource "aws_iam_role" "sagemaker_execution_role" {
  name = "sm-notebook-instance-role"

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

resource "aws_sagemaker_notebook_instance" "customer_churn" {
  name     = "customer-churn"
  role_arn = aws_iam_role.sagemaker_execution_role.arn

  instance_type = "ml.t2.medium"
}
