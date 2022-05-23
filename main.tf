terraform {
  required_version = "~> 1.2.0"
  backend "gcs" {
    bucket = "tfstate"
  }
}

## project ##
provider "google" {
  project = var.gcp_project_id
  region  = var.primary_region
}

module "artifact-registry" {
  source                     = "./modules/artifact-registry"
  gcp_project_id             = var.gcp_project_id
  artifact_registry_location = var.primary_region
}