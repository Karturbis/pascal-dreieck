fn main() {
    let number:u128 = 16;
    fakultaet(number)
}

/**
Berechnet die Fakultaet der uebergebenen Zahl.
*/
fn fakultaet(faknumber:u128){

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
    while i <= faknumber{
        
        result = result * i;

        i +=1;
    }
    println!("Die Fakultät der Nummer ist: {}", result);

}
