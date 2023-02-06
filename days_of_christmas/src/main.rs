fn main() {

    let mut count = 0;

    let presents = ["12 drummers drumming",
    "11 pipers piping",
    "10 lords a leaping",
    "9 ladies dancing",
    "8 maids a milking",
    "7 swans a swimming",
    "6 geese a laying",
    "5 golden rings",
    "4 calling birds",
    "3 french hens",
    "2 turtle doves",
    "And a partridge in a pear tree"];

    let days = ["first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"];
    
    while count < 12 {
        println!("On the {} day of Christmas", days[count]);
        println!("My true love sent to me:");

        if count == 0 {
            println!("A partridge in a pear tree");
            println!(" ");
        } else {
            for index in (presents.len() - count - 1)..presents.len() {
                println!("{}", presents[index]); 
            }
            println!(" ");
        }     
        count += 1; 
    }
}
