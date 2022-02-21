struct ProofOfWork;
struct ProofOfStack;

trait Proof {
    fn proof();
}

impl Proof for ProofOfWork {
    fn proof() {
        todo!()
    }
}

impl Proof for ProofOfStack {
    fn proof() {
        todo!()
    }
}