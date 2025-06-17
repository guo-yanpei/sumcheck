use ark_bn254::Fr;
use ark_ff::{AdditiveGroup, Field, UniformRand};

fn prove<F: Field>(mut f: Vec<F>, mut g: Vec<F>, r: &Vec<F>) -> Vec<F> {
    let l = r.len();
    let mut messages = vec![];
    for i in 0..l {
        // todo!("implement prover's algo. Do not use r[i + 1] before generating messages[i]");
        let mut prover_msg = [F::ZERO; 3];

        for i in 0..3 {
            messages.push(prover_msg[i]);
        }
    }
    return messages;
}

fn verifer<F: Field>(com_f: Commit<F>, com_g: Commit<F>, r: &Vec<F>, claim: F, messages: Vec<F>) {
    // todo!("implement verifier's algo");
    let l = r.len();

    let mut current_claim = claim;
    for i in 0..l {}

    // Verifier now needs to assert that f(r) * g(r) = final claim
    assert_eq!(
        com_f.evaluation(r.to_vec()) * com_g.evaluation(r.to_vec()),
        current_claim
    );
}

struct Commit<F: Field>(Vec<F>);

impl<F: Field> Commit<F> {
    fn evaluation(&self, point: Vec<F>) -> F {
        let l = point.len();
        let mut res = F::ZERO;
        res
    }
}

fn main() {
    // implement sumcheck for \sum f[i]*g[i] = y.
    let mut rng = ark_std::test_rng();
    let l = 20;
    let f = (0..(1 << l))
        .map(|_| Fr::rand(&mut rng))
        .collect::<Vec<_>>();
    let g = (0..(1 << l))
        .map(|_| Fr::rand(&mut rng))
        .collect::<Vec<_>>();
    // f and g are MLE evaluations over hypercube

    let y = f
        .iter()
        .zip(g.iter())
        .fold(Fr::ZERO, |sum, (&x, &y)| sum + x * y);
    let r = (0..l).map(|_| Fr::rand(&mut rng)).collect::<Vec<_>>();

    let m = prove(f.clone(), g.clone(), &r);
    verifer(Commit(f), Commit(g), &r, y, m);
}
