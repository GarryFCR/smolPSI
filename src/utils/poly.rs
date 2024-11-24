use curve25519_elligator2::scalar::Scalar;

pub struct Poly {
    coeffs: Vec<Scalar>,
}

impl Poly {
    fn minus_const(c: Scalar) -> Poly {
        let neg = -c; // Negate the constant

        Poly {
            coeffs: vec![neg, Scalar::ONE], // Create polynomial coefficients
        }
    }

    fn Add(&self, p: &Poly) -> Poly {
        if self.coeffs.len() != p.coeffs.len() {
            panic!("incorrect coeff length");
        }
        let coeffs: Vec<Scalar> = self
            .coeffs
            .iter()
            .zip(&p.coeffs) // Pair corresponding coefficients
            .map(|(a, b)| a + b) // Add them using the Scalar's add method
            .collect();

        Poly { coeffs }
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
            let mut basis = Poly::lagrange_basis(j, x.clone());

            for i in 0..basis.coeffs.len() {
                basis.coeffs[i] = basis.coeffs[i] * y[j];
            }

            if let Some(ref mut acc) = acc_poly {
                acc_poly = Some(acc.Add(&basis)); // Add L_j * y_j
            } else {
                acc_poly = Some(basis);
            }
        }

        acc_poly.ok_or_else(|| "Failed to recover polynomial".to_string())
    }
}
