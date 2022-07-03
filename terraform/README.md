# terraform
This terraform module will deploy a virtual machine, ssh keys, elastic ip, internet gateway, vpc and subnets <br/>
<br/>
Requirements<br/>
access key<br/>
secret key<br/>
<br/>
module Values
<br/>
Available values are listed below, along with variables <br/>
values.auto.tfvars<br/>
<br/>
<pre>
interview_tag     = "second_interview"
image_ami         = ["ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-*"]
cidr              = "172.16.0.0/16"
subnet_cidr       = "172.16.0.0/24"
zone              = "eu-central-1a"
instance_type     = "c5.xlarge"
pub_ip_assoc      = true
key_name          = "sshkey"
key_path          = "../ansible/files"
sg_name           = "infrasg"
private_ipaddress = ["172.16.0.100"]
inventory_path    = "../ansible/inventories"
</pre>
<br/>
The docker_edition should be either ce (Community Edition) or ee (Enterprise Edition). <br/>
<br/>
</pre>
<br/>
Terraform installation options.<br/>
hashicorp/setup-terraform@v2<br/><br/>
<br/>
deployment of the module<br/>
<pre>
terraform apply --auto-aprove -lock=false
</pre>
<br/>
License<br/>
<br/>
MIT / BSD
