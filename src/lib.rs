use std::marker::PhantomData;

pub struct SpringyBonesPlugin<C, S> {
    pub phantom_c: PhantomData<C>,
    pub phantom_s: PhantomData<S>,
}

