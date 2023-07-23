use super::Pipeline;

pub trait Pipe<I> {
    type In;
    type Out;
    fn pipe(self, to: I) -> Pipeline<Self::In, Self::Out>;
}
