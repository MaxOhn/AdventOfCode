use std::env;
use std::fmt::Write as _;
use std::fs;
use std::io::{Result as IoResult, Write as _};
use std::path::{Path, PathBuf};

/*
    In order for solutions to be picked up:
        - The name of the directory of the year must be the year and be located next to the `aoc-solver` directory
        - The directory must contain a `Cargo.toml` file to ensure it's a rust project
        - The year's project's name is assumed to be `aoc{year:02}`
        - The directory must contain a `src` directory
        - For each day, the `src` directory is expected to have either a file called `day{:02}.rs` or a directory called `day{:02}`
        - Each day's module is assumed to expose a function of the following form:
            ```
            pub fn run(input: &str) -> eyre::Result<aoc_rust::Solution> {
                ...
            }
            ```
*/

fn main() -> IoResult<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("years.rs");
    let mut dest = fs::File::create(&dest_path)?;

    let mut content = Content::new();

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let mut project_path = PathBuf::from(manifest_dir);
    let _ = project_path.pop();
    handle_years(&project_path, &mut content)?;

    content.finish();
    content.write(&mut dest)?;

    println!("cargo:rerun-if-changed={}", project_path.display());

    Ok(())
}

fn handle_years(project_path: impl AsRef<Path>, content: &mut Content) -> IoResult<()> {
    for res in fs::read_dir(project_path)? {
        let entry = res?;
        let file_type = entry.file_type()?;

        if !file_type.is_dir() {
            continue;
        }

        handle_year(entry, content)?;
    }

    let _ = writeln!(
        content.solve_fn,
        "        _ => |_| eyre::bail!(\"invalid year\"),"
    );

    let _ = writeln!(content.solved_days, "        _ => 0,");

    Ok(())
}

fn handle_year(entry: fs::DirEntry, content: &mut Content) -> IoResult<()> {
    let file_name = entry.file_name();
    let year = file_name.to_string_lossy();

    if year.parse::<u16>().is_err() {
        return Ok(());
    }

    let mut path = entry.path();

    path.push("Cargo.toml");

    if !path.exists() {
        return Ok(());
    }

    path.pop();
    path.push("src/lib.rs");

    if !path.exists() {
        return Ok(());
    }

    path.pop();

    let _ = writeln!(content.solve_fn, "        {year} => match day {{");
    let _ = write!(content.solved_days, "        {year} => ");
    let _ = write!(content.years, "{year}, ");

    handle_days(year.as_ref(), &path, content)?;

    let _ = writeln!(
        content.solve_fn,
        "            _ => |_| eyre::bail!(\"invalid day\"),
        }},"
    );

    Ok(())
}

fn handle_days(year: &str, path: impl AsRef<Path>, content: &mut Content) -> IoResult<()> {
    let Some(year) = year.get(year.len().saturating_sub(2)..) else {
        return Ok(());
    };

    let mut days_bits = 0;

    for res in fs::read_dir(path)? {
        let entry = res?;

        let file_name = entry.file_name();
        let day_file = file_name.to_string_lossy();

        let Some(day) = day(&day_file).filter(|&day| day > 0) else {
            continue;
        };

        days_bits |= 1 << (day - 1);

        let _ = writeln!(
            content.solve_fn,
            "            {day} => aoc{year:02}::day{day:02}::run,"
        );
    }

    let _ = writeln!(content.solved_days, "0b{days_bits:b},");

    Ok(())
}

fn day(file_name: &str) -> Option<u8> {
    file_name
        .strip_prefix("day")?
        .trim_end_matches(".rs")
        .parse::<u8>()
        .ok()
}

struct Content {
    /// `fn solve_fn(year: u16, day: u8) -> fn(&str) -> eyre::Result<aoc_rust::Solution>`
    solve_fn: String,
    /// `fn solved_days(year: u16) -> u32`
    solved_days: String,
    /// `fn years() -> &'static [u16]`
    years: String,
}

impl Content {
    fn new() -> Self {
        let solve_fn = format!(
            "pub fn solve_fn(year: u16, day: u8) -> fn(&str) -> eyre::Result<aoc_rust::Solution> {{
    match year {{\n"
        );

        let solved_days = format!(
            "pub fn solved_days(year: u16) -> u32 {{
    match year {{\n"
        );

        let years = format!(
            "pub fn years() -> &'static [u16] {{
    &["
        );

        Self {
            solve_fn,
            solved_days,
            years,
        }
    }

    fn finish(&mut self) {
        let _ = writeln!(
            self.solve_fn,
            "
    }}
}}"
        );

        let _ = writeln!(
            self.solved_days,
            "
    }}
}}"
        );

        let _ = writeln!(
            self.years,
            "]
}}"
        );
    }

    fn write(&self, dest: &mut fs::File) -> IoResult<()> {
        dest.write_all(self.solve_fn.as_bytes())?;
        dest.write_all(self.solved_days.as_bytes())?;
        dest.write_all(self.years.as_bytes())?;

        Ok(())
    }
}
