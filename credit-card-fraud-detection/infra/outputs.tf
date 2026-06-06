output "s3_bucket_name" {
  value = module.s3.bucket_name
}

output "s3_bucket_arn" {
  value = module.s3.bucket_arn
}

output "sagemaker_role_arn" {
  value = module.iam.sagemaker_role_arn
}
