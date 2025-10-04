data "aws_caller_identity" "current" {}

data "aws_region" "current" {}

resource "random_string" "prefix" {
  length  = 8
  upper   = false
  special = false
}
