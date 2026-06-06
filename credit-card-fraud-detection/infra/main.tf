data "aws_caller_identity" "current" {}

data "aws_region" "current" {}

module "s3" {
  source       = "./modules/s3"
  project_name = var.project_name
  bucket_name  = "${var.project_name}-${data.aws_caller_identity.current.account_id}"
  data_path    = "${path.root}/../data"
}

module "iam" {
  source       = "./modules/iam"
  project_name = var.project_name
}

module "sagemaker" {
  source             = "./modules/sagemaker"
  project_name       = var.project_name
  sagemaker_role_arn = module.iam.sagemaker_role_arn
}
