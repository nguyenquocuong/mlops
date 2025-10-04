output "dataset_filesets" {
  value = local.dataset_filesets
}

output "notebook_filesets" {
  value = local.notebook_filesets
}

resource "aws_s3_bucket" "this" {
  bucket = local.bucket_name
  region = data.aws_region.current.region
}

resource "aws_s3_object" "datasets" {
  for_each = local.dataset_filesets

  bucket = local.bucket_name
  key    = "data/raw/${each.value}"

  source = "${local.dataset_path}/${each.value}"

  etag = filemd5("${local.dataset_path}/${each.value}")

  depends_on = [aws_s3_bucket.this]
}

resource "aws_s3_object" "notebooks" {
  for_each = local.notebook_filesets

  bucket = local.bucket_name
  key    = "notebooks/${each.value}"

  source = "${local.notebook_path}/${each.value}"

  etag = filemd5("${local.notebook_path}/${each.value}")

  depends_on = [aws_s3_bucket.this]
}
