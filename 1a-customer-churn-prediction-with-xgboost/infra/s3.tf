output "datasets_filenames" {
  value = local.filenames
}

resource "aws_s3_bucket" "this" {
  bucket = local.bucket_name
  region = var.region
}

resource "aws_s3_object" "data_raw" {
  for_each = local.filenames

  bucket = local.bucket_name
  key    = "/data/raw/${each.value}"

  source = "${local.datasets_path}/${each.value}"

  etag = filemd5("${local.datasets_path}/${each.value}")

  depends_on = [aws_s3_bucket.this]
}
