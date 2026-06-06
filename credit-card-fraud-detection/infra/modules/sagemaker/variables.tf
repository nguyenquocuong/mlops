variable "project_name" {
  description = "Used as a prefix for all resource names"
  type        = string
}

variable "sagemaker_role_arn" {
  description = "IAM role ARN for SageMaker execution"
  type        = string
}
