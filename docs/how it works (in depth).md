## How does this project even work

This project is to 100% written in Rust, utilizing serval libraries which even make this project as great as it is at the moment.

### A list of the most important libraries which have been used
- **[poise](https://github.com/serenity-rs/poise)**
- **[tokio](https://tokio.rs/)**
- **[reqwest](https://github.com/seanmonstar/reqwest)**
- **[toml](https://github.com/toml-rs/toml)**
- **[serde_json](https://github.com/serde-rs/json)**
- **[sqlx postgre](https://github.com/launchbadge/sqlx)**
- **[image](https://github.com/image-rs/image)**
- **[wgpu](https://wgpu.rs/)**

there are a lot more to mention, but the libraries named above are the msot important ones. 

## Poise? What is that? how is it useful? 
Poise is a top layer library for [serenity](https://github.com/serenity-rs/serenity), a framework to build discord bots with.
Poise extends the functionality of Serenity, to be specific its a command framework, with poise it's much simpler to write slash commands and prefix commands at once, without the need of doing everything separately. But the most important piece of Poise is as mentioned earlier, Serenity, you can imagine Serenity as the heart of Poise, without Serenity, there would be no Poise. 

## Tokio? 
First of all, what even is Tokio?
Tokio is a runtime for writing asynchronous code in Rust, providing tools and abstractions to efficiently handle tasks like networking, file I/O, and concurrency. It enables developers to build high-performance and scalable applications by leveraging Rust's async/await syntax and non-blocking I/O operations.
Without tokio, frameworks like serenity and everything else which relies somehow on async/await wouldnt be as good as it is to this day. 

## Reqwest?
Well, what even is Reqwest?
Reqwest is a high-level HTTP client library for Rust that simplifies making HTTP requests and handling/ responses. It provides a convenient API for common HTTP operations like GET, POST, and more, making it easier to interact with web services and APIs in Rust applications.
Without Reqwest, some important things in the bot wouldnt be as efficient as they are right now, Poise and Serenity are not to 100% perfect, and that is why you sometimes have to come up with own implementations, and that is where Reqwest enters the game, some things can be just done faster and more efficient by directly sending a HTTP request to the discord api,compared to Serenity.

## toml and serde_json? 
I will keep it short.
Both toml and serde_json are parsing libraries for certain file types, to be specific, for .json, serde_json and for .toml, toml
These libraries make it easier possible with their advanced features to get uploaded files through discords slashcommands and validate the data in the either json or toml file for correctness. 

## SQLx and PostgreSQL
Well as you know, we need to store user data to maintain the bot functional... should be clear
SQLx is a wrapper for databases like MySql, PostgreSql which we use, and SQLite for smaller applications with only low data traffic 
Both MySql and PostgreSQL are good tools, well personally, we decided to use PostgreSQL because we believe it's easier to use. 
Well a good database like postgreSQL is needed to store, fast, securely, concurrently and efficient user data as mentioned earlier.

## The Image crate in combination with wgpu
If you have already played around a little with the discord bot, you might have found out there is a `/avatar` slash command, which offers various features like

- grayscale filter
- invert filter
- sepia tone filter

somehow we need to process these pixels of the image and change them, and there is where the image crate comes into place, which allows us to change pixels and save them. Well, just doing it with the CPU would be boring right? and why would we want to do it with the CPU, if we have these days good GPUs which can take these kind of worksloads off. 

this is where wgpu comes into place, a library which allows us to utilize the gpu for these tasks, and our cpu can relax while the images are being processed by the gpu, which saves us a bunch of resources. 

Well unfortunately, this is the only part of the project, where we did not use rust to 100%.. we had to use partially wgsl to give the GPU instructions about what it needs to do, this typical involves coding a shader in specific languages made for this purpose, aka wgsl.

## The result of everything

Well we can clearly see the results, a reliable, efficient discord bot which makes use of multi threading to get more tasks done at once, thanks to the rust language we have more control over our own code and can partially how memory is being managed compared to languages with a garbage collector, like Go or JavaScript. Rust gives us the space we need for optimising our code as good as possible to aim for the best results, in speed, efficiency, and performance even on systems which have less resources available. 

We already tested the bot on a very old PC with a i7 870, 8gb 1333 mhz ram, and as a gpu it uses a gt 710, it works all wonderful together with the bot. 