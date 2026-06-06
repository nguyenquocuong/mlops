data "aws_region" "current" {}

data "aws_vpc" "default" {
  default = true
}

data "aws_subnets" "default_vpc" {
  filter {
    name   = "vpc-id"
    values = [data.aws_vpc.default.id]
  }
}

resource "aws_sagemaker_domain" "this" {
  domain_name = var.project_name
  auth_mode   = "IAM"

  vpc_id     = data.aws_vpc.default.id
  subnet_ids = data.aws_subnets.default_vpc.ids

  default_space_settings {
    execution_role = var.sagemaker_role_arn
  }

  default_user_settings {
    execution_role = var.sagemaker_role_arn
  }
}

resource "aws_sagemaker_user_profile" "mlops" {
  domain_id         = aws_sagemaker_domain.this.id
  user_profile_name = "mlops"

  user_settings {
    execution_role = var.sagemaker_role_arn
  }
}

resource "aws_sagemaker_space" "this" {
  domain_id  = aws_sagemaker_domain.this.id
  space_name = var.project_name

  space_sharing_settings {
    sharing_type = "Shared"
  }

  ownership_settings {
    owner_user_profile_name = aws_sagemaker_user_profile.mlops.user_profile_name
  }

  space_settings {
    app_type = "JupyterLab"

    jupyter_lab_app_settings {
      default_resource_spec {
        instance_type       = "ml.t3.medium"
        sagemaker_image_arn = local.sagemaker_distribution_image_arn
      }
    }
  }
}
