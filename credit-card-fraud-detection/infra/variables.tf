variable "region" {
  description = "AWS region"
  type        = string
  default     = "ap-southeast-1"
}

variable "project_name" {
  description = "Used as a prefix for all resource names"
  type        = string
  default     = "credit-card-fraud-detection"
}
