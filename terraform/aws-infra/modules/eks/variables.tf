variable "subnet_ids" {
  description = "The list of subnet IDs to launch the EKS cluster and worker nodes in"
  type        = list(string)
}

variable "cluster_name" {
  description = "The name of the EKS cluster"
  type        = string
}
