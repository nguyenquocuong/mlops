output "default_vpc_subnet_ids" {
  value = data.aws_subnets.default_vpc_subnets.ids
}
