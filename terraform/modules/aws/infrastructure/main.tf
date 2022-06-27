# Create aws_ami filter to pick up the ami available in your region
###############################
# AMI Image
###############################
data "aws_ami" "ubuntu" {
  most_recent = true

  filter {
    name   = "name"
    values = var.image_ami
  }

  filter {
    name   = "virtualization-type"
    values = ["hvm"]
  }

  owners = ["099720109477"] # Canonical
}

# SSH definition
resource "tls_private_key" "key" {
  algorithm = "RSA"
}

################################
# SSH KEY GENERATION
################################
resource "local_file" "private_key" {
  filename          = "${var.key_path}/${var.key_name}.pem"
  sensitive_content = tls_private_key.key.private_key_pem
  file_permission   = "0775"
}

resource "aws_key_pair" "key_pair" {
  key_name   = var.key_name
  public_key = tls_private_key.key.public_key_openssh
}

################################
# VPC
################################
# Configure the EC2 instance in a public subnet
resource "aws_vpc" "my_vpc" {
  cidr_block           = var.cidr
  enable_dns_hostnames = true

  tags = {
    Name = var.interview_tag
  }
}

###############################
# SUBNET
###############################
resource "aws_subnet" "my_subnet" {
  vpc_id            = aws_vpc.my_vpc.id
  cidr_block        = var.subnet_cidr
  availability_zone = var.zone

  tags = {
    Name = var.interview_tag
  }
}

###################################
# SECURITY GROUP
###################################
// SG to allow SSH connections from anywhere
resource "aws_security_group" "my_sg" {
  name        = var.sg_name
  description = "Allow SSH inbound traffic"
  vpc_id      = aws_vpc.my_vpc.id

  ingress {
    description = "SSH access"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 65535
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  tags = {
    Name = var.interview_tag
  }
}

###################################
# NIC
###################################
resource "aws_network_interface" "my_interface" {
  subnet_id       = aws_subnet.my_subnet.id
  security_groups = [aws_security_group.my_sg.id]
  private_ips     = var.private_ipaddress

  tags = {
    Name = var.interview_tag
  }
}

###################################
# ROUTE TABLE
###################################
resource "aws_route_table" "public" {
  vpc_id = aws_vpc.my_vpc.id
  tags = {
    tag-key = var.interview_tag
  }
}

###################################
# INTERNET GATEWAY
###################################
resource "aws_internet_gateway" "gw" {
  vpc_id = aws_vpc.my_vpc.id
}

###################################
# ROUTE
##################################
resource "aws_route" "public_internet_gateway" {
  route_table_id         = aws_route_table.public.id
  destination_cidr_block = "0.0.0.0/0"
  gateway_id             = aws_internet_gateway.gw.id
}

###################################
# ROUTE Association
##################################
resource "aws_route_table_association" "public" {
  subnet_id = aws_subnet.my_subnet.id

  route_table_id = aws_route_table.public.id
}

###################################
# EIP
##################################
resource "aws_eip" "my_eip" {
  vpc                       = true
  network_interface         = aws_network_interface.my_interface.id
  associate_with_private_ip = aws_network_interface.my_interface.private_ip
  depends_on                = [aws_internet_gateway.gw]
}

################################
# IAM ROLE
################################
resource "aws_iam_role" "interview_role" {
  name = var.interview_tag

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "ec2.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF

  tags = {
    tag-key = var.interview_tag
  }
}

resource "aws_iam_instance_profile" "interview_profile" {
  name = var.interview_tag
  role = aws_iam_role.interview_role.name
}

##################################
# VM INSTANCE
##################################
resource "aws_instance" "my_vm" {
  ami                  = data.aws_ami.ubuntu.id
  instance_type        = var.instance_type
  key_name             = var.key_name
  iam_instance_profile = aws_iam_instance_profile.interview_profile.name
  user_data            = "${path.module}/init.sh"

  network_interface {
    network_interface_id = aws_network_interface.my_interface.id
    device_index         = 0
  }

  credit_specification {
    cpu_credits = "unlimited"
  }

  tags = {
    Name = var.interview_tag
  }
}

##################################
# Data template file
##################################
data "template_file" "inventory" {
  template = file("inventory.tmpl")
  vars = {
    interview_host = aws_instance.my_vm.public_dns
  }
}

resource "local_file" "inventory" {
  filename          = "${var.inventory_path}/inventory.yml"
  sensitive_content = data.template_file.inventory.rendered
}