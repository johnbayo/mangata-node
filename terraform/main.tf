module "VM" {
  source            = "./modules/aws/infrastructure"
  interview_tag     = var.interview_tag
  image_ami         = var.image_ami
  cidr              = var.cidr
  subnet_cidr       = var.subnet_cidr
  zone              = var.zone
  instance_type     = var.instance_type
  pub_ip_assoc      = var.pub_ip_assoc
  sg_name           = var.sg_name
  key_name          = var.key_name
  key_path          = var.key_path
  private_ipaddress = var.private_ipaddress
  inventory_path    = var.inventory_path
}
