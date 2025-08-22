use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static MESSAGES: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    
    // General
    m.insert("app.name".to_string(), "Legacy Mess Detector".to_string());
    m.insert("app.description".to_string(), "A ruthless tool for digging up code disasters, exposing the ugly truth, and roasting your code with savage humor. Find out just how bad your code really is!".to_string());

    // Metric names
    m.insert("metric.cyclomatic_complexity".to_string(), "Cyclomatic Complexity".to_string());
    m.insert("metric.function_length".to_string(), "State Management".to_string());
    m.insert("metric.comment_ratio".to_string(), "Comment Ratio".to_string());
    m.insert("metric.error_handling".to_string(), "Error Handling".to_string());
    m.insert("metric.naming_convention".to_string(), "Naming Convention".to_string());
    m.insert("metric.code_duplication".to_string(), "Code Duplication".to_string());
    m.insert("metric.structure_analysis".to_string(), "Code Structure".to_string());

    // Analyzer progress
    m.insert("analyzer.searching_files".to_string(), "Searching for source code files...".to_string());
    m.insert("analyzer.files_found".to_string(), "Files found".to_string());
    m.insert("analyzer.analyzing_files".to_string(), "Analyzing files...".to_string());
    m.insert("analyzer.analysis_complete".to_string(), "Analysis complete".to_string());

    // Issue categories
    m.insert("report.no_issues".to_string(), "Congratulations! No problematic files found!".to_string());
    m.insert("issue.category.complexity".to_string(), "Complexity Issues".to_string());
    m.insert("issue.category.comment".to_string(), "Comment Issues".to_string());
    m.insert("issue.category.naming".to_string(), "Naming Issues".to_string());
    m.insert("issue.category.structure".to_string(), "Structure Issues".to_string());
    m.insert("issue.category.duplication".to_string(), "Duplication Issues".to_string());
    m.insert("issue.category.error".to_string(), "Error Handling Issues".to_string());
    m.insert("issue.category.other".to_string(), "Other Issues".to_string());

    // Quality levels
    m.insert("level.clean".to_string(), "Fresh as spring breeze".to_string());
    m.insert("level.mild".to_string(), "A whiff of trouble".to_string());
    m.insert("level.moderate".to_string(), "Slightly stinky youth".to_string());
    m.insert("level.bad".to_string(), "Code reeks, mask up".to_string());
    m.insert("level.terrible".to_string(), "Medium legacy mess".to_string());
    m.insert("level.disaster".to_string(), "Hidden toxic tumor".to_string());
    m.insert("level.disaster.severe".to_string(), "Severe legacy mess".to_string());
    m.insert("level.disaster.very_bad".to_string(), "Code graveyard, no one survives".to_string());
    m.insert("level.disaster.extreme".to_string(), "Nuclear disaster zone".to_string());
    m.insert("level.disaster.worst".to_string(), "Generational legacy mess".to_string());
    m.insert("level.disaster.ultimate".to_string(), "Ultimate King of Mess".to_string());

    // Command line
    m.insert("cmd.short".to_string(), "💻 fuck-shit-code".to_string());
    m.insert("cmd.long".to_string(), "🔍 Code Quality Detector - Objectively assess your code quality\n\nIt can analyze code quality, output scores, and help you find 💩 in your code. Suitable for:\n- Quality assessment before project refactoring\n- Team code review assistance tool\n- Learning programming best practices".to_string());
    m.insert("cmd.start_analyzing".to_string(), "Start analyzing: %s".to_string());
    m.insert("cmd.exclude_patterns".to_string(), "Excluding the following file/directory patterns:".to_string());
    m.insert("cmd.analysis_failed".to_string(), "Analysis failed: %s".to_string());

    // Report
    m.insert("report.title".to_string(), "Code Quality Analysis Report".to_string());
    m.insert("report.overall_score".to_string(), "Overall Score: %.2f / 100".to_string());
    m.insert("report.level".to_string(), "Quality Level: %s".to_string());
    m.insert("report.metrics_details".to_string(), "Metrics Details".to_string());
    m.insert("report.worst_files".to_string(), "Problem Files Ranking".to_string());
    m.insert("report.conclusion".to_string(), "Conclusion".to_string());
    m.insert("report.file_score".to_string(), "Issue Score: %.2f".to_string());
    m.insert("report.more_issues".to_string(), "...and %d more issues".to_string());
    m.insert("report.score_calc".to_string(), "Score Calculation: ".to_string());
    m.insert("report.overall_assessment".to_string(), "Overall Assessment".to_string());
    m.insert("report.quality_score".to_string(), "Quality Score".to_string());
    m.insert("report.quality_level".to_string(), "Quality Level".to_string());
    m.insert("report.analyzed_files".to_string(), "Analyzed Files".to_string());
    m.insert("report.total_lines".to_string(), "Total Lines".to_string());
    m.insert("report.quality_metrics".to_string(), "Quality Metrics".to_string());
    m.insert("report.metric".to_string(), "Metric".to_string());
    m.insert("report.score".to_string(), "Score".to_string());
    m.insert("report.weight".to_string(), "Weight".to_string());
    m.insert("report.status".to_string(), "Status".to_string());
    m.insert("report.problem_files".to_string(), "Problem Files".to_string());
    m.insert("report.issue_categories".to_string(), "Issue Categories".to_string());
    m.insert("report.main_issues".to_string(), "Main Issues".to_string());
    m.insert("report.and".to_string(), "and".to_string());
    m.insert("report.more_issues_short".to_string(), "more issues".to_string());
    m.insert("report.improvement_suggestions".to_string(), "Improvement Suggestions".to_string());

    // Metric score suffix
    m.insert("metric.score.suffix".to_string(), " pts".to_string());

    // Cyclomatic complexity comments
    m.insert("metric.complexity.good".to_string(), "Clear structure, no unnecessary complexity, great!".to_string());
    m.insert("metric.complexity.medium".to_string(), "Winding logic, like a maze for your brain".to_string());
    m.insert("metric.complexity.bad".to_string(), "Functions like labyrinths, maintenance like a dungeon raid".to_string());

    // Function length comments
    m.insert("metric.length.good".to_string(), "Clear state management, reasonable variable scope, predictable state".to_string());
    m.insert("metric.length.medium".to_string(), "Average state management, some global state or unclear state changes".to_string());
    m.insert("metric.length.bad".to_string(), "Chaotic state management, excessive use of global variables, difficult to track state changes".to_string());

    // Comment ratio comments
    m.insert("metric.comment.good".to_string(), "Good comments, they'll help you survive".to_string());
    m.insert("metric.comment.medium".to_string(), "Sparse comments, readers need imagination".to_string());
    m.insert("metric.comment.bad".to_string(), "No comments, understanding depends on luck".to_string());

    // Error handling comments
    m.insert("metric.error.good".to_string(), "Errors are handled with care, code shows compassion".to_string());
    m.insert("metric.error.medium".to_string(), "Error handling exists, but barely helps".to_string());
    m.insert("metric.error.bad".to_string(), "Errors ignored? Just like life's problems".to_string());

    // Naming convention comments
    m.insert("metric.naming.good".to_string(), "Clear naming, the light of programmer civilization".to_string());
    m.insert("metric.naming.medium".to_string(), "Naming is okay, some guesswork needed".to_string());
    m.insert("metric.naming.bad".to_string(), "Variable names look like keyboard smashes: x, y, z, tmp, xxx".to_string());

    // Code duplication comments
    m.insert("metric.duplication.good".to_string(), "Proper abstraction, satisfying for the OCD programmer".to_string());
    m.insert("metric.duplication.medium".to_string(), "Some repetition, abstraction wouldn't hurt".to_string());
    m.insert("metric.duplication.bad".to_string(), "Copy-paste evidence everywhere, Ctrl+C/V medal earned".to_string());

    // Code structure comments
    m.insert("metric.structure.good".to_string(), "Beautiful structure, easy to follow".to_string());
    m.insert("metric.structure.medium".to_string(), "Structure is okay, but somewhat confusing".to_string());
    m.insert("metric.structure.bad".to_string(), "Nested like Russian dolls, dizzying to read".to_string());

    // Quality advice
    m.insert("advice.good".to_string(), "👍 Keep going, you're the clean freak of the coding world, a true code hygiene champion".to_string());
    m.insert("advice.moderate".to_string(), "🔧 Suggestion: This code is like a rebellious teenager, needs some tough love to become useful".to_string());
    m.insert("advice.bad".to_string(), "🧨 Suggestion: Delete the repo and run, or seal it for the next generation to suffer".to_string());

    // Improvement priority
    m.insert("advice.priority.high".to_string(), "High Priority".to_string());
    m.insert("advice.priority.medium".to_string(), "Medium Priority".to_string());
    m.insert("advice.priority.low".to_string(), "Low Priority".to_string());

    // Good code advice
    m.insert("advice.good.maintain".to_string(), "Keep up the clean code standards, don't let the mess creep in".to_string());
    m.insert("advice.good.optimize".to_string(), "Go further—optimize for performance and readability, just because you can".to_string());
    m.insert("advice.good.document".to_string(), "Polish your docs and comments, make your team love you even more".to_string());

    // Moderate code advice
    m.insert("advice.moderate.refactor".to_string(), "Refactor those spaghetti functions and modules before they strangle you".to_string());
    m.insert("advice.moderate.complexity".to_string(), "Cut down the cyclomatic complexity, make your code less of a maze".to_string());
    m.insert("advice.moderate.naming".to_string(), "Give variables and functions real names, not cryptic nonsense".to_string());
    m.insert("advice.moderate.comments".to_string(), "Add more comments, unless you want future you to suffer".to_string());
    m.insert("advice.moderate.duplication".to_string(), "Wipe out duplicate code, extract common stuff, stop the Ctrl+C/V madness".to_string());
    m.insert("advice.moderate.structure".to_string(), "Untangle the nesting, make the structure readable for humans".to_string());
    m.insert("advice.moderate.style".to_string(), "Unify your code style, don't let formatting chaos reign".to_string());

    // Bad code advice
    m.insert("advice.bad.urgent_refactor".to_string(), "Emergency! Refactor those monster functions, one job per function please".to_string());
    m.insert("advice.bad.complexity".to_string(), "Slash the cyclomatic complexity, break up the logic before it breaks you".to_string());
    m.insert("advice.bad.error_handling".to_string(), "Add real error handling, not just wishful thinking".to_string());
    m.insert("advice.bad.naming".to_string(), "Fix all the names, no more x, y, z, tmp, or xxx".to_string());
    m.insert("advice.bad.duplication".to_string(), "Exterminate duplicate code, build a real reuse system".to_string());
    m.insert("advice.bad.comments".to_string(), "Flood the code with comments, make it readable for mortals".to_string());
    m.insert("advice.bad.structure".to_string(), "Redesign the architecture, save the project from itself".to_string());
    m.insert("advice.bad.style".to_string(), "Set up strict coding standards and actually follow them".to_string());

    // Metric descriptions
    m.insert("metric.function_length.description".to_string(), "Detects how you manage state variables. Good state management means you won't lose your mind maintaining this code.".to_string());
    m.insert("metric.comment_ratio.description".to_string(), "Checks if your code has enough comments. Good comments mean you won't curse your past self.".to_string());
    m.insert("metric.error_handling.description".to_string(), "Sniffs out your error handling. Good error handling means your code won't explode at runtime.".to_string());
    m.insert("metric.naming_convention.description".to_string(), "Checks if your naming is civilized. Good names mean less guessing, more coding.".to_string());
    m.insert("metric.code_duplication.description".to_string(), "Evaluates how much copy-paste you did. More duplication means you need to refactor, or just admit you love Ctrl+C/V.".to_string());
    m.insert("metric.structure_analysis.description".to_string(), "Detects nesting depth and reference complexity. The less Russian doll, the less headache.".to_string());
    m.insert("metric.cyclomatic_complexity.description".to_string(), "Measures how twisted your control flow is. The higher the complexity, the more likely you'll regret touching this code.".to_string());

    // Quality level descriptions
    m.insert("level.clean.description".to_string(), "Code so clean, it's a joy to read—like a spa day for your eyes.".to_string());
    m.insert("level.mild.description".to_string(), "Mostly fine, but a little stinky. Air it out and you'll survive.".to_string());
    m.insert("level.moderate.description".to_string(), "A faint whiff, open a window and hope for the best.".to_string());
    m.insert("level.bad.description".to_string(), "Code is starting to stink, approach with caution and a mask.".to_string());
    m.insert("level.terrible.description".to_string(), "Obvious code odor, even fresh air can't save it.".to_string());
    m.insert("level.disaster.description".to_string(), "Fun to write, but you'll cry when you have to fix it.".to_string());
    m.insert("level.disaster.severe.description".to_string(), "Toxic fumes everywhere, gas mask recommended.".to_string());
    m.insert("level.disaster.very_bad.description".to_string(), "No programmer enters and leaves alive—abandon hope.".to_string());
    m.insert("level.disaster.extreme.description".to_string(), "A crime against humanity, best to incinerate it.".to_string());
    m.insert("level.disaster.worst.description".to_string(), "Legacy mess, built by generations, impossible to maintain.".to_string());
    m.insert("level.disaster.ultimate.description".to_string(), "So wild your own mother would disown you for writing it.".to_string());

    // Overall score comments
    m.insert("score.comment.0".to_string(), "Like a spring breeze, kissed by angels—code so clean it heals your soul.".to_string());
    m.insert("score.comment.10".to_string(), "Fresh and pleasant, like morning dew—almost makes you want to refactor for fun.".to_string());
    m.insert("score.comment.20".to_string(), "A hint of fragrance, sometimes a whiff of funk—still safe to touch.".to_string());
    m.insert("score.comment.30".to_string(), "A bit smelly, but not lethal—just hold your nose and keep going.".to_string());
    m.insert("score.comment.40".to_string(), "Stench hits you, mask recommended—read at your own risk.".to_string());
    m.insert("score.comment.50".to_string(), "Toxic fumes everywhere, code review is torture—bring snacks and tissues.".to_string());
    m.insert("score.comment.60".to_string(), "Stench fills the air, maintainers coughing blood—pray for mercy.".to_string());
    m.insert("score.comment.70".to_string(), "Biohazard zone, write your will before taking over—may luck be with you.".to_string());
    m.insert("score.comment.80".to_string(), "Nuclear waste site, bring a hazmat suit—every edit is a gamble.".to_string());
    m.insert("score.comment.90".to_string(), "Disaster level tumor, every glance shortens your life by ten years—run while you still can.".to_string());

    // Error messages
    m.insert("error.path_not_accessible".to_string(), "Cannot access path: %s".to_string());
    m.insert("error.file_read_failed".to_string(), "Failed to read file %s: %s".to_string());
    m.insert("error.code_parse_failed".to_string(), "Failed to parse code %s: %s".to_string());
    m.insert("error.source_files_not_found".to_string(), "Failed to find source files: %s".to_string());
    m.insert("error.file_analysis_failed".to_string(), "Failed to analyze file %s: %s".to_string());

    // Warnings and tips
    m.insert("warning.format".to_string(), "Warning: %s\n".to_string());

    // Function complexity issues
    m.insert("issue.high_complexity".to_string(), "Function %s has very high cyclomatic complexity (%d), consider refactoring".to_string());
    m.insert("issue.medium_complexity".to_string(), "Function %s has high cyclomatic complexity (%d), consider simplifying".to_string());
    m.insert("issue.file_high_complexity".to_string(), "File has very high complexity (%d), consider splitting into multiple files".to_string());
    m.insert("issue.file_medium_complexity".to_string(), "File has high complexity (%d), consider optimizing".to_string());

    // Function length issues
    m.insert("issue.function_very_long".to_string(), "Function %s has too many lines of code (%d), strongly recommend splitting".to_string());
    m.insert("issue.function_long".to_string(), "Function %s has many lines of code (%d), consider splitting into smaller functions".to_string());
    m.insert("issue.function_medium".to_string(), "Function %s has %d lines of code, consider if it can be simplified".to_string());
    m.insert("issue.file_very_long".to_string(), "File has too many lines of code (%d), recommend splitting into multiple files".to_string());
    m.insert("issue.file_long".to_string(), "File has many lines of code (%d), consider optimizing the structure".to_string());

    // Comment ratio issues
    m.insert("issue.comment_very_low".to_string(), "Code comment ratio is extremely low (%.2f%%), almost no comments".to_string());
    m.insert("issue.comment_low".to_string(), "Code comment ratio is low (%.2f%%), consider adding more comments".to_string());
    m.insert("issue.exported_func_no_comment".to_string(), "Exported function %s lacks documentation comment".to_string());
    m.insert("issue.exported_type_no_comment".to_string(), "Exported type %s lacks documentation comment".to_string());

    // Verbose report
    m.insert("verbose.basic_statistics".to_string(), "📊 Basic stats (brace yourself):".to_string());
    m.insert("verbose.total_files".to_string(), "Total files:".to_string());
    m.insert("verbose.total_lines".to_string(), "Total lines:".to_string());
    m.insert("verbose.total_issues".to_string(), "Total issues:".to_string());
    m.insert("verbose.metric_details".to_string(), "🔍 Metric details (the juicy bits):".to_string());
    m.insert("verbose.weight".to_string(), "Weight:".to_string());
    m.insert("verbose.description".to_string(), "Description:".to_string());
    m.insert("verbose.score".to_string(), "Score:".to_string());
    m.insert("verbose.all_files".to_string(), "All code files analyzed (no mercy):".to_string());
    m.insert("verbose.no_files_found".to_string(), "🎉 No files found for analysis! Your repo is either empty or blessed.".to_string());
    m.insert("verbose.file_good_quality".to_string(), "Code quality is decent, nothing too tragic—keep it up!".to_string());

    // File analysis progress
    m.insert("report.analyzing_files".to_string(), "Files analyzed".to_string());
    m.insert("report.files".to_string(), "files".to_string());

    m
});