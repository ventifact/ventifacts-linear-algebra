pub trait VectorSpace
{
    type Scalar;

    type Vector;

    fn dimensions(&self) -> usize;
}

pub trait VAdd
{
    type Vector;

    fn vadd_mut(&self, lhs: &mut Self::Vector, rhs: &Self::Vector);

    fn vadd(&self, lhs: &Self::Vector, rhs: &Self::Vector) -> Self::Vector;
}

pub trait VScale 
{
    type Scalar;

    type Vector;

    fn vscale(&self, vector: &mut Self::Vector, scalar: &Self::Scalar);
}


pub trait VIdentity: VMultiplicativeIdent + VAdditiveIdent
{
    // Supertrait.
}  


pub trait VAdditiveIdent
{
    type Output;

    fn additive_ident(&self) -> Self::Output;    
}


pub trait VMultiplicativeIdent
{
    type Output;

    fn mul_ident(&self) -> Self::Output;
}

pub trait VAdditiveInverse
{
    type Vector;

    fn additive_inv(&self, vector: &mut Self::Vector);
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
            $vector_space.vscale(&mut $vec, $scalar);
            $vec
        }
    };
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests 
{
    use super::*;
    
    use crate::ndarray;

    macro_rules! test {
        ($name:ident, $object:ty, $space:ty) => {
            mod $name {
                use super::*;

                #[test]
                fn test_addition() 
                {
                    let vector_space = <$space>::new();
                    let mut x = <$object>::from([ 3, 0, -1 ]);
                    let y = <$object>::from([ 10, 1, 2 ]);

                    let exp: $object = <$object>::from([ 13, 1, 1 ]);
                    let test: $object = vadd!(vector_space, x, &y);

                    assert!( vector_space.eq(&exp, &test) );
                }

                #[test]
                fn test_multiplication()
                {
                    let vector_space = <$space>::new();
                    let mut x = <$object>::from([ 3, 0, -1 ]);
                    let c = 2;

                    let exp = <$object>::from([ 6, 0, -2 ]);
                    let test = vscale!(vector_space, x, &c);
                    assert!( vector_space.eq(&exp, &test) );
                }

                #[test]
                fn test_commutative()
                {
                    let vector_space = <$space>::new();
                    let mut x1 = <$object>::from([ 3, 1, 5 ]);
                    let x2: $object = x1.clone();
                    let mut y = <$object>::from([ 6, 2, 7 ]);

                    let lhs = vadd!(vector_space, x1, &y);
                    let rhs = vadd!(vector_space, y, &x2);
                    assert!( vector_space.eq(&lhs, &rhs) );
                }

                #[test]
                fn test_associative_addition()
                {
                    let vector_space = <$space>::new();
                    let mut x1: $object = <$object>::from([ 3, 1, 5 ]);
                    let x2: $object = x1.clone();
                    let mut y: $object = <$object>::from([ 6, 2, 7 ]);
                    let z = <$object>::from([ 4, 5, 1 ]);

                    let lhs: $object = vadd!(vector_space, x1, &y, &z);
                    let rhs: $object = vadd!(vector_space, y, &z, &x2);
                    assert!( vector_space.eq(&lhs, &rhs) );
                }

                #[test]
                fn test_additive_ident()
                {
                    let vector_space = <$space>::new();
                    let exp: $object = <$object>::from([ 0, 0, 0 ]);

                    let test: $object = vector_space.additive_ident();
                    assert!( vector_space.eq(&exp, &test) );
                }

                #[test]
                fn test_additive_inverse()
                {
                    let vector_space = <$space>::new();
                    let mut x: $object = <$object>::from([ 3, 1, 5 ]);
                    let exp: $object = <$object>::from([ -3, -1, -5 ]);
                    
                    vector_space.additive_inv(&mut x);
                    assert!( vector_space.eq(&exp, &x) );
                }

                #[test]
                fn test_vadd() 
                {
                    let vector_space = <$space>::new();
                    
                    let mut x: $object = <$object>::from([ 3, 1, 5 ]);
                    let y: $object = <$object>::from([ 6, 2, 7 ]);
                    let z: $object = <$object>::from([ 4, 5, 1 ]);
                    let test: $object = vadd!(vector_space, x, &y, &z);

                    let exp: $object = <$object>::from([ 13, 8, 13]);
                    assert!( vector_space.eq(&test, &exp) );
                }   
            }
        };
    }

   
    mod test_array
    {
        use super::*;

        ndarray! {
            @vector_space(VectorSpaceArray) {
                @vector_ident(VectorArray)
                @length(3)
                @generic(T)
                @array([T; 3])
            }
        }

        test!(test_ndarray, VectorArray<isize>, VectorSpaceArray<isize>);
    
        impl VAdditiveIdent for VectorSpaceArray<isize>
        {
            type Output = VectorArray<isize>;
    
            fn additive_ident(&self) -> Self::Output
            {
                VectorArray::new( [0; 3] )
            }
        }
    
        impl VMultiplicativeIdent for VectorSpaceArray<isize>
        {
            type Output = isize;
    
            fn mul_ident(&self) -> Self::Output
            {
                1
            }
        }
    }


    mod test_vec 
    {
        use super::*;

        ndarray! {
            @vector_space(VectorSpaceVec) {
                @vector_ident(VectorVec)
                @length(3)
                @generic(T)
                @vec(Vec<T>)
            }
        }

        test!(test_ndvec, VectorVec<isize>, VectorSpaceVec<isize>);

        impl From<[isize; 3]> for VectorVec<isize>
        {
            fn from(array: [isize; 3]) -> Self {
                Self::from( array.to_vec() )
            }
        }

        impl VAdditiveIdent for VectorSpaceVec<isize>
        {
            type Output = VectorVec<isize>;
    
            fn additive_ident(&self) -> Self::Output
            {
                VectorVec::new( vec![0; 3] )
            }
        }
    
        impl VMultiplicativeIdent for VectorSpaceVec<isize>
        {
            type Output = isize;
    
            fn mul_ident(&self) -> Self::Output
            {
                1
            }
        }
    }

}
