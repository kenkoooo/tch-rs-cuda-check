fn main() {
    println!("{:?}", tch::Device::cuda_if_available());
    println!("{:?}", tch::Cuda::cudnn_is_available());
}
