resource "google_compute_network" "vpc" {
  name                    = "${var.vpc_name}-vpc-network"
  project                 = var.project_id
  auto_create_subnetworks = var.auto_create_subnetworks
}