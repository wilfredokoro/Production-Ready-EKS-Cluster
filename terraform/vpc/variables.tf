variable "region" {
  type        = string
  description = "Default region"
  default     = "us-east-1"
}

variable "cidr" {
  description = "The CIDR block for the VPC"
  type        = string
  default     = ""
}

variable "availability_zones" {
  description = "List of AZ in the region"
  type        = list(string)
  default     = ["us-east-1a", "us-east-1b", "us-east-1c"]

}

variable "vpc-name" {
  type        = string
  description = "VPC name"
  default     = ""
}
