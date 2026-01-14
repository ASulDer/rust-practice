use std::fs::File;
use std::env;
use std::io::{BufReader, BufRead, Write};
use std::str::FromStr;
use rand::thread_rng;
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_traits::{FromPrimitive, One, Zero};

// ==================
// |     STRUCTS    |
// ==================

enum Mode {
    RabinMiller,
    Fermat,
}

enum IsPrime {
    ProbablyPrime,
    Composite,
    CompositeWithNumber,
}

// ==================
// |      MAIN      |
// ==================

fn main() -> std::io::Result<()> {
    let mut mode: Mode = Mode::RabinMiller;
    for argument in env::args().skip(1) {
        match argument.as_str() {
            "-f" => mode = Mode::Fermat,
            _ => panic!("Nie ma takiej opcji!"),
        }
    }

    let data = prime_test(mode);
    let _ = write_to_file("wyjscie.txt", data);

    Ok(())
}

fn prime_test(mode: Mode) -> (IsPrime, BigUint) {
    let input_file = File::open("wejscie.txt").unwrap();
    let mut number_reader = BufReader::new(input_file);
    let mut big_numbers: Vec<String> = Vec::new();
    while {
        let mut buffer_str = String::new();
        number_reader.read_line(&mut buffer_str).expect("Nie dało się przeczytać danych.");
        big_numbers.push(buffer_str);

        !number_reader.buffer().is_empty()
    } {}

    // big_numbers content:
    // 0 - number n (test "object")
    // 1 - universal exponent / factor of (universal exponent+1)
    // 2 - factor of (universal exponent+1)
    
    match mode {
        Mode::RabinMiller => {
            let n = BigUint::from_str(big_numbers[0].trim()).unwrap();
            let r;
            match big_numbers.len() {
                1 => r = BigUint::zero(),

                2 => r = BigUint::from_str(big_numbers[1].trim()).unwrap(),

                3 => {
                    let a = BigUint::from_str(big_numbers[1].trim()).unwrap();
                    let b = BigUint::from_str(big_numbers[2].trim()).unwrap();
                    r = (a * b) - BigUint::one();
                },

                _ => panic!("Too much lines in input file!"),
            }

            miller_rabin(&n, &r)
        },

        Mode::Fermat => {
            let n = BigUint::from_str(big_numbers[0].trim()).unwrap();
            fermat_test(&n)
        },
    }
}

fn fermat_test(n: &BigUint) -> (IsPrime, BigUint) {
    if n == &BigUint::from_u8(2).unwrap() || n == &BigUint::from_u8(3).unwrap() {
        return (IsPrime::ProbablyPrime, BigUint::zero());
    }
    
    if n <= &BigUint::one() || n % 2u8 == BigUint::zero() {
        return (IsPrime::Composite, BigUint::zero());
    }
    
    let n_minus_one = n - BigUint::one();
    let mut rng = rand::thread_rng();
    
    for _ in 0..40 {
        let a: BigUint = rng.gen_biguint_range(&2u8.to_biguint().unwrap(), &(n - 2u8));

        let devisor = gcd(a.clone(), n.clone());
        if &devisor > &BigUint::one() && &devisor < n {
            return (IsPrime::Composite, BigUint::zero());
        }

        let result = fast_powering(&a, &n_minus_one, n);
        
        if result != BigUint::one() {
            return (IsPrime::Composite, BigUint::zero());
        }
    }
    
    // Probability: 2^-40
    (IsPrime::ProbablyPrime, BigUint::zero())
}

fn miller_rabin(n: &BigUint, r: &BigUint) -> (IsPrime, BigUint) {
    if n <= &1u8.to_biguint().unwrap() {
        return (IsPrime::Composite, BigUint::zero());
    }
    if n <= &3u8.to_biguint().unwrap() {
        return (IsPrime::ProbablyPrime, BigUint::zero());
    }
    if n % 2u8 == BigUint::zero() {
        return (IsPrime::CompositeWithNumber, 2u8.to_biguint().unwrap());
    }
    
    // n-1 = m * 2^k
    let n_minus_one = if *r == BigUint::zero() {
        n - BigUint::one()
    } else {
        r.clone()
    };
    let mut m = n_minus_one.clone();
    let mut k = 0u64;
    
    while &m % 2u8 == BigUint::zero() {
        m >>= 1;
        k += 1;
    }
    
    let mut rng = thread_rng();
    for _ in 0..40 {
        let a = rng.gen_biguint_range(&2u8.to_biguint().unwrap(), &(n - 2u8));
        
        let mut b = fast_powering(&a, &m, n);
        if b == BigUint::one() || b == n_minus_one {
            continue;
        }

        // b_0, b_j, b_j+1, b_j+2 etc. iteration
        for _ in 0..k {
            let b_prev = b.clone();
            b = (&b * &b) % n;

            if b == BigUint::one() {
                if b_prev != n_minus_one {
                    let factor1 = gcd(&b_prev - 1u8, n.clone());
                    let factor2 = gcd(&b_prev + 1u8, n.clone());
                    
                    if &factor1 > &BigUint::one() && &factor1 < n {
                        return (IsPrime::CompositeWithNumber, factor1);
                    }
                    if &factor2 > &BigUint::one() && &factor2 < n {
                        return (IsPrime::CompositeWithNumber, factor2);
                    }
                    
                    return (IsPrime::Composite, BigUint::zero());
                } else {
                    break;
                }
            }
        }
        
        // Fermat condition
        if b != BigUint::one() {
            return (IsPrime::Composite, BigUint::zero());
        }
    }
    
    (IsPrime::ProbablyPrime, BigUint::zero())
}

fn fast_powering(a: &BigUint, exponent: &BigUint, module: &BigUint) -> BigUint {
    if module.is_one() {
        return BigUint::zero();
    }
    
    let mut result = BigUint::one();
    let mut a = a % module;
    let mut exp = exponent.clone();
    
    while exp > BigUint::zero() {
        if &exp & BigUint::one() == BigUint::one() {
            result = (result * &a) % module;
        }
        
        a = (&a * &a) % module;
        exp >>= 1;
    }
    
    result
}

fn gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let temp = b.clone();
        b = a % &temp;
        a = temp;
    }
    a
}

fn write_to_file(filename: &str, content: (IsPrime, BigUint)) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    match content.0 {
        IsPrime::ProbablyPrime => writeln!(file, "Most likely prime.")?,
        IsPrime::Composite => writeln!(file, "Surely composite.")?,
        IsPrime::CompositeWithNumber => writeln!(file, "{}", content.1)?,
    }

    Ok(())
}