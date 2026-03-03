use serde::Deserialize; //Allows us to use the Deserialize trait from the serde crate. Think import in java

#[derive(Deserialize)] //Automatically generates JSON parsing code for Story struct
struct Story{
    title: String,
    url: Option<String>, //Generics
    score: u32, //u32 is a 32 bit unsigned integer. i32 would be signed.
    by: String,
    descendants: u32,
}

fn main() {
    println!("Top 10 Hacker News Stories\n");

    let client = reqwest::blocking::Client::new(); //Using reqwest's blocking client. HTTP requests run synchronously

    let top_ids: Vec<u64> = client //Dynamic array of u64 (unsigned 64 bit integers)
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")//Builds a GET request
        .send()//Sends that GET request
        .expect("Failed to fetch top stories")//Quick error handling. Program will crash with this message showing
        .json()//Parses the response body as JSON
        .expect("Failed to parse story IDs");
    
        for(i, id) in top_ids.iter().take(25).enumerate(){
            //.iter() creats the iterator for the top_ids vector
            //.take(10) defines the upper limit of the vector, being the first 10 items.
            //.enumerate() Gives us the index(i) and it's value(id)
            let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
            //format! is like a println! but it returns the String instead of printing it
            //{id} directly inserts the variable.
            
            let story: Story = client
                .get(&url)
                .send()
                .expect("Failed to fetch story")
                .json()
                .expect("Failed to parse story");
            
                let link = story.url.as_deref().unwrap_or("(no URL)");
                //.as_deref() converts Option<String> to Option<&str>
                // //.unwrap_or("(no URL)") provides a fallback if the URL is none
                println!("{}. {} ({} points by {}) {} comments", i+1, story.title, story.score, story.by, story.descendants);
                println!("  {}\n", link);
        }
}
