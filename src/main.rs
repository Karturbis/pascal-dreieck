use std::io;

fn main() {

    let n:u128 = 12;
    let k:u128 = 8;
    binomkoeff(n, k);
}

/**
 * Berechnet die Fakultaet der uebergebenen Zahl.
 * die uebergebene Zahl darf maximal 34 sein.
*/
fn fakul(faknum:u128) -> u128{

    if faknum <=34{

        let mut result:u128 = 1;
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
            
            result = result * i;

            i +=1;
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
fn binomkoeff(n:u128, k:u128) {

    //n ueber k = n!/(k!*(n-k)!)

    let result:u128 = fakul(n)/(fakul(k)*fakul(n-k));

    println!("Der Binomialkoeffizient ist: {}", result)
    
}
