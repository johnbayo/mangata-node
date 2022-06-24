provider "google" {
  project = var.project_id
  region  = var.region
}

data "google_client_config" "default" {
}

terraform {
  backend "gcs" {
    bucket = var.bucket_name
    prefix = "terraform/state/${var.environment}"
  }
}

data "google_project" "project" {
  project_id = var.project_id
}

module "my_vpc" {
  source                  = "../modules/gcp/vpc"
  project_id              = data.google_project.project_id
  vpc_name                = var.bucket_name
  auto_create_subnetworks = var.auto_create_subnetworks
}