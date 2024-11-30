use curve25519_elligator2::scalar::Scalar;
use std::vec;

pub struct Poly {
    pub coeffs: Vec<Scalar>,  //6 -5x + x^2  => coeffs = [6,5,1]
}

impl Poly {
    fn minus_const(c: Scalar) -> Poly {
        let neg = -c; // Negate the constant

        Poly {
            coeffs: vec![neg, Scalar::ONE], // Create polynomial coefficients : neg + x
        }
    }

    fn add(&self, p: &Poly) -> Poly {
        if self.coeffs.len() != p.coeffs.len() {
            panic!("incorrect coeff length");
        }
        let coeffs: Vec<Scalar> = self
            .coeffs
            .iter()
            .zip(&p.coeffs) // Pair corresponding coefficients
            .map(|(a, b)| a + b) // Add them using the Scalar's add method
            .collect();
        
        let mut n = self.coeffs.len();
        for i in coeffs.iter().rev(){
            if *i == Scalar::ZERO{
                n-=1;
            } else{
                break;
            }
        }
        Poly { coeffs : coeffs[0..n].to_vec() }
    }

    fn mul(&self, q: &Poly) -> Poly {
        let d1 = self.coeffs.len() - 1;
        let d2 = q.coeffs.len() - 1;
        let new_degree = d1 + d2;

        let mut coeffs = vec![Scalar::ZERO; new_degree + 1]; // Initialize coefficients to zero

        for i in 0..self.coeffs.len() {
            for j in 0..q.coeffs.len() {
                let tmp = q.coeffs[j] * (self.coeffs[i]); // Multiply coefficients
                coeffs[i + j] = coeffs[i + j] + (tmp); // Add to the result
            }
        }

        Poly { coeffs } // Return the new polynomial
    }

    // Evaluates the polynomial at a given value of x
    pub fn evaluate(&self, x: Scalar) -> Scalar {
        let mut result = Scalar::ZERO; // Assuming Scalar has a zero method
        let mut x_pow = Scalar::ONE; // Starting from x^0

        for &coeff in self.coeffs.iter().rev() {
            result += coeff * x_pow; // result += coeff * x^i
            x_pow *= x; // Update x^i to x^(i+1)
        }

        result
    }
}


// Computes the Lagrange basis polynomial l_j
fn lagrange_basis(i: usize, xs: Vec<Scalar>) -> Poly {
    let mut basis = Poly {
        coeffs: vec![Scalar::ONE],
    };

    let mut acc = Scalar::ONE;

    for m in 0..xs.len() {
        if i == m {
            continue;
        }

        // Create -xm polynomial
        basis = basis.mul(&Poly::minus_const(xs[m].clone())); // Multiply by -xm polynomial

        let den = xs[i] - (xs[m]); // den = xi - xm
        let inverted_den = den.invert(); // den = 1 / den
        acc = acc * inverted_den; // acc = acc * den
    }

    // Multiply all coefficients by the denominator
    for coeff in &mut basis.coeffs {
        *coeff = *coeff * acc;
    }

    basis
}

pub fn recover_pri_poly(x: Vec<Scalar>, y: Vec<Scalar>) -> Result<Poly, String> {
    if x.len() != y.len() {
        return Err("x,y values do not have the correct length".to_string());
    }

    let mut acc_poly: Option<Poly> = None;

    for j in 0..x.len() {
        let mut basis = lagrange_basis(j, x.clone());

        for i in 0..basis.coeffs.len() {
            basis.coeffs[i] = basis.coeffs[i] * y[j];
        }

        if let Some(ref mut acc) = acc_poly {
            acc_poly = Some(acc.add(&basis)); // Add L_j * y_j
        } else {
            acc_poly = Some(basis);
        }
    }

    acc_poly.ok_or_else(|| "Failed to recover polynomial".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use curve25519_elligator2::scalar::Scalar;

    // Helper function to create Scalar from a u64 value (for easier test writing)
    fn scalar_from_u64(n: u64) -> Scalar {
        Scalar::from(n)
    }

    #[test]
    fn test_add_polynomials() {
        let p1 = Poly {
            coeffs: vec![scalar_from_u64(1), scalar_from_u64(2)],
        };
        let p2 = Poly {
            coeffs: vec![scalar_from_u64(3), scalar_from_u64(4)],
        };

        let result = p1.add(&p2);

        assert_eq!(result.coeffs.len(), 2);
        assert_eq!(result.coeffs[0], scalar_from_u64(4));
        assert_eq!(result.coeffs[1], scalar_from_u64(6));
    }

    #[test]
    fn test_multiply_polynomials() {
        let p1 = Poly {
            coeffs: vec![scalar_from_u64(1), scalar_from_u64(2)],
        };
        let p2 = Poly {
            coeffs: vec![scalar_from_u64(1), scalar_from_u64(3)],
        };

        let result = p1.mul(&p2);

        assert_eq!(result.coeffs.len(), 3);
        assert_eq!(result.coeffs[0], scalar_from_u64(1)); // 
        assert_eq!(result.coeffs[1], scalar_from_u64(5)); // 
        assert_eq!(result.coeffs[2], scalar_from_u64(6)); //
    }

    #[test]
    fn test_polynomial_evaluation() {
        let p = Poly {
            coeffs: vec![scalar_from_u64(1), scalar_from_u64(2), scalar_from_u64(3)],
        };

        let x = scalar_from_u64(2);
        let result = p.evaluate(x);

        // p(x) = 1 * x^2 + 2 * x + 3 = 1 * 4 + 2 * 2 + 3 = 4 + 4 + 3 = 11
        assert_eq!(result, scalar_from_u64(11));
    }

    #[test]
    fn test_lagrange_basis() {
        let xs = vec![
            scalar_from_u64(1),
            scalar_from_u64(2),
            scalar_from_u64(3),
        ];
        let basis_poly = lagrange_basis(2, xs.clone());

        // Expected coefficients for L_0(x) for points (1, 2, 3):
        // L_0(x) = ((x - 2)(x - 3)) / ((1 - 2)(1 - 3)) = (x^2 - 5x + 6) / 2
        let b = scalar_from_u64(2).invert();

        assert_eq!(basis_poly.coeffs.len(), 3); 
        assert_eq!(basis_poly.coeffs[0], scalar_from_u64(1)); // 1
        assert_eq!(basis_poly.coeffs[1], -scalar_from_u64(3)*b ); // -3/2
        assert_eq!(basis_poly.coeffs[2], scalar_from_u64(1)*b); // 1/2
    }

    #[test]
    fn test_recover_pri_poly() {
        let xs = vec![
            scalar_from_u64(1),
            scalar_from_u64(2),
            scalar_from_u64(3),
        ];
        let ys = vec![
            scalar_from_u64(2),
            scalar_from_u64(4),
            scalar_from_u64(6),
        ];

        let recovered_poly = recover_pri_poly(xs, ys).unwrap();

        // The recovered polynomial should be y = 2x, so the coefficients should be [0, 2]
        assert_eq!(recovered_poly.coeffs.len(), 2);
        assert_eq!(recovered_poly.coeffs[0], scalar_from_u64(0));
        assert_eq!(recovered_poly.coeffs[1], scalar_from_u64(2));
    }

    #[test]
    fn test_recover_pri_poly_empty_input() {
        let xs: Vec<Scalar> = vec![];
        let ys: Vec<Scalar> = vec![];

        let result = recover_pri_poly(xs, ys);

        assert!(result.is_err()); // Expect an error due to empty input
    }

    #[test]
    fn test_recover_pri_poly_single_point() {
        let xs = vec![scalar_from_u64(1)];
        let ys = vec![scalar_from_u64(2)];

        let recovered_poly = recover_pri_poly(xs, ys).unwrap();

        // Single point should directly return the constant polynomial
        assert_eq!(recovered_poly.coeffs.len(), 1);
        assert_eq!(recovered_poly.coeffs[0], scalar_from_u64(2));
    }

    #[test]
    fn test_recover_pri_poly_inconsistent_lengths() {
        let xs = vec![scalar_from_u64(1), scalar_from_u64(2)];
        let ys = vec![scalar_from_u64(2)];

        let result = recover_pri_poly(xs, ys);

        assert!(result.is_err()); // Expect an error due to inconsistent lengths
    }
}
