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
  domain_name = var.app_name
  auth_mode   = "IAM"

  vpc_id     = aws_default_vpc.default.id
  subnet_ids = data.aws_subnets.default_vpc_subnets.ids

  default_user_settings {
    execution_role = aws_iam_role.sagemaker_execution_role.arn
  }
}

resource "aws_sagemaker_user_profile" "this" {
  domain_id         = aws_sagemaker_domain.this.id
  user_profile_name = "machine-learning-engineer"

  user_settings {
    execution_role = aws_iam_role.sagemaker_execution_role.arn
  }
}

resource "aws_sagemaker_space" "this" {
  domain_id  = aws_sagemaker_domain.this.id
  space_name = var.app_name

  space_sharing_settings {
    sharing_type = "Private"
  }

  ownership_settings {
    owner_user_profile_name = aws_sagemaker_user_profile.this.user_profile_name
  }

  space_settings {
    app_type = "JupyterLab"

    jupyter_lab_app_settings {
      default_resource_spec {
        instance_type       = "ml.t3.medium"
        sagemaker_image_arn = local.sagemaker_image_arn
      }
    }
  }
}

resource "aws_sagemaker_app" "this" {
  domain_id = aws_sagemaker_domain.this.id

  space_name = aws_sagemaker_space.this.space_name

  app_type = "JupyterLab"
  app_name = "default"

  resource_spec {
    instance_type       = "ml.t3.medium"
    sagemaker_image_arn = local.sagemaker_image_arn
  }
}
