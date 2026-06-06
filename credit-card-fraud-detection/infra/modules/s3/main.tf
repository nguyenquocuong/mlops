resource "aws_s3_bucket" "this" {
  bucket = var.bucket_name
}

resource "aws_s3_bucket_versioning" "this" {
  bucket = aws_s3_bucket.this.bucket

  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_object" "data" {
  for_each = local.data_filesets

  bucket = var.bucket_name
  key    = "data/raw/${each.value}"
  source = "${var.data_path}/${each.value}"
  etag   = filemd5("${var.data_path}/${each.value}")

  depends_on = [aws_s3_bucket.this]
}
