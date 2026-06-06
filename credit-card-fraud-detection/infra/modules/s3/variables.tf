variable "project_name" {
  description = "Used as a prefix for all resource names"
  type        = string
}

variable "bucket_name" {
  description = "Project bucket name"
  type        = string
}

variable "data_path" {
  description = "Local path to the data directory to upload"
  type        = string
}
