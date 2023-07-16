# iwt: I Wish To...

> A cli pipeline helper tool, backed by openai, inspired by `rg`, `jq`, etc. But you know .... magic instead of science. Make a wish!

This is currently using the OpenAI API. I don't have access to GPT-4 API yet, so this is using `gpt-3.5-turbo-16k`. Eventually that should be configurable, and even cooler would be to support alternate APIs or local models. Let's see if we can get it working at all first though, k?

## Usage

First, set up your OpenAI API key:

```sh
export OPENAI_API_KEY=yourkeyhere
```

Generally: `<some command> | iwt <some wish> | <some other command>`

Example: `curl https://news.ycombinator.com | iwt turn this into a csv of articles with title, domain name, link, points > out.csv`

That should output a csv with a row for each article and a title, domain name, link, and points. Yup. Like magic.

## Concept / Ideas

* Data input from STDIN
* Data output to STDOUT
* Instructions as the parameters
* Either explicit or implicit row-based processing. Maybe we should take the first chunk of data and ask the AI if it should be row based or not?
* Use TTY for additional guidance from the operator?
* Break long input into batches within token/context limits
* Warn (again, via TTY) for LARGE input?
* Progress via STDERR

## References, Simlar Things, Cool Stuff

* [Initial source used from async-openai chat example (MIT)](https://github.com/64bit/async-openai)
* [plz is a universal CLI tool that converts plain-English into an executable command.](https://plz.software/)
* [openai-cli](https://github.com/peterdemin/openai-cli)
* [Another openai-cli](https://github.com/levitatingbusinessman/openai-cli)
* [prompt](https://github.com/raiyanyahya/prompt)
* [chat-openai-cli](https://github.com/maurobonfietti/chat-open-ai-cli)
