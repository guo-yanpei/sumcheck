use ark_ff::Field;

struct Proof<F: Field> {
    fields: Vec<F>,
    commits: Commit<F>,
}

fn prove<F: Field>(mut f: Vec<F>, mut g: Vec<F>, r: F) -> Proof<F> {
    unimplemented!()
}

fn verify<F: Field>(com_f: Commit<F>, com_g: Commit<F>, r: F, claim: F, proof: Proof<F>) {
    unimplemented!()
}

struct Commit<F: Field>(Vec<F>);

impl<F: Field> Commit<F> {
    fn evaluation(&self, point: F) -> F {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use ark_bn254::Fr;
    use ark_ff::UniformRand;
    use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
    use ark_std::test_rng;

    use super::*;

    #[test]
    fn domains() {
        let size = 1 << 20;
        let mut rng = test_rng();
        let domain = Radix2EvaluationDomain::<Fr>::new(size).unwrap();
        let f_evals = (0..size).map(|_| Fr::rand(&mut rng)).collect::<Vec<_>>();
        let g_evals = (0..size).map(|_| Fr::rand(&mut rng)).collect::<Vec<_>>();
        let f_coeff = domain.ifft(&f_evals);
        let g_coeff = domain.ifft(&g_evals);

        let ext_domain = Radix2EvaluationDomain::<Fr>::new(size << 1).unwrap();
        let f_ext_evals = ext_domain.fft(&f_coeff);
        let g_ext_evals = ext_domain.fft(&g_coeff);
    }

    #[test]
    fn it_works() {}
}
