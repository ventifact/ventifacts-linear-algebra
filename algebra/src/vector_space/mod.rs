mod blas;
pub use blas::*;

pub trait VectorSpace
{
    type Scalar;

    type Vector;
}

pub trait VAdd
{
    type Vector;

    fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector;
}

pub trait VAddMut
{
    type Vector;

    fn vadd_mut(&self, lhs: &mut Self::Vector, rhs: &Self::Vector);
}

pub trait VScale 
{
    type Scalar;

    type Vector;
    
    fn vscale(&self, vector: &Self::Vector, scalar: &Self::Scalar) -> Self::Vector;
}

pub trait VScaleMut 
{
    type Scalar;

    type Vector;

    fn vscale_mut(&self, vector: &mut Self::Vector, scalar: &Self::Scalar);    
}


pub trait VIdentity: VMultiplicativeIdentity + VAdditiveIdentity
{
    // Supertrait.
}  


pub trait VAdditiveIdentity
{
    type Output;

    fn additive_identity(&self) -> Self::Output;    
}


pub trait VMultiplicativeIdentity
{
    type Output;

    fn multiplicative_identity(&self) -> Self::Output;
}

pub trait VAdditiveInverse
{
    type Vector;

    fn additive_inv(&self, vector: &Self::Vector) -> Self::Vector;
}

pub trait VAdditiveInverseMut
{
    type Vector;

    fn additive_inv_mut(&self, vector: &mut Self::Vector);
}


pub trait VPartialEq
{
    type Vector;

    fn eq(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> bool;
}


#[macro_export]
macro_rules! vadd 
{
    ($vector_space:expr, $lhs:expr, $($rhs:expr),+) => {
        {
            $($vector_space.vadd_mut(&mut $lhs, $rhs);)+
            $lhs
        }
    };
}

#[macro_export]
macro_rules! vscale 
{
    ($vector_space:expr, $vec:expr, $scalar:expr) => {
        {
            $vector_space.vscale_mut(&mut $vec, $scalar);
            $vec
        }
    };
}
