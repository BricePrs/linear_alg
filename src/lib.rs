
use rand::Rng;
use std::{convert::Into, ops};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/* 
//------------------------------//
// Vector Comparaison functions //
//------------------------------//

impl cmp::PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}
 */

//-----------------------------//
// Vector Operations functions //
//-----------------------------//


// vector-vector operator

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.x+rhs.x, 
            self.y+rhs.y, 
            self.z+rhs.z,
                )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.x-rhs.x, 
            self.y-rhs.y, 
            self.z-rhs.z,
                )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.x*rhs.x, 
            self.y*rhs.y, 
            self.z*rhs.z,
                )
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Vec3 {
        assert_ne!(rhs.x*rhs.y*rhs.z, 0., "Error: Division by 0");
        Vec3::new(
            self.x/rhs.x, 
            self.y/rhs.y, 
            self.z/rhs.z,
                )
    }
}

// vector-Into(float) operator

impl<T: Into<f64>> ops::Mul<T> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: T) -> Vec3 {
        let val = rhs.into();
        Vec3::new(
            self.x*val, 
            self.y*val, 
            self.z*val,
                )
    }
}


impl<T: Into<f64>> ops::Div<T> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: T) -> Vec3 {
        let val = rhs.into();
        assert_ne!(val, 0., "Error : Division by 0");
        Vec3::new(
            self.x/val, 
            self.y/val, 
            self.z/val,
                )
    }
}

// Vector struct Utility functions

impl Vec3 {
    
    pub fn zero() -> Vec3 {
        Vec3{
            x: 0., 
            y: 0., 
            z: 0.,
        }
    }

    pub fn new<X, Y, Z>(x: X, y: Y, z: Z) -> Vec3 
        where X: Into<f64>,
              Y: Into<f64>,
              Z: Into<f64>,
    {
        Vec3 {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn is_close(self, v: Vec3) -> bool {
        length(self-v).abs() < 0.000001
    }

}

// Vector Utility functions

pub fn length(v: Vec3) -> f64 {
    f64::sqrt((v.x*v.x)+(v.y*v.y)+(v.z*v.z))
}

pub fn length_sq(v: Vec3) -> f64 {
    (v.x*v.x)+(v.y*v.y)+(v.z*v.z)
}

pub fn normalize(v: Vec3) -> Vec3 {
    v / length(v)
}

pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
    v1.x*v2.x + v1.y*v2.y + v1.z*v2.z
}

/// 
/// Returns the cross product between 'v1' and 'v2'
/// 
pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 {
        x: v1.y*v2.z - v2.y*v1.z,
        y: v1.z*v2.x - v2.z*v1.x,
        z: v1.x*v2.y - v2.x*v1.y,
    }
}

/// 
/// Returns the reflection of v according to Vect(n)
/// An optimized version exist and doesn't normalize 
/// n at each call in order to perform less calculations
/// 
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    let n = normalize(n);
    n * 2.*dot(n, v) - v
}

///
/// 'n' MUST be normalized 
/// Returns the reflection of v according to Vect(n)
///
pub fn reflect_opt(v: Vec3, n: Vec3) -> Vec3 {
    n * 2.*dot(n, v) - v
}

pub fn rand_in_unit_cube() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3 {
        x: rng.gen_range::<f64, _>(-1.0..=1.0),
        y: rng.gen_range::<f64, _>(-1.0..=1.0),
        z: rng.gen_range::<f64, _>(-1.0..=1.0),
    }
}

pub fn rand_on_unit_sphere() -> Vec3 {
    loop {
        let a = rand_in_unit_cube();
        let l = length(a);
        if l < 1. {
            return a/l;
        }
    }
}

pub fn lerp(v1: Vec3, v2: Vec3, x: f64) -> Vec3 {
    Vec3::new(
        v1.x*(1.-x)+v2.x*x,
        v1.y*(1.-x)+v2.y*x,
        v1.z*(1.-x)+v2.z*x,
    )
}


#[cfg(test)]
mod tests {

    use super::*;


    /* 
     *  Impl test Vector struct 
     */

    #[test]
    fn impl_zero() {
        let real_zero = Vec3{
            x: 0.,
            y: 0.,
            z: 0.,
        };

        assert_eq!(real_zero, Vec3::zero());
        assert_eq!(real_zero, Vec3::new(0, 0., -0));
        assert_eq!(real_zero, Vec3::new(0, 0., -0.));

    }

    /* 
     *  Cmp test Vector struct 
     */

    #[test]
    fn cmp_equal() {
        let any_vec = Vec3::new(3., 1., -9.);
        let same_vec = Vec3 {
            x: 3.,
            y: 1.,
            z: -9.,
        };
        assert_eq!(any_vec, same_vec);
        assert!(any_vec.is_close(same_vec));
    }

    /* 
     *  Operators test Vector struct 
     */

    #[test]
    fn op_new() {
        let any_vec = Vec3{
            x: 4.,
            y: -0.9,
            z: 100000.,
        };
        let same_vec = Vec3::new(4, -0.9, 100000);

        assert_eq!(any_vec, same_vec);

        let any_vec = Vec3{
            x: 0.,
            y: 5.,
            z: -3.,
        };

        let same_vec = Vec3::new(0, 5, -3);

        assert_eq!(any_vec, same_vec);
    }

    #[test]
    fn op_add() {

        let a = Vec3::new(4, -0.9, 100000);
        let b = Vec3::new(-4., 3, 9.);

        assert_eq!(a + b, Vec3::new(0, 2.1, 100009));

    }

    #[test]
    fn op_sub() {
        let a = Vec3::new(4, -0.9, 100000);
        let b = Vec3::new(-4., 3, 9.);

        assert_eq!(a - b, Vec3::new(8, -3.9, 99991));
    }

    #[test]
    fn op_mul() {

        let a = Vec3::new(4, -0.9, 100000);
        let b = Vec3::new(-4., 3, 9.);

        assert_eq!(a * b, Vec3::new(-16., -2.7, 900000));

        assert_eq!(a * 2, Vec3::new(8., -1.8, 200000));
        assert_eq!(b * (-5.), Vec3::new(20., -15., -45));

        assert_eq!(b * 0, Vec3::zero());
    }

    #[test]
    fn op_div() {
        let a = Vec3::new(4, -0.9, 100000);
        let b = Vec3::new(-4., 3, 100.);

        assert_eq!(a / b, Vec3::new(-1., -0.3, 1000));

        assert_eq!(a / 2, Vec3::new(2., -0.45, 50000));
        assert_eq!(b / (-5.), Vec3::new(0.8, -0.6, -20));
    }

    #[test]
    #[should_panic(expected = "Division by 0")]
    fn op_div_vec_panic() {
        Vec3::new(4, -0.9, 0.) / Vec3::new(5., 3, 0.);
    }

    #[test]
    #[should_panic(expected = "Division by 0")]
    fn op_div_scal_panic() {
        Vec3::new(4, -0.9, 100000) / 0;
    }


    /* 
     *   Vector Utility functions test 
     */


    #[test]
    fn util_length_and_length_sq() {

        assert_eq!(length(Vec3::new(0., 0.8, 0.6)), 1.);

        let l = length(Vec3::new(4., 5., 9.));
        assert_ne!(l, f64::NAN);
        assert!(
            f64::abs(l*l - length_sq(Vec3::new(4., 5., 9.))) < 0.0000001
        );

        assert!(
            f64::abs(length(Vec3::new(0., -3., 0.))-3.) < 0.0000001
        );
    }

    #[test]
    fn util_dot() {

        assert_eq!(dot(Vec3::new(0., 1., 0.), Vec3::new(0., -0., 4.)), 0.);

        assert_eq!(dot(Vec3::new(1., 1., 0.), Vec3::new(1., -1., 4.)), 0.);

        assert_eq!(dot(Vec3::new(4., 5., 9.), Vec3::new(4., 5., 9.)), length_sq(Vec3::new(4., 5., 9.)));

        assert_eq!(
            dot(Vec3::new(1., 4., -9.), Vec3::new(5., -3., 4.)),
            dot(Vec3::new(5., -3., 4.), Vec3::new(1., 4., -9.)),
        );
    }

    #[test]
    fn util_cross() {

        assert_eq!(
            cross(Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.)),
            Vec3::new(0., 0., 1.)
        );

        
        assert_eq!(
            cross(Vec3::new(0., 1., 0.), Vec3::new(0., 0., 1.)),
            Vec3::new(1., 0., 0.)
        );

        assert_eq!(
            cross(Vec3::new(0., 0., 1.), Vec3::new(1., 0., 0.)),
            Vec3::new(0., 1., 0.)
        );
        


        assert_eq!(
            cross(Vec3::new(1., 0., 0.), Vec3::new(0., 0., 1.)),
            Vec3::new(0., -1., 0.)
        );


        assert_eq!(
            cross(Vec3::new(1., 2., 3.), Vec3::new(9., -6., 0.2)),
            Vec3::new(18.4, 26.8, -24.),
        );
        
    }


    // TODO: More fancy tests
    #[test]
    fn util_reflect_and_reflect_opt() {

        let v = Vec3::new(1, 1, 0);
        let n = Vec3::new(0, 4, 0);

        assert_eq!(reflect(v, n), Vec3::new(-1, 1, 0));
        assert_ne!(reflect_opt(v, n), Vec3::new(-1, 1, 0));
        assert_eq!(reflect_opt(v, normalize(n)), Vec3::new(-1, 1, 0));

        let v = Vec3::new(3, 3, 3);
        let n = Vec3::new(-4, -4, -4);

        assert!(reflect(v, n).is_close(Vec3::new(3, 3, 3)));
    }

}
