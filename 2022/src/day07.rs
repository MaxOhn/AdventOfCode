use std::{
    cell::RefCell,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::Add,
    rc::{Rc, Weak},
    str::{FromStr, Lines},
};

use crate::prelude::*;

pub fn run(input: &str) -> Result<Solution> {
    let fs = FileSystem::parse(input)?;
    // println!("{fs}");

    let map = |entry: &DirEntry| match entry {
        DirEntry::Dir(dir) => {
            let size = dir.borrow().size();

            if size <= 100_000 {
                size
            } else {
                0
            }
        }
        DirEntry::File(_) => 0,
    };

    let reduce = <usize as Add>::add;

    let p1 = fs.map_reduce(map, reduce);

    const TOTAL_MEM: usize = 70_000_000;
    const REQUIRED_MEM: usize = 30_000_000;
    const FREE_MEM: usize = TOTAL_MEM - REQUIRED_MEM;
    let to_delete = fs.size() - FREE_MEM;

    let p2 = fs.dir_min_by_key(|dir| {
        let size = dir.size();

        if size < to_delete {
            usize::MAX
        } else {
            size
        }
    });

    Ok(Solution::new().part1(p1).part2(p2))
}

struct FileSystem<'i> {
    root: Dir<'i>,
}

impl<'i> FileSystem<'i> {
    fn parse(input: &'i str) -> Result<Self> {
        let mut lines = input.lines();

        let this = Self::default();
        let mut dir = Rc::clone(&this.root);

        while let Some(next) = read_dir(dir, &mut lines)? {
            dir = next;
        }

        Ok(this)
    }

    fn size(&self) -> usize {
        self.root.borrow().size()
    }

    fn map_reduce<M, O, F>(&self, map: M, fold: F) -> O
    where
        M: Copy + Fn(&DirEntry) -> O,
        F: Copy + Fn(O, O) -> O,
    {
        DirEntry::Dir(Rc::clone(&self.root)).map_reduce(map, fold)
    }

    fn dir_min_by_key<F, V>(&self, f: F) -> V
    where
        F: Copy + Fn(&Directory) -> V,
        V: Ord,
    {
        self.root.borrow().min_by_key(f)
    }
}

impl Display for FileSystem<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.root.borrow().print(f, 0)
    }
}

impl Default for FileSystem<'_> {
    fn default() -> Self {
        Self {
            root: Rc::new(RefCell::new(Directory::new("/"))),
        }
    }
}

fn read_dir<'i>(curr: Dir<'i>, lines: &mut Lines<'i>) -> Result<Option<Dir<'i>>> {
    match lines.next().map(str::parse) {
        Some(Ok(Command::Cd(cmd))) => process_cd(cmd, curr),
        Some(Ok(Command::Ls)) => {
            for line in &mut *lines {
                if let Ok(entry) = DirEntry::parse(line) {
                    if let DirEntry::Dir(ref dir) = entry {
                        dir.borrow_mut().parent = Some(Rc::downgrade(&curr));
                    }

                    curr.borrow_mut().entries.push(entry);
                } else if let Ok(cmd) = line.parse() {
                    match cmd {
                        Command::Cd(cmd) => return process_cd(cmd, curr),
                        Command::Ls => {}
                    }
                } else {
                    bail!("line `{line}` is neither dir entry nor command");
                }
            }

            Ok(None)
        }
        Some(Err(err)) => Err(err),
        None => Ok(None),
    }
}

fn process_cd(cmd: CdCommand, curr: Dir) -> Result<Option<Dir>> {
    match cmd {
        CdCommand::Root => Ok(Some(Directory::root(curr))),
        CdCommand::StepOut => Ok(curr.borrow().parent.as_ref().and_then(Weak::upgrade)),
        CdCommand::StepInto { dir } => {
            let entry = curr
                .borrow()
                .entries
                .iter()
                .find_map(|entry| match entry {
                    DirEntry::Dir(dir_entry) => {
                        (dir_entry.borrow().name == dir).then_some(dir_entry)
                    }
                    DirEntry::File(_) => None,
                })
                .map(Rc::clone);

            match entry {
                Some(entry) => Ok(Some(entry)),
                None => bail!(
                    "directory `{parent}` contains no directory `{child}`",
                    parent = curr.borrow().name,
                    child = dir,
                ),
            }
        }
    }
}

enum Command {
    Cd(CdCommand),
    Ls,
}

impl FromStr for Command {
    type Err = Report;

    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        let cmd = cmd.strip_prefix("$ ").wrap_err("missing command prefix")?;

        if cmd.starts_with("ls") {
            Ok(Self::Ls)
        } else if let Some(suffix) = cmd.strip_prefix("cd ") {
            suffix.parse().map(Self::Cd)
        } else {
            bail!("invalid command `{cmd}`")
        }
    }
}

enum CdCommand {
    Root,
    StepOut,
    StepInto { dir: String },
}

impl FromStr for CdCommand {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".." => Ok(Self::StepOut),
            "/" => Ok(Self::Root),
            _ => Ok(Self::StepInto { dir: s.to_owned() }),
        }
    }
}

enum DirEntry<'i> {
    Dir(Dir<'i>),
    File(File<'i>),
}

impl<'i> DirEntry<'i> {
    fn parse(s: &'i str) -> Result<Self> {
        let (prefix, suffix) = s.split_once(' ').wrap_err("invalid dir entry")?;

        if prefix == "dir" {
            Ok(Self::Dir(Rc::new(RefCell::new(Directory::new(suffix)))))
        } else if let Ok(size) = prefix.parse() {
            Ok(Self::File(File { name: suffix, size }))
        } else {
            bail!("invalid pefix `{prefix}` for dir entry");
        }
    }

    fn size(&self) -> usize {
        match self {
            DirEntry::Dir(dir) => dir.borrow().size(),
            DirEntry::File(file) => file.size,
        }
    }

    fn map_reduce<M, O, F>(&self, map: M, fold: F) -> O
    where
        M: Copy + Fn(&Self) -> O,
        F: Copy + Fn(O, O) -> O,
    {
        match self {
            DirEntry::Dir(dir) => dir
                .borrow()
                .entries
                .iter()
                .map(|entry| entry.map_reduce(map, fold))
                .fold(map(self), fold),
            DirEntry::File(_) => map(self),
        }
    }
}

type Dir<'i> = Rc<RefCell<Directory<'i>>>;

struct Directory<'n> {
    name: &'n str,
    entries: Vec<DirEntry<'n>>,
    parent: Option<Weak<RefCell<Directory<'n>>>>,
}

impl<'n> Directory<'n> {
    fn new(name: &'n str) -> Self {
        Self {
            name,
            entries: Vec::new(),
            parent: None,
        }
    }

    fn root(curr: Dir) -> Dir {
        if let Some(parent) = curr.borrow().parent.as_ref().and_then(Weak::upgrade) {
            return parent;
        }

        curr
    }

    fn size(&self) -> usize {
        self.entries.iter().map(DirEntry::size).sum()
    }

    fn print(&self, f: &mut Formatter<'_>, mut indent: usize) -> FmtResult {
        writeln!(
            f,
            "{space}{dash:>indent$} {name} (dir)",
            space = if indent == 0 { "" } else { " " },
            dash = "-",
            name = self.name
        )?;

        indent += 2;

        for entry in self.entries.iter() {
            match entry {
                DirEntry::Dir(next) => next.borrow().print(f, indent)?,
                DirEntry::File(file) => writeln!(
                    f,
                    " {dash:>indent$} {name} (file, size={size})",
                    dash = "-",
                    name = file.name,
                    size = file.size,
                )?,
            }
        }

        Ok(())
    }

    fn min_by_key<F, V>(&self, f: F) -> V
    where
        F: Copy + Fn(&Self) -> V,
        V: Ord,
    {
        let entries_min = self
            .entries
            .iter()
            .filter_map(|entry| match entry {
                DirEntry::Dir(ref dir) => Some(dir.borrow().min_by_key(f)),
                DirEntry::File(_) => None,
            })
            .min();

        match entries_min {
            Some(min) => f(self).min(min),
            None => f(self),
        }
    }
}

struct File<'n> {
    name: &'n str,
    size: usize,
}
