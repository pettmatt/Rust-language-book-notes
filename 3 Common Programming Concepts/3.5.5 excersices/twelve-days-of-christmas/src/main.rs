fn main() {
    // NOTE: Total verses: 12
    // Which means the main loop should loop 12 times.

    let lyrics = [
        "On the twelfth day of Christmas, my true love sent to me",
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];

    let days = [ 
        "first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    println!("Christmas carol - Twelve days of Christmas");

    // "lyrics" alone isn't enough which is  
    // why it needs to be to be interated. 
    // In JS this would be achieved with "lyrics.length".
    for (i, _lyric) in lyrics.iter().enumerate() {
        // Does nothing but shows which verse the current one is.
        println!("Verse {}", i+1);

        let mut verse = String::new();
        verse += lyrics[0];
        verse = verse.replace("twelfth", days[i]);
        verse += "\n";

        if 0 < i {
            // 1..i -> iterate from 1 to X value.
            for j in 0..i {
                verse += lyrics[j+1];
                verse += "\n";
            }
        }

        verse += lyrics[12];

        // "i+2" because of the first and the last sentences should be ignored
        // if they are not the loop will keep going for too long and rust will panic,
        // beacuse the program is trying to access array[index] which is out of bound.
        if i+2 == lyrics.len() { break };
    }
}