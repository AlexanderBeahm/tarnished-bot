use rand::{thread_rng, Rng};
use std::{thread, time};

mod lists;
mod twitter_client;

#[tokio::main]
async fn main() {    
    let conjunctions_list = lists::get_conjunctions();
    let words_list = lists::get_words();
    let templates_list = lists::get_templates();

    //Main program lifeline loop.
    loop {
        lifeline_loop(conjunctions_list, words_list, templates_list)
    }

    //twitter_client::ping_twitter().await;
    //let response = twitter_client::login_twitter().await;
    //println!("Logged in");
}

fn lifeline_loop(conjunctions_list: [&str; 10], words_list: [&str; 369], templates_list: [&str; 21]) {
    let mut rng = thread_rng();
        
    let conjunction_chance = rng.gen_ratio(1, 5);

    let conjunction_index: usize = rng.gen_range(0..10);
    let templates_index: usize = rng.gen_range(0..21);
    let templates_index_second: usize = rng.gen_range(0..21);
    
    let words_index: usize = rng.gen_range(0..369);
    let words_index_second: usize = rng.gen_range(0..369);

    let selected_conjunction = conjunctions_list.get(conjunction_index).unwrap();
    let selected_template = templates_list.get(templates_index).unwrap();
    let selected_template_second = templates_list.get(templates_index_second).unwrap();
    let selected_words_second = words_list.get(words_index_second).unwrap();
    let selected_words = words_list.get(words_index).unwrap();

    println!("Selected tarnished message components:");

    println!("{selected_template}");
    println!("{selected_words}");

    if conjunction_chance {
        println!("{selected_conjunction}");
        println!("{selected_template_second}");
        println!("{selected_words_second}");
    }

    let phrase = &selected_template.replacen("{}", selected_words, 2);

    if conjunction_chance {
        let second_phrase = &selected_template_second.replacen("{}", selected_words_second, 2);
        let conjunction_phrase = &selected_conjunction.replacen("{}", phrase, 1);
        let conjunction_phrase = &conjunction_phrase.replacen("{}", second_phrase, 1);
        println!("{conjunction_phrase}");
    }
    else {
        println!("{phrase}");
    }

    
    let ten_millis = time::Duration::from_millis(10000);
    //let now = time::Instant::now();

    thread::sleep(ten_millis);
}
