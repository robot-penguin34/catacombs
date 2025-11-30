# CATACOMBS: Delve into your documents.
A simple static site generator written in rust.

## Installation
This installation is basic and is likely not production ready.
> [!IMPORTANT]
> This project uses a couple of less common libraries. Make sure you trust those before running this project (remember, it's still executable code).
> Also, it is not intended to be ran on windows. (use at your own risk and upkeep).
1. `git clone` this repository (the current website likely, or the code tab)
2. `cd` into the directory, the command will probably be `cd catacombs`
3. add an index.md \[using markdown] (or index.html) inside the `public/` folder using the guide below (Adding pages)
4. `cargo run`, if you have rust installed. Otherwise, follow a guide on installing rust.

## Adding pages
All documents and folders under the `public/` folder are accessible to anyone viewing your server. You can add any file to this folder and it will be sent to the client, but only markdown (`.md`) and html (`.html`) files will be rendered by default.

## Modifying the theme:
> [!NOTE]
> This project is in very early stages, because of that, many features might not work or be fully implemented.
- open `resources/style.html` with your desired code editor
- edit as html [(beginner documentation from w3schools)](https://www.w3schools.com/html/default.asp) and use custom formatting (as seen below):
    - `{{ content }}` will be replaced with the content of an entry
    - `{{ title }}` will be the filename currently.
    > have an idea for another template? submit a github issue. (disclaimer: due to maintanance reasons, I can't accept most requests)

## Modifying the colors:
1. open `public/app.css` with your desiered code editor
2. edit as common css [(w3schools has a good tutorial)](https://www.w3schools.com/Css/)
