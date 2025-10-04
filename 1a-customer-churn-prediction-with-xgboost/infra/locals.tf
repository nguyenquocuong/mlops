locals {
  datasets_path = "../datasets"

  filenames = fileset(local.datasets_path, "**")

  bucket_name = "${random_string.prefix.id}-${var.app_name}"

  sagemaker = {
    jupyter_image_tag = "sagemaker-distribution-cpu"

    image_arn_prefixes = {
      us-east-1      = "arn:aws:sagemaker:us-east-1:081325390199:image"
      us-east-2      = "arn:aws:sagemaker:us-east-2:429704687514:image"
      us-west-1      = "arn:aws:sagemaker:us-west-1:742091327244:image"
      us-west-2      = "arn:aws:sagemaker:us-west-2:236514542706:image"
      af-south-1     = "arn:aws:sagemaker:af-south-1:559312083959:image"
      ap-east-1      = "arn:aws:sagemaker:ap-east-1:493642496378:image"
      ap-south-1     = "arn:aws:sagemaker:ap-south-1:394103062818:image"
      ap-northeast-2 = "arn:aws:sagemaker:ap-northeast-2:806072073708:image"
      ap-southeast-1 = "arn:aws:sagemaker:ap-southeast-1:492261229750:image"
      ap-southeast-2 = "arn:aws:sagemaker:ap-southeast-2:452832661640:image"
      ap-northeast-1 = "arn:aws:sagemaker:ap-northeast-1:102112518831:image"
      ca-central-1   = "arn:aws:sagemaker:ca-central-1:310906938811:image"
      eu-central-1   = "arn:aws:sagemaker:eu-central-1:936697816551:image"
      eu-west-1      = "arn:aws:sagemaker:eu-west-1:470317259841:image"
      eu-west-2      = "arn:aws:sagemaker:eu-west-2:712779665605:image"
      eu-west-3      = "arn:aws:sagemaker:eu-west-3:615547856133:image"
      eu-north-1     = "arn:aws:sagemaker:eu-north-1:243637512696:image"
      eu-south-1     = "arn:aws:sagemaker:eu-south-1:592751261982:image"
      sa-east-1      = "arn:aws:sagemaker:sa-east-1:782484402741:image"
      ap-northeast-3 = "arn:aws:sagemaker:ap-northeast-3:792733760839:image"
      ap-southeast-3 = "arn:aws:sagemaker:ap-southeast-3:276181064229:image"
      me-south-1     = "arn:aws:sagemaker:me-north-1:117516905037:image"
      me-central-1   = "arn:aws:sagemaker:me-central-1:103105715889:image"
    }

    distribution_image_arn_prefixes = {
      us-east-1      = "arn:aws:sagemaker:us-east-1:885854791233:image"
      us-east-2      = "arn:aws:sagemaker:us-east-2:137914896644:image"
      us-west-1      = "arn:aws:sagemaker:us-west-1:053634841547:image"
      us-west-2      = "arn:aws:sagemaker:us-west-2:542918446943:image"
      af-south-1     = "arn:aws:sagemaker:af-south-1:238384257742:image"
      ap-east-1      = "arn:aws:sagemaker:ap-east-1:523751269255:image"
      ap-south-1     = "arn:aws:sagemaker:ap-south-1:245090515133:image"
      ap-northeast-2 = "arn:aws:sagemaker:ap-northeast-2:064688005998:image"
      ap-southeast-1 = "arn:aws:sagemaker:ap-southeast-1:022667117163:image"
      ap-southeast-2 = "arn:aws:sagemaker:ap-southeast-2:648430277019:image"
      ap-northeast-1 = "arn:aws:sagemaker:ap-northeast-1:010972774902:image"
      ca-central-1   = "arn:aws:sagemaker:ca-central-1:481561238223:image"
      eu-central-1   = "arn:aws:sagemaker:eu-central-1:545423591354:image"
      eu-west-1      = "arn:aws:sagemaker:eu-west-1:819792524951:image"
      eu-west-2      = "arn:aws:sagemaker:eu-west-2:021081402939:image"
      eu-west-3      = "arn:aws:sagemaker:eu-west-3:856416204555:image"
      eu-north-1     = "arn:aws:sagemaker:eu-north-1:175620155138:image"
      eu-south-1     = "arn:aws:sagemaker:eu-south-1:810671768855:image"
      sa-east-1      = "arn:aws:sagemaker:sa-east-1:567556641782:image"
      ap-northeast-3 = "arn:aws:sagemaker:ap-northeast-3:564864627153:image"
      ap-southeast-3 = "arn:aws:sagemaker:ap-southeast-3:370607712162:image"
      me-south-1     = "arn:aws:sagemaker:me-north-1:523774347010:image"
      me-central-1   = "arn:aws:sagemaker:me-central-1:358593528301:image"
    }
  }

  sagemaker_image_arn_prefix = lookup(local.sagemaker.image_arn_prefixes, data.aws_region.current.region, "us-east-1")
  sagemaker_image_arn        = "${local.sagemaker_image_arn_prefix}/${local.sagemaker.jupyter_image_tag}"

  sagemaker_distribution_image_arn_prefix = lookup(local.sagemaker.distribution_image_arn_prefixes, data.aws_region.current.region, "us-east-1")
  sagemaker_distribution_image_arn        = "${local.sagemaker_distribution_image_arn_prefix}/${local.sagemaker.jupyter_image_tag}"
}

