use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub source_ext: String,
    pub run_cmd: String,
    pub build_cmd: String,
    pub check_cmd: String,
    pub project_dir: String,
    pub clean_cmd: String,
}

impl Project {
    fn new(name: &str, source_ext: &str, run: &str, build: &str, check: &str, clean: &str) -> Self {
        Self {
            name: name.into(),
            source_ext: source_ext.into(),
            run_cmd: run.into(),
            build_cmd: build.into(),
            check_cmd: check.into(),
            project_dir: ".".into(),
            clean_cmd: clean.into(),
        }
    }
}

/// Key = evidence file name, or "*.ext" for extension-based evidence.
/// An empty string means the command doesn't apply to that project type.
pub fn project_evidence() -> HashMap<&'static str, Project> {
    let mut m = HashMap::new();

    // Rust / Go
    m.insert("Cargo.toml", Project::new("Rust (Cargo)", "rs", "cargo run", "cargo build", "cargo check", "cargo clean"));
    m.insert("go.mod", Project::new("Go", "go", "go run .", "go build ./...", "go vet ./...", "go clean"));

    // JavaScript / TypeScript / Deno
    m.insert("package.json", Project::new("Node.js (npm)", "js", "node index.js", "npm run build", "npm test", "rm -rf node_modules dist"));
    m.insert("tsconfig.json", Project::new("TypeScript", "ts", "npx ts-node src/index.ts", "npx tsc", "npx tsc --noEmit", "rm -rf dist"));
    m.insert("deno.json", Project::new("Deno", "ts", "deno run main.ts", "deno compile main.ts", "deno check main.ts", "rm -f main"));

    // Python
    m.insert("requirements.txt", Project::new("Python (pip)", "py", "python main.py", "pip install -r requirements.txt", "python -m compileall .", "find . -name __pycache__ -exec rm -rf {} +"));
    m.insert("pyproject.toml", Project::new("Python (pyproject)", "py", "python main.py", "pip install -e .", "python -m pytest", "rm -rf build dist *.egg-info"));

    // JVM
    m.insert("pom.xml", Project::new("Java (Maven)", "java", "mvn exec:java", "mvn package", "mvn verify", "mvn clean"));
    m.insert("build.gradle", Project::new("Java (Gradle)", "java", "gradle run", "gradle build", "gradle check", "gradle clean"));
    m.insert("build.gradle.kts", Project::new("Kotlin (Gradle)", "kt", "gradle run", "gradle build", "gradle check", "gradle clean"));

    // .NET
    m.insert("*.csproj", Project::new("C# (.NET)", "cs", "dotnet run", "dotnet build", "dotnet format --verify-no-changes", "dotnet clean"));

    // C / C++
    m.insert("CMakeLists.txt", Project::new("C/C++ (CMake)", "cpp", "./build/app", "cmake -B build && cmake --build build", "ctest --test-dir build", "rm -rf build"));
    m.insert("Makefile", Project::new("Make", "c", "make run", "make", "make test", "make clean"));
    m.insert("*.c", Project::new("C (gcc)", "c", "./main", "gcc -Wall -Wextra *.c -o main", "gcc -fsyntax-only -Wall *.c", "rm -f main"));
    m.insert("*.cpp", Project::new("C++ (g++)", "cpp", "./main", "g++ -std=c++17 -Wall *.cpp -o main", "g++ -fsyntax-only -std=c++17 *.cpp", "rm -f main"));

    // Assembly (Linux x86-64, NASM / GAS)
    m.insert("*.asm", Project::new("Assembly (NASM)", "asm", "./main", "nasm -f elf64 main.asm -o main.o && ld main.o -o main", "nasm -f elf64 -w+all main.asm -o /dev/null", "rm -f main main.o"));
    m.insert("*.s", Project::new("Assembly (GAS)", "s", "./main", "as main.s -o main.o && ld main.o -o main", "as --fatal-warnings main.s -o /dev/null", "rm -f main main.o"));

    // PHP / Ruby / PocketMine
    m.insert("composer.json", Project::new("PHP (Composer)", "php", "php index.php", "composer install", "composer validate", "rm -rf vendor"));
    m.insert("plugin.yml", Project::new("PocketMine-MP plugin", "php", "", "composer install", "find src -name '*.php' -exec php -l {} \\;", "rm -rf vendor"));
    m.insert("Gemfile", Project::new("Ruby (Bundler)", "rb", "ruby main.rb", "bundle install", "ruby -wc main.rb", "rm -rf .bundle"));

    // Functional / other
    m.insert("mix.exs", Project::new("Elixir (Mix)", "ex", "mix run", "mix compile", "mix test", "mix clean"));
    m.insert("rebar.config", Project::new("Erlang (rebar3)", "erl", "rebar3 shell", "rebar3 compile", "rebar3 dialyzer", "rebar3 clean"));
    m.insert("stack.yaml", Project::new("Haskell (Stack)", "hs", "stack run", "stack build", "stack test", "stack clean"));
    m.insert("*.cabal", Project::new("Haskell (Cabal)", "hs", "cabal run", "cabal build", "cabal check", "cabal clean"));
    m.insert("pubspec.yaml", Project::new("Dart", "dart", "dart run", "dart compile exe bin/main.dart", "dart analyze", "dart clean"));
    m.insert("Package.swift", Project::new("Swift (SwiftPM)", "swift", "swift run", "swift build", "swift test", "swift package clean"));
    m.insert("build.zig", Project::new("Zig", "zig", "zig build run", "zig build", "zig build test", "rm -rf zig-out .zig-cache"));
    m.insert("*.sh", Project::new("Shell script", "sh", "bash main.sh", "", "bash -n main.sh", ""));

    m
}

/// Returns every project whose evidence exists directly inside `dir`.
pub fn detect<'a>(dir: &Path, map: &'a HashMap<&'static str, Project>) -> Vec<&'a Project> {
    let entries: Vec<String> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();

    map.iter()
        .filter(|&(evidence, _)| {
            if let Some(ext) = evidence.strip_prefix("*.") {
                entries.iter().any(|f| f.ends_with(&format!(".{ext}")))
            } else {
                entries.iter().any(|f| f.as_str() == *evidence)
            }
        })
        .map(|(_, p)| p)
        .collect()
}
