# melcher.io

Hi!

This is the repo for my own website, but it's a bit unconventional. I don't really like html and js (me 🤝 JavaScript) so I'm writing my own website in <bold>Rust</bold> and with the help of my self-made templating engine [html-site-generator](https://github.com/letsmelon/html-site-generator).

Another problem I encountered is that I'm hosting my website on a small server and I want to rebuild the website whenever something changes at the github repo. So I created `/github_webhook_handler` a small program that can accept webhook events from github and if desired (only on a creation of a tag) the website gets rebuild via Docker.
