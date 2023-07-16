# Halp!

A cli pipeline helper tool, backed by openai, inspired by `rg`, `jq`, etc.

# Usage

(This is pretend until I get it to work)

```
curl https://news.ycombinator.com | halp turn this into a csv of articles with title, domain name, link, points > out.csv
```

That should output .... a csv with a row for each article and a title, domain name, link, and points. Yup.

# Concept / Ideas

* Data input from STDIN
* Data output to STDOUT
* Instructions as the parameters
* Either explicit or implicit row-based processing. Maybe we should take the first chunk of data and ask the AI if it should be row based or not?
* Use TTY for additional guidance from the operator?

# References

* [Initial source used from async-openai (MIT)](https://github.com/64bit/async-openai)
