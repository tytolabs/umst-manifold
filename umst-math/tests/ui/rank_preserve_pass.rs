use umst_math::density::DensityDiag;
use umst_math::tensor::partial_trace_second;

fn main() {
    let ab = DensityDiag::<6>::try_from_diag([1.0 / 6.0; 6]).expect("uniform");
    let traced: DensityDiag<2> = partial_trace_second::<2, 3, 6>(&ab).expect("trace");
    let _preserved: DensityDiag<2> = traced;
}
