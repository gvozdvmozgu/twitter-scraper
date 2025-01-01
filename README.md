# twitter-scraper

## Getting Cookies from Firefox

1. Open inspector on page https://x.com/home

![alt text](image.png)

2. Find the address, you can filter it via api.x.com/1.1, or refresh the page and check the first request

![alt text](image-1.png)

3. After selecting the address, filter the headers by the word 'Cookie' and copy the cookies.

![alt text](image-2.png)

## Bearer token

- https://developer.x.com/en/docs/authentication/oauth-2-0/bearer-tokens

To create a bearer token you need to create an application according to the documentation, I used from the GO code `AAAAAAAAAAAAAAAAAAAAAAAFQODgEAAAAAVHTp76lzh3rFzcHbmHVvQxYYpTw%3DckAlMINMjmCwxUcaXbAN4XqJVdgMJaHqNOFgPMK0zN1qLqLQCF`

## Configuration of `twitter-scraper.toml`

```toml
base = "https://api.twitter.com"
bearer_token = "..."
auth = { type = "cookie", cookie = ''' ... ''' }
```

OR

```toml
base = "https://api.twitter.com"
bearer_token = "AAAAAAAAAAAAAAAAAAAAAFQODgEAAAAAVHTp76lzh3rFzcHbmHVvQxYYpTw%3DckAlMINMjmCwxUcaXbAN4XqJVdgMJaHqNOFgPMK0zN1qLqLQCF"
auth = { type = "user", username = "..", password = "..", email = ".." }
```


## Search tweets

```powershell
PS D:\Projects\gvozdvmozgu\twitter-scraper> cargo r -- tweets --help  
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\twitter-scraper.exe tweets --help`
Usage: twitter-scraper.exe tweets [OPTIONS] <QUERY>

Arguments:
  <QUERY>

Options:
      --from <FROM>
      --search-mode <SEARCH_MODE>  [default: top]
      --count <COUNT>              [default: 50]
      --cursor <CURSOR>
      --output <OUTPUT>            [default: pretty_print]
      --all
  -h, --help                       Print help
```

```powershell
PS D:\Projects\gvozdvmozgu\twitter-scraper> cargo r -- tweets bevy --count 1       
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\twitter-scraper.exe tweets bevy --count 1`
==============================
QueryTweetsResponse:
==============================

Tweet #1:
------------------------------
Text: From gamedev to solving quadratic equations real fast! #bevy #indiegames https://t.co/2oEOBakDAT
Created At: Mon Dec 30 11:41:42 +0000 2024
Likes: 14
Retweets: 1
Replies: 2
User ID: 1535005729
Username: OwlyCode
Photos:
  Photo: URL: https://pbs.twimg.com/media/GgC0h5tXgAAFY4F.png
------------------------------

Next Page: DAACCgACGgKa7QaAJxAKAAMaAprtBn_Y8AgABAAAAAILAAUAAAAoRW1QQzZ3QUFBZlEvZ0dKTjB2R3AvQUFBQUFFYUFMU01DZHF3ZFE9PQgABgAAAAAIAAcAAAAADAAICgABGgC0jAnasHUAAAA
Previous Page: DAACCgACGgKa7QaAJxAKAAMaAprtBn_Y8AgABAAAAAELAAUAAAAoRW1QQzZ3QUFBZlEvZ0dKTjB2R3AvQUFBQUFFYUFMU01DZHF3ZFE9PQgABgAAAAAIAAcAAAAADAAICgABGgC0jAnasHUAAAA
==============================
```


## Search profiles

```powershell
PS D:\Projects\gvozdvmozgu\twitter-scraper> cargo r -- profiles --help 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\twitter-scraper.exe profiles --help`
Usage: twitter-scraper.exe profiles [OPTIONS] <QUERY>

Arguments:
  <QUERY>

Options:
      --count <COUNT>    [default: 10]
      --cursor <CURSOR>
      --output <OUTPUT>  [default: pretty_print]
      --all
  -h, --help             Print help
```

```powershell
PS D:\Projects\gvozdvmozgu\twitter-scraper> cargo r -- profiles rust-lang --count 1
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running `target\debug\twitter-scraper.exe profiles rust-lang --count 1`
==============================
QueryProfilesResponse:
==============================

Profile #1:
------------------------------
ID: 165262228
Username: rustlang
Name: Rust Language
Description: A programming language empowering everyone to build reliable and efficient software.
Location:
URL: https://www.rust-lang.org/
Protected: false
Verified: false
Followers: 149486
Following: 2
Tweets: 17967
Listed: 1960
Created At: 2010-07-11 02:35:18 UTC
Profile Image URL: https://pbs.twimg.com/profile_images/1687033172905644032/ZjFPPLUM.jpg
Profile Banner URL: N/A
Pinned Tweet ID: N/A
Is Blue Verified: true
------------------------------

Next Page: DAAFCgABGgKaqci___0LAAIAAAAoRW1QQzZ3QUFBZlEvZ0dKTjB2R3AvQUFBQUFFQUFBQUFDZG16bEE9PQAA
Previous Page: DAAJAAA
==============================
```
