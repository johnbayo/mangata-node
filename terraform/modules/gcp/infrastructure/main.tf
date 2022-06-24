resource "google_service_account" "default" {
  account_id   = var.account_id
  display_name = var.name
}

resource "google_compute_instance" "default" {
  name         = var.name
  machine_type = var.instance_type
  zone         = var.zone

  tags = [var.name]

  boot_disk {
    initialize_params {
      image = var.image_type
    }
  }

  // Local SSD disk
  scratch_disk {
    interface = var.disk_type
  }

  network_interface {
    network = "${var.vpc_name}-vpc-network"

    access_config {
      // Ephemeral public IP
    }
  }

  #  metadata_startup_script = "echo hi > /test.txt"

  service_account {
    # Google recommends custom service accounts that have cloud-platform scope and permissions granted via IAM Roles.
    email  = google_service_account.default.email
    scopes = ["cloud-platform"]
  }
}