interview_tag     = "second_interview"
image_ami         = ["ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-*"]
cidr              = "172.16.0.0/16"
subnet_cidr       = "172.16.0.0/24"
zone              = "eu-central-1a"
instance_type     = "t2.micro"c5.xlarge"
pub_ip_assoc      = true
key_name          = "sshkey"
key_path          = "../ansible/files"
sg_name           = "infrasg"
private_ipaddress = ["172.16.0.100"]
inventory_path    = "../ansible/inventories"