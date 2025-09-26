# Vibe Summarizer

Describes a particular service's vibes using a clanker.
Originally written in [Python](https://gist.github.com/alphanin9/4e493fd5a9bca34cb4b9f35c40125b10),
but I decided to rewrite it in Rust because I couldn't be bothered to deal with `pip` and Cargo is a saner package manager.

# Usage

1. Get a Gemini API key from [Google AI Studio](https://aistudio.google.com/apikey), set env var `GEMINI_API_KEY` to use it
2. `cargo install --git https://github.com/alphanin9/VibeSummarizer.git`
3. `vibe-summarizer-rs service_dir`
4. ????
5. PROFIT

The clanker's output is also saved in `clanker.md`.
