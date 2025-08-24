# 🌸 fuck-shit-code - Legacy Mess Detector

English | [简体中文](./README.md)

## 📖 Introduction

`fuck-shit-code` is a ruthless tool for digging up code disasters, exposing the ugly truth, and roasting your code with savage humor. Find out just how bad your code really is!

This is a Rust rewrite of the original Go implementation `fuck-u-code`, maintaining complete feature parity while providing better performance and memory safety.

## ✨ Features

- 🔍 **Multi-language Support** - Supports Rust, Go, JavaScript, TypeScript, Python, Java, C/C++, C#, **PHP, HTML, CSS** and more
- 📊 **7 Quality Metrics** - Cyclomatic complexity, state management, comment ratio, error handling, naming convention, code duplication, code structure
- 🌏 **Internationalization** - Supports Chinese and English interfaces
- 📈 **Detailed Reports** - Provides colorful console output and Markdown format reports
- 🚀 **High Performance** - Uses Rust parallel processing for fast analysis of large codebases
- 🎯 **Smart Exclusion** - Automatically excludes node_modules, target, vendor and other dependency directories
- 📱 **Progress Display** - Real-time analysis progress display
- 🌱 **Empty Project Support** - **Supports detection of empty project folders with no code files**
- 🎨 **Frontend File Support** - **Complete support for HTML/CSS/JS file quality detection**

## 🛠️ Installation

### Build from Source

Requires Rust toolchain:

```bash
# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/NORMAL-EX/fuck-shit-code.git
cd fuck-shit-code

# Build release version
cargo build --release

# Install to system
cargo install --path .
```

### Install with Cargo

```bash
cargo install fuck-shit-code
```

## 📋 Usage

### Basic Usage

```bash
# Analyze current directory
fuck-shit-code .

# Analyze specific directory
fuck-shit-code /path/to/your/project

# Analyze single file
fuck-shit-code src/main.rs

# Analyze empty project folder (gives analysis report even for no code files)
fuck-shit-code ./empty-project

# Analyze frontend project
fuck-shit-code ./my-website
```

### Supported File Types

#### Backend Languages
- **Rust** (.rs)
- **Go** (.go)
- **Python** (.py, .pyw)
- **Java** (.java)
- **C/C++** (.c, .h, .cpp, .cc, .cxx, .hpp, .h++)
- **C#** (.cs, .razor)
- **PHP** (.php, .php3, .php4, .php5, .php7, .php8, .phtml)

#### Frontend Languages
- **JavaScript** (.js, .mjs, .cjs)
- **TypeScript** (.ts, .tsx, .jsx)
- **HTML** (.html, .htm, .xhtml)
- **CSS** (.css, .scss, .sass, .less)

### Command Line Options

```bash
fuck-shit-code [OPTIONS] [PATH]

OPTIONS:
  -l, --lang <LANG>        Specify output language (zh-CN, en-US) [default: zh-CN]
  -v, --verbose            Show detailed analysis report
  -t, --top <NUM>          Number of files with most issues to show [default: 5]
  -i, --issues <NUM>       Number of issues to show per file [default: 5]
  -s, --summary            Show only conclusion, skip the process
  -m, --markdown           Output streamlined Markdown format report
  -e, --exclude <PATTERN>  Exclude file/directory patterns (can be used multiple times)
  -x, --skipindex          Skip all index.js/index.ts files
  -h, --help               Show help information
  -V, --version            Show version information
```

### Examples

```bash
# Show detailed report
fuck-shit-code -v ./src

# Output English report
fuck-shit-code -l en-US ./src

# Generate Markdown report
fuck-shit-code -m ./src > report.md

# Exclude test files
fuck-shit-code -e "**/*_test.rs" -e "**/tests/**" ./src

# Show top 10 problematic files
fuck-shit-code -t 10 ./src

# Show summary only
fuck-shit-code -s ./src

# Analyze PHP project
fuck-shit-code ./my-php-project

# Analyze frontend project (including HTML/CSS/JS)
fuck-shit-code ./my-website

# Analyze full-stack project
fuck-shit-code ./full-stack-project
```

## 📊 Quality Metrics

### 1. Cyclomatic Complexity (Weight: 30%)
Measures the control flow complexity of functions. Higher complexity means harder to understand and test.
- **All Languages**: Accurately detects if/else/for/while/switch control structures
- **PHP Features**: Supports foreach, elseif and other PHP-specific syntax
- **Frontend Features**: Detects DOM manipulation and event handling complexity

### 2. State Management (Weight: 20%)
Detects how you manage state variables. Good state management improves maintainability and predictability.
- **Backend Languages**: Detects global variables, static variable usage
- **Frontend Languages**: Detects global state, DOM state management

### 3. Comment Ratio (Weight: 15%)
Checks if your code has enough comments. Good comments improve readability and maintainability.
- **Multiple Comment Formats**: Supports //, /* */, #, <!-- --> etc.
- **Documentation Comments**: Supports PHPDoc, JSDoc and other documentation comment detection

### 4. Code Duplication (Weight: 15%)
Evaluates the proportion of duplicate logic. More duplication means more need for abstraction and refactoring.
- **Smart Detection**: Not only detects literal duplication, but also structurally similar code
- **Cross-file Detection**: Detects duplicate patterns within projects

### 5. Code Structure (Weight: 15%)
Detects nesting depth and reference complexity. Evaluates structural clarity.
- **HTML Structure**: Detects DOM nesting depth, form complexity
- **CSS Structure**: Detects selector complexity, rule nesting

### 6. Error Handling (Weight: 10%)
Sniffs out your error handling. Good error handling improves code robustness.
- **Language Specific**: Supports error handling mechanisms for each language
- **Frontend Features**: Detects try-catch, Promise error handling

### 7. Naming Convention (Weight: 8%)
Checks if your naming is civilized. Good names mean less guessing, more coding.
- **Multi-language Standards**: Supports naming conventions for each language
- **Frontend Standards**: Detects CSS class names, HTML ID conventions

## 🏆 Quality Levels

| Score Range | Level | Description |
|-------------|-------|-------------|
| 0-5 | 🌱 Fresh as spring breeze | Code so clean, it's a joy to read |
| 5-15 | 🌸 A whiff of trouble | Mostly fine, but a little stinky |
| 15-25 | 😐 Slightly stinky youth | A faint whiff, open a window |
| 25-40 | 😷 Code reeks, mask up | Code is starting to stink, approach with caution |
| 40-55 | 💩 Medium legacy mess | Obvious code odor, even fresh air can't save it |
| 55-65 | 🤕 Hidden toxic tumor | Fun to write, but you'll cry when you have to fix it |
| 65-75 | ☣️ Severe legacy mess | Toxic fumes everywhere, gas mask recommended |
| 75-85 | 🧟 Code graveyard | No programmer enters and leaves alive |
| 85-95 | ☢️ Nuclear disaster zone | A crime against humanity, best to incinerate it |
| 95-100 | 🪦 Generational legacy mess | Built by generations, impossible to maintain |
| 100 | 👑💩 Ultimate King of Mess | So wild your own mother would disown you |

## 🎯 Default Excluded Files/Directories

The tool automatically excludes common dependency and build directories:

### Frontend Projects
- `**/node_modules/**` - Node.js dependencies
- `**/dist/**`, `**/build/**` - Frontend build output
- `**/.next/**`, `**/.nuxt/**` - Framework build directories
- `**/*.min.js`, `**/*.min.css` - Minified files
- `**/*.bundle.js`, `**/*.chunk.js` - Bundled files

### Backend Projects
- `**/target/**` - Rust/Java build directory
- `**/vendor/**` - Go/PHP dependencies
- `**/__pycache__/**`, `**/venv/**` - Python related
- `**/composer.lock`, `**/Cargo.lock` - Dependency lock files

### General Exclusions
- `**/.git/**`, `**/.vscode/**`, `**/.idea/**` - Version control and IDE config
- `**/*_test.*`, `**/test/**`, `**/tests/**` - Test files
- `**/logs/**`, `**/tmp/**`, `**/temp/**` - Temporary files

## 📝 Output Examples

### Empty Project Detection
```
🔍 Start analyzing: ./empty-project

  ╔════════════════════════════════════════╗
  ║       🏜️  Wasteland Detector  🏜️      ║
  ╚════════════════════════════════════════╝

  😅 Detected a wasteland...
  📭 It's empty here, not even a single line of code!

  Suggestions:
  1. 🎯 Go write some code, or I have nothing to roast!
  2. 💡 Or check if the path is correct?
  3. 🤔 Maybe the exclusion rules are too strict?

  💭 Remember: Empty projects are the cleanest, but also the most useless!
```

### Frontend Project Detection
```
🔍 Start analyzing: ./my-website
📂 Files found: 28 (including HTML: 5, CSS: 8, JS: 12, PHP: 3)

  🌸 Code Quality Analysis Report 🌸
──────────────────────────────────────────

  Overall Score: 42.31 / 100 - Code is starting to stink, approach with caution
  Quality Level: Medium legacy mess - Obvious code odor, even fresh air can't save it

◆ Metrics Details

  ✓  Cyclomatic Complexity  18.45 pts   Structure mostly clear, occasional tangles
  •  State Management       35.20 pts   DOM state management needs improvement
  ⚠  Comment Ratio          58.73 pts   Frontend code lacks sufficient comments
  !  Code Duplication       61.22 pts   CSS selectors have duplicate patterns
  !! HTML Structure         72.18 pts   Nesting too deep, needs optimization
  !! CSS Complexity         68.91 pts   Selectors overly complex

◆ Problem Files Ranking

  1. styles/main.css (Issue Score: 78.45)
     CSS selector complexity too high, nesting reaches 8 levels
     Found 19 duplicate style rules
     
  2. js/app.js (Issue Score: 71.23)
     Function 'handleUserInteraction' has very high complexity (23)
     Excessive global variable usage, chaotic state management
     
  3. index.html (Issue Score: 65.78)
     HTML structure nested too deeply, form complexity too high
     Missing semantic tags, accessibility needs improvement
```

### PHP Project Detection
```
🔍 Start analyzing: ./my-php-app
📂 Files found: 34 (including PHP: 28, HTML: 4, CSS: 2)

  🌸 Code Quality Analysis Report 🌸
──────────────────────────────────────────

  Overall Score: 38.92 / 100 - A faint whiff, open a window
  Quality Level: Code reeks - Code is starting to stink, approach with caution

◆ Metrics Details

  ✓  Cyclomatic Complexity  22.15 pts   PHP control structures reasonably sound
  ✓  State Management       26.83 pts   Class design relatively reasonable
  •  Comment Ratio          45.67 pts   Missing PHPDoc documentation comments
  ⚠  Error Handling         52.34 pts   Exception handling needs improvement
  !  Code Duplication       58.91 pts   Found multiple similar CRUD methods
  !  Naming Convention      61.45 pts   Some variable naming non-standard

◆ Problem Files Ranking

  1. src/Controllers/UserController.php (Issue Score: 69.23)
     Method 'handleUserRegistration' too long (156 lines)
     Missing proper exception handling mechanisms
     
  2. src/Models/User.php (Issue Score: 58.47)
     Class complexity high, recommend splitting responsibilities
     Missing documentation comments
```

## 🤝 Contributing

Issues and Pull Requests are welcome!

### Development Guide

1. Fork this repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

MIT License - See [LICENSE](LICENSE) file for details

## 🙏 Acknowledgments

- Original Go implementation: [fuck-u-code](https://github.com/Done-0/fuck-u-code)
- Thanks to all contributors and users

## ⚠️ Disclaimer

The evaluation results of this tool are for reference only, aimed at helping developers find potential issues in their code in a humorous way. Please treat the scores rationally and continuously improve code quality.

Remember: **There is no perfect code, only evolving code**.

---

### 🆕 New Feature Highlights

- ✨ **Full PHP Support**: Complete PHP syntax analysis including classes, methods, namespaces
- 🎨 **Frontend File Detection**: HTML structure analysis, CSS selector complexity, JavaScript DOM operation detection  
- 🌱 **Empty Project Detection**: Even empty folders get interesting analysis reports
- 📊 **Smarter Metrics**: Quality assessment algorithms optimized for different language characteristics
- 🔍 **More Comprehensive Coverage**: Supports more file extensions and programming paradigms


Now you can analyze complete projects from frontend to backend with one tool!
