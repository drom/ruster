extern crate rand;
use rand::Rng;

fn main () {
    // TODO pick random from https://moodlist.net/
    let moods = [
        "Accepted", "Accomplished", "Aggravated", "Alone", "Amused", "Angry", "Annoyed", "Anxious",
        "Apathetic", "Apologetic", "Ashamed", "Awake", "Bewildered", "Bitchy", "Bittersweet",
        "Blah", "Blank", "Blissful", "Bored"
    ];
    let mut rng = rand::thread_rng();
    let ptr = rng.gen::<usize>() % moods.len();
    let mood = moods[ptr];
    println!("I am {}", mood);
}
