# rust_unfollow
A small program that finds the people who don't follow you back on Instagram.
It does __not__ use the Instagram API and __does__ require you to have Google Chrome installed.

### This program is totally safe
It doesn't steal you credentials or hack your account. Your credentials are only sent to Instagram to login.

### It is only tested and working on Windows 10
And pressumably older Windows versions.
Linux and MacOS might have some problems.

## How to
It is quite simple to get it working.
  1. Input your Instagram username
  2. Input your account's password
  3. Input the username of the person you want to find the "unfollowers".
     This can be your own username, a username of a public account or a private account you follow
  4. Wait (for quite a while...) 
  
     >This program scrapes Instagram with a headless Chrome instance and communicates via the DevTools protocol and thus it needs to be slow about it...
     
  5. After it is done getting both the following and the followers, it will display all of the accounts that don't follow you back in a list. 
  
     >It also creates a file called "unfollowers.txt" in the same directory as the program that contains the same list. This helps out the hasty people that close the program after it's done but forget to see the list.
  
  6. And that's it! Enjoy :)
  
## But how do I get it?
There are two ways to do it
  1. Download the binary file (.exe, etc) from the [release page](https://github.com/theoteck/rust_unfollow/releases/latest) of the repository (Boringg wayy)
  2. Download the [Rust toolchain](https://www.rust-lang.org/tools/install) and [build it](https://doc.rust-lang.org/cargo/) yourself with cargo (Awesomee way)
