fn main() {



    let client = reqwest::ClientBuilder::new()
        .build()
        .unwrap();

        let f = std::fs::File::create("./dictionary_site_scrape.txt").unwrap();

        let mut wtr = LineWriter::new(f);

        'link_number:  for iter in 70241..=84_427 { // 84_427

        
            let url = format!("https://rus-forms.slovaronline.com/{iter}");
            println!("\nNEW URL\n{}", url); // !) PRINT STATEMENT

            let req = client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/113.0");

            let res = req
                .send()
                .await
                .unwrap()
                .text()
                .await
                .expect("parsing of text");

            // ?) Treating the response as html dom, find the 1st instance of a node with class "blockquote" if it exists or else move on the the next link
            let definition = match Document::from(res.as_str())
                .find(select::predicate::Class("blockquote"))
                .nth(0) {
                    None => continue 'link_number,
                    Some(d) => d.text(),
                };      
            
            // ?) Find the spot in the entry where this phrase occurs, which marks the end of the form list
            let loc = match definition.find("(Источник: «Полная акцентуированная парадигма по А. А. Зализняку»)") {
                None => {
                    println!("No location found for the block");
                    continue 'link_number
                },
                Some(loc) => loc
            };

            // ?) Give me just the data from the beginning to that point
            let mut word_stretch: String = definition[0..loc].to_string();

            // ?) If the page contains two different stress representations, give me just the first one
            if word_stretch.contains("1.") { 
                let loc = match definition.find("2. ") {
                    None => {
                        println!("No location found for '2.'");
                        continue 'link_number
                    },
                    Some(loc) => loc
                };
                word_stretch = word_stretch[3..word_stretch.floor_char_boundary(loc)].to_string(); //[2..loc]
            }


            let word_stretch = word_stretch
                .replace("\u{301}", "")
                .replace("\n", "")
                // .replace("а́", "а")
                // .replace("я́", "я")
                // .replace("о́", "о")
                // .replace("е́", "е")
                // .replace("ы́", "ы")
                // .replace("и́", "и")
                // .replace("у́", "у")
                // .replace("ю́", "ю");
                .replace("ё", "е");

            println!("Word_stretch: \n{:?}\n", word_stretch);

            let word_stretch = word_stretch + "\n";

            // ?) Take our list of word forms and write it to the file or else let me know it's failed
            match wtr.write_all(word_stretch.as_bytes()) {
                Ok(_) => (),
                Err(e) => println!("Hey man, couldn't write word_stretch {word_stretch} to ur file because {e}"),
            };

            
        
        } // 'link_number

        wtr.flush();

}