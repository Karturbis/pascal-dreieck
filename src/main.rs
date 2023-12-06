
use std::io;
use num_bigint::BigUint;
use num_traits::FromPrimitive;


fn main() {


    //how to assign BIGINT:  //let bignumber: BigUint = BigUint::from(variable:u128);
    
    let mut inputn = String::new();
    println!("Please type the row of the Pascla-triangle, you want to calculate.");
    io::stdin()
        .read_line(&mut inputn)
        .expect("Could not read input from stdin.");

    let n:u128 = inputn.trim().parse::<u128>().unwrap();
    binomkoeff(n);
}

/**
 * Berechnet die Fakultaet der uebergebenen Zahl.
 * die uebergebene Zahl darf maximal 34 sein.
*/
fn fakul(faknum:u128) -> BigUint{

    if true{ //TODO remove if statement

        let mut result:BigUint = BigUint::from_u64(1).unwrap();

        let mut i:u128 = 1;
        /*
        Fakultäten:
        0! = 1          = 1
        1! = 1          = 1
        2! = 1*2        = 2
        3! = 1*2*3      = 6
        4! = 1*2*3*4    = 24
        5! = 1*2*3*4*5  = 120
        daher x*(x-1)! = x!
        */ 
        while i <= faknum{
            
            let j:BigUint = BigUint::from_u128(i).unwrap();
            result = result * j;

            i += 1;
        }
        return result;
    }
    else {
        println!("Die an Fakul übergebene Zahl ist größer als 34, das Programm wird beendet!");
        std::process::exit(1);
    }

}

/**
 * berechnet den Binomialkoeffizienten der uebergebenen Zahlen 'n' und 'k' und gibt das Ergebnis ueber die Konsole aus.
*/
fn binomkoeff(n:u128,) {
   
    let mut k:u128 = 0;
    let mut result:BigUint = BigUint::from_u8(1).unwrap();
    //n ueber k = n!/(k!*(n-k)!)

    while k <= n {
        
        result = fakul(n)/(fakul(k)*fakul(n-k));
        println!("Die Stelle {0} der Reihe{1} ist: {2}", k, n, result);
        k += 1;
    }
    
}
