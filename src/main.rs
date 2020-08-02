/* 
 * MIT License
 *
 * Copyright (c) 2020 Theocharis Giannitsis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE. 
**/

/*
 * This is not the most efficient program ever created but it gets the job done :) (it's my first project in Rust)
 * Most of the things are repeated twice, both for the followers and for the following,
 * making them an excellent candidate for a function, but I'm too bored to make it.
 * 
 * The way the program finds most of it's info (by css classes) is prone to cause errors in the future,
 * as Instagram might change those class names or even change the entire DOM.
 * 
 * Tho, I am probably going to keep maintaining it until my friends stop asking for the people that don't follow them back
 * (not soon)
**/

use failure::Fallible;
use headless_chrome::Browser;
use std::io::{self, Write, stdin, prelude::*};
use std::{thread, time};
use std::fs::File;


fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}


fn print_num(num: &usize) {
    print!("\r");

    print!("{}", num);

    io::stdout().flush()
        .unwrap();
}


fn save_to_file(name: &str, data: &Vec<&String>) {
    let mut f = File::create(name)
        .unwrap();

    write!(
        f, "{}", format!("{:#?}", data)
    ).unwrap();
    
}


fn fetch_unfollowers() -> Fallible<()> {

    let sleep_time = time::Duration::from_secs(5);
    
    // User input --------------------------

    let mut username = String::new();
    let mut password = String::new();
    let mut creep = String::new();


    println!("Enter username:");

    stdin().read_line(&mut username)
        .expect("Error reading the username");
        
    println!("");


    println!("Enter password:");

    stdin().read_line(&mut password)
        .expect("Error reading the password");
    
    println!("");

    println!("Enter username to fetch unfollowers for: (public acc or you follow it)");

    stdin().read_line(&mut creep)
        .expect("Error reading the username");
    
    println!("");


    //We do some trimming to remove the CRLF from the back of the strings
    username.truncate(username.len() - 2);
    password.truncate(password.len() - 2);
    creep.truncate(creep.len() - 2);

    //--------------------------------------


    //Init and logon -----------------------

    let browser = Browser::default()
        .unwrap();

    let tab = browser.wait_for_initial_tab()
        .unwrap();


    print!("Logging in... ");
    io::stdout().flush()
        .unwrap();

    //

    tab.navigate_to("https://www.instagram.com/accounts/login/")
        .unwrap();
    
    //

    thread::sleep(sleep_time);


    //Username field
    tab.wait_for_element("#react-root > section > main > div > article > div > div:nth-child(1) > div > form > div:nth-child(2) > div > label > input")
        .unwrap()
        .type_into(&username)?;

    //

    //Password field
    tab.wait_for_element("#react-root > section > main > div > article > div > div:nth-child(1) > div > form > div:nth-child(3) > div > label > input")
        .unwrap()    
        .type_into(&password)?;

    //

    thread::sleep(sleep_time); // Go to sleep for 5s
    
    //Log-in button
    tab.wait_for_element("#react-root > section > main > div > article > div > div:nth-child(1) > div > form > div:nth-child(4) > button > div")?
        .click()?;

    //

    print!("Done!\n");
    println!("");

    thread::sleep(sleep_time); // Go to sleep for 5s

    //--------------------------------------


    let mut follower_name_list : Vec<String> = vec![];
    let mut followed_name_list : Vec<String> = vec![];


    //Get following ------------------------

    tab.navigate_to(
        &format!("https://www.instagram.com/{}/", creep)
    )?;

    //We get the number of following from the profile page
    let followed_count_str = tab.wait_for_element("#react-root > section > main > div > header > section > ul > li:nth-child(3) > a > span")?
                            .get_inner_text()?;

    let followed_count = followed_count_str.replace(",", "")
                            .parse::<i32>()?;

    println!("{} follows {} account/s", creep, followed_count);
    println!("");
    
    //Followed
    tab.wait_for_element("#react-root > section > main > div > header > section > ul > li:nth-child(3) > a")?
        .click()?;

    //

    thread::sleep(sleep_time * 2); // Go to sleep for 10s (plotwist)

    assert_ne!(
        format!("https://www.instagram.com/{}/following", creep),
        tab.get_url()
    );

    let followed_holder = tab.find_element("div.PZuss").unwrap();

    let followed_list = followed_holder.find_elements("li").unwrap();


    let mut butitsthesamee_fod = 0;
    let mut prev_count_fod : usize = 0;


    println!("Getting following...");
    
    while followed_count > followed_list.len() as i32 {
        let followed_list = followed_holder.find_elements("li");

        //Not the best practice, I think, but it gets the job done!
        let followed_list = match followed_list {
            Ok(followed_list) => followed_list,
            Err(_error) => match followed_holder.find_elements("li") {
                Ok(followed_list_2) => followed_list_2,
                Err(error) => panic!("Err {} happened twice", error)
            }
        
        };

        let length = followed_list.len();

        print_num(&length);
        
        followed_list[length - 1].move_mouse_over()?;

        thread::sleep(time::Duration::from_secs(2));

        if prev_count_fod == length {
            butitsthesamee_fod += 1;
        }
        else {
            prev_count_fod = length;
        }

        if butitsthesamee_fod > 3 {
            println!("");
            println!("Found the same following count the past 4 times...");
            println!("Either Instagram is bad at counting or you have a network error (probably the 1st)");
            println!("Continuing...");
            break;
        }
    }

    println!("");

    for followed in &followed_list {
        
        followed_name_list.push(
            followed.find_element("a.FPmhX")?
                .get_inner_text()?
        );

    }

    println!("Done!");
    println!("");

    //--------------------------------------


    //Get followers ------------------------

    tab.navigate_to(
        &format!("https://www.instagram.com/{}/", creep)
    )?;

    thread::sleep(time::Duration::from_secs(2));

    //We get the number of followers from the profile page
    let follower_count_str = tab.wait_for_element("#react-root > section > main > div > header > section > ul > li:nth-child(2) > a > span")?
                            .get_inner_text()?;

    let follower_count = follower_count_str.replace(",", "")
                            .parse::<i32>()?;


    println!("{} has {} follower/s", creep, follower_count);
    println!("");

    //Followers
    tab.wait_for_element("#react-root > section > main > div > header > section > ul > li:nth-child(2) > a")?
        .click()?;

    //

    thread::sleep(sleep_time * 2); // Go to sleep for 10s (plotwist)

    assert_ne!(
        format!("https://www.instagram.com/{}/followers", creep),
        tab.get_url()
    );
    

    let follower_holder = tab.find_element("div.PZuss")?;

    let follower_list = follower_holder.find_elements("li")?;


    let mut butitsthesamee_fol = 0;
    let mut prev_count_fol : usize = 0;


    println!("Getting followers...");

    while follower_count > follower_list.len() as i32 {
        let follower_list = follower_holder.find_elements("li");

        //Again. Not the best practice I think but it gets the job done!
        let follower_list = match follower_list {
            Ok(follower_list) => follower_list,
            Err(_error) => match follower_holder.find_elements("li") {
                Ok(follower_list_2) => follower_list_2,
                Err(error) => panic!("Err {} happened twice", error)
            }
        
        };
        
        let length = follower_list.len();

        print_num(&length);
        
        follower_list[length - 1].move_mouse_over()?;

        thread::sleep(time::Duration::from_secs(2));

        if prev_count_fol == length {
            butitsthesamee_fol += 1;
        }
        else {
            prev_count_fol = length;
        }

        if butitsthesamee_fol > 3 {
            println!("");
            println!("Found the same follower count the past 4 times...");
            println!("Either Instagram is bad at counting or you have a network error (probably the 1st)");
            println!("Continuing...");
            break;
        }
    }

    println!("");

    for follower in &follower_list {
        
        follower_name_list.push(
            follower.find_element("a.FPmhX").unwrap()
                .get_inner_text()?
        );

    }

    println!("Done!");
    println!("");

    //--------------------------------------


    //Get them sweet unfollowers (bad terminology I know) -----------

    println!("Calculating people you should probably unfollow...");

    let mut unfollowers : Vec<&String> = vec![];

    for i in 0..&followed_name_list.len() - 1 {
        let mut itmatches = false;

        for j in 0..&follower_name_list.len() - 1{
            if &followed_name_list[i] == &follower_name_list[j] { itmatches = true; }
        }

        if !itmatches {
            unfollowers.push(&followed_name_list[i]);
        }
    }

    println!("Done!");
    println!("");

    //--------------------------------------

    println!("They are...");
    println!("");

    println!("{:#?}", unfollowers);

    save_to_file("unfollowers.txt", &unfollowers);

    pause();

    Ok(())
}

fn main() -> Fallible<()> {
    println!("");
    println!("Instagram Unfollower List");
    println!("");

    fetch_unfollowers()
}