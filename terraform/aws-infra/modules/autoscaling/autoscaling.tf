resource "aws_launch_configuration" "eks_worker_launch_config" {
  name          = "eks-worker-launch-config"
  image_id      = "ami-053a45fff0a704a47"  # Use an appropriate EKS worker node AMI ID
  instance_type = "t2.medium"
  security_groups = ["sg-xxxxxxxx"]  # Security group ID for EKS worker nodes
#  security_groups = [var.eks_worker_sg_id]
  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_autoscaling_group" "eks_worker_asg" {
  desired_capacity     = 2
  max_size             = 3
  min_size             = 1
  vpc_zone_identifier  = var.subnet_ids
  launch_configuration = aws_launch_configuration.eks_worker_launch_config.id
}
#variable "eks_worker_sg_id" {
#  description = "The security group ID for the EKS worker nodes"
#  type        = string
#}
