use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

use futures::TryStreamExt;
use gemini_rust::Gemini;

#[derive(Debug, Clone)]
struct MisuseError;
impl Error for MisuseError {}

impl fmt::Display for MisuseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

static PROMPT: &str = r"Analyze the following codebase for an A/D CTF service.
Describe what components it consists of, what programming languages the components use and what the service's intended purpose is.
The service likely has multiple vulnerabilities that require patches. Attempt to find said vulnerabilities. If possible, provide example patches.
Attempt to find places where flags may be stored.
For Docker compose/container files, notify if a container is privileged or otherwise has access it should not.
Be thorough and precise.";

fn gather_dir_entries(dir: &Path, file_entries: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let prettified = path.display();
        if path.is_file() {
            match fs::read_to_string(&path) {
                Ok(data) => {
                    file_entries.push(format!("File {prettified}:\n```\n{data}```\n"));
                }
                Err(_) => {
                    file_entries.push(format!(
                        "File {prettified}: \n<unknown, failed to read UTF8>\n"
                    ));
                }
            }
        } else if path.is_dir() {
            if path.file_name().unwrap().to_str().unwrap().starts_with(".") {
                println!("Skipping hidden directory {}!", prettified);
                continue;
            }
            gather_dir_entries(&path, file_entries)?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        eprintln!("Usage: {} dir_name", env!("CARGO_PKG_NAME"));
        return Err(MisuseError.into());
    }

    let api_key = match env::var("GEMINI_API_KEY") {
        Ok(s) => Ok(s),
        Err(e) => {
            eprintln!("Missing Gemini API key in env!");
            Err(e)
        }
    }?;

    let client = Gemini::new(api_key)?;

    let dir = Path::new(&argv[1]);
    let mut data = Vec::new();

    gather_dir_entries(dir, &mut data)?;

    let clanker_data_future = client
        .generate_content()
        .with_user_message(format!("{PROMPT}\n{}", data.join("\n")))
        .execute_stream();

    println!("Waiting for clanker...");

    let mut clanker_data = clanker_data_future.await?;

    let mut clanker_all_tokens = Vec::new();
    while let Some(chunk) = clanker_data.try_next().await? {
        let text = chunk.text();
        print!("{}", text);

        clanker_all_tokens.push(text);
    }
    println!("\n\nWriting clanker output to clanker.md...");

    std::fs::write("clanker.md", clanker_all_tokens.join(""))?;

    Ok(())
}
