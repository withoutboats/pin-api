use mem::Anchor;

pub enum CoResult<Y, R> {
    Yield(Y),
    Return(R),
}

pub trait StaticGenerator {
    type Yield;
    type Return;

    fn static_resume(this: Anchor<&mut Self>) -> CoResult<Self::Yield, Self::Return>;
}

pub trait Generator: StaticGenerator {
    fn resume(&mut self) -> CoResult<Self::Yield, Self::Return>;
}
