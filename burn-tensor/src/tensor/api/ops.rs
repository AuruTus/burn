use super::Tensor;
use crate::tensor::backend::Backend;
use crate::tensor::Element;

impl<const D: usize, B> std::ops::Add<Self> for Tensor<B, D>
where
    B: Backend,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tensor::add(&self, &other)
    }
}

impl<E, const D: usize, B> std::ops::Add<E> for Tensor<B, D>
where
    E: Element,
    B: Backend<Elem = E>,
{
    type Output = Self;

    fn add(self, other: E) -> Self {
        Tensor::add_scalar(&self, &other)
    }
}

impl<const D: usize, B> std::ops::Sub<Self> for Tensor<B, D>
where
    B: Backend,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tensor::sub(&self, &other)
    }
}

impl<E, const D: usize, B> std::ops::Sub<E> for Tensor<B, D>
where
    E: Element,
    B: Backend<Elem = E>,
{
    type Output = Self;

    fn sub(self, other: E) -> Self {
        Tensor::sub_scalar(&self, &other)
    }
}

impl<const D: usize, B> std::ops::Mul<Self> for Tensor<B, D>
where
    B: Backend,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Tensor::mul(&self, &other)
    }
}

impl<E, const D: usize, B> std::ops::Mul<E> for Tensor<B, D>
where
    E: Element,
    B: Backend<Elem = E>,
{
    type Output = Self;

    fn mul(self, other: E) -> Self {
        Tensor::mul_scalar(&self, &other)
    }
}

impl<const D: usize, B> std::ops::Div<Self> for Tensor<B, D>
where
    B: Backend,
{
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Tensor::div(&self, &other)
    }
}

impl<E, const D: usize, B> std::ops::Div<E> for Tensor<B, D>
where
    E: Element,
    B: Backend<Elem = E>,
{
    type Output = Self;

    fn div(self, other: E) -> Self {
        Tensor::div_scalar(&self, &other)
    }
}
