use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static MESSAGES: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    
    // 通用
    m.insert("app.name".to_string(), "屎山代码检测器".to_string());
    m.insert("app.description".to_string(), "一个专为挖掘\"屎山代码\"设计的工具，能无情揭露代码的丑陋真相，并用毫不留情的幽默语言告诉你：你的代码到底有多烂。".to_string());

    // 指标名称
    m.insert("metric.cyclomatic_complexity".to_string(), "循环复杂度".to_string());
    m.insert("metric.function_length".to_string(), "状态管理".to_string());
    m.insert("metric.comment_ratio".to_string(), "注释覆盖率".to_string());
    m.insert("metric.error_handling".to_string(), "错误处理".to_string());
    m.insert("metric.naming_convention".to_string(), "命名规范".to_string());
    m.insert("metric.code_duplication".to_string(), "代码重复度".to_string());
    m.insert("metric.structure_analysis".to_string(), "代码结构".to_string());

    // 分析器进度
    m.insert("analyzer.searching_files".to_string(), "正在搜索源代码文件...".to_string());
    m.insert("analyzer.files_found".to_string(), "已找到文件数".to_string());
    m.insert("analyzer.analyzing_files".to_string(), "正在分析文件...".to_string());
    m.insert("analyzer.analysis_complete".to_string(), "分析完成".to_string());

    // 问题分类
    m.insert("report.no_issues".to_string(), "恭喜！没有特别多问题的文件！".to_string());
    m.insert("issue.category.complexity".to_string(), "复杂度问题".to_string());
    m.insert("issue.category.comment".to_string(), "注释问题".to_string());
    m.insert("issue.category.naming".to_string(), "命名问题".to_string());
    m.insert("issue.category.structure".to_string(), "结构问题".to_string());
    m.insert("issue.category.duplication".to_string(), "重复问题".to_string());
    m.insert("issue.category.error".to_string(), "错误处理问题".to_string());
    m.insert("issue.category.other".to_string(), "其他问题".to_string());

    // 质量等级
    m.insert("level.clean".to_string(), "清新可人".to_string());
    m.insert("level.mild".to_string(), "偶有异味".to_string());
    m.insert("level.moderate".to_string(), "微臭青年".to_string());
    m.insert("level.bad".to_string(), "屎气扑鼻".to_string());
    m.insert("level.terrible".to_string(), "中度屎山".to_string());
    m.insert("level.disaster".to_string(), "隐性毒瘤".to_string());
    m.insert("level.disaster.severe".to_string(), "重度屎山".to_string());
    m.insert("level.disaster.very_bad".to_string(), "代码化尸场".to_string());
    m.insert("level.disaster.extreme".to_string(), "核平级灾难".to_string());
    m.insert("level.disaster.worst".to_string(), "祖传老屎".to_string());
    m.insert("level.disaster.ultimate".to_string(), "终极屎王".to_string());

    // 命令行
    m.insert("cmd.short".to_string(), "💻 fuck-shit-code".to_string());
    m.insert("cmd.long".to_string(), "🔍 屎山代码检测器 - 客观评估您的代码质量\n\n它可以分析代码质量、输出评分，帮助您发现代码中的💩。适用于：\n- 项目重构前的质量评估\n- 团队代码审查辅助工具\n- 学习编程最佳实践".to_string());
    m.insert("cmd.start_analyzing".to_string(), "开始嗅探：%s".to_string());
    m.insert("cmd.exclude_patterns".to_string(), "排除以下文件/目录模式:".to_string());
    m.insert("cmd.analysis_failed".to_string(), "分析失败：%s".to_string());

    // 报告
    m.insert("report.title".to_string(), "屎山代码分析报告".to_string());
    m.insert("report.overall_score".to_string(), "总体评分: %.2f / 100".to_string());
    m.insert("report.level".to_string(), "屎山等级: %s".to_string());
    m.insert("report.metrics_details".to_string(), "评分指标详情".to_string());
    m.insert("report.worst_files".to_string(), "最屎代码排行榜".to_string());
    m.insert("report.conclusion".to_string(), "诊断结论".to_string());
    m.insert("report.file_score".to_string(), "屎气指数: %.2f".to_string());
    m.insert("report.more_issues".to_string(), "...还有 %d 个问题实在太屎，列不完了".to_string());
    m.insert("report.score_calc".to_string(), "评分计算: ".to_string());
    m.insert("report.overall_assessment".to_string(), "总体评估".to_string());
    m.insert("report.quality_score".to_string(), "质量评分".to_string());
    m.insert("report.quality_level".to_string(), "质量等级".to_string());
    m.insert("report.analyzed_files".to_string(), "分析文件数".to_string());
    m.insert("report.total_lines".to_string(), "代码总行数".to_string());
    m.insert("report.quality_metrics".to_string(), "质量指标".to_string());
    m.insert("report.metric".to_string(), "指标".to_string());
    m.insert("report.score".to_string(), "得分".to_string());
    m.insert("report.weight".to_string(), "权重".to_string());
    m.insert("report.status".to_string(), "状态".to_string());
    m.insert("report.problem_files".to_string(), "问题文件".to_string());
    m.insert("report.issue_categories".to_string(), "问题分类".to_string());
    m.insert("report.main_issues".to_string(), "主要问题".to_string());
    m.insert("report.and".to_string(), "还有".to_string());
    m.insert("report.more_issues_short".to_string(), "个问题".to_string());
    m.insert("report.improvement_suggestions".to_string(), "改进建议".to_string());

    // 指标评分后缀
    m.insert("metric.score.suffix".to_string(), "分".to_string());

    // 循环复杂度评价
    m.insert("metric.complexity.good".to_string(), "结构清晰，不绕弯子，赞".to_string());
    m.insert("metric.complexity.medium".to_string(), "绕来绕去，跟你脑子一样乱".to_string());
    m.insert("metric.complexity.bad".to_string(), "函数像迷宫，维护像打副本".to_string());

    // 函数长度评价
    m.insert("metric.length.good".to_string(), "状态管理清晰，变量作用域合理，状态可预测".to_string());
    m.insert("metric.length.medium".to_string(), "状态管理一般，存在部分全局状态或状态变化不明确的情况".to_string());
    m.insert("metric.length.bad".to_string(), "状态管理混乱，大量使用全局变量，状态变化难以追踪".to_string());

    // 注释覆盖率评价
    m.insert("metric.comment.good".to_string(), "注释不错，能靠它活下来".to_string());
    m.insert("metric.comment.medium".to_string(), "注释稀薄，读者全靠脑补".to_string());
    m.insert("metric.comment.bad".to_string(), "没有注释，靠缘分理解吧".to_string());

    // 错误处理评价
    m.insert("metric.error.good".to_string(), "错误都照顾到了，代码有大爱".to_string());
    m.insert("metric.error.medium".to_string(), "有处理，但处理得跟没处理一样".to_string());
    m.insert("metric.error.bad".to_string(), "err 见了就跳过？宛如人生".to_string());

    // 命名规范评价
    m.insert("metric.naming.good".to_string(), "命名清晰，程序员的文明之光".to_string());
    m.insert("metric.naming.medium".to_string(), "命名还行，有些得猜".to_string());
    m.insert("metric.naming.bad".to_string(), "变量名像键盘砸出来的：x, y, z, tmp, xxx".to_string());

    // 代码重复度评价
    m.insert("metric.duplication.good".to_string(), "该抽象的都抽象了，强迫症舒服了".to_string());
    m.insert("metric.duplication.medium".to_string(), "有点重复，抽象一下不难吧".to_string());
    m.insert("metric.duplication.bad".to_string(), "一眼复制痕迹，Ctrl+C/V 荣誉勋章".to_string());

    // 代码结构评价
    m.insert("metric.structure.good".to_string(), "结构优美，不容易看岔".to_string());
    m.insert("metric.structure.medium".to_string(), "结构还行，但有点混乱".to_string());
    m.insert("metric.structure.bad".to_string(), "层层嵌套，套娃结构，看完眼花".to_string());

    // 质量建议
    m.insert("advice.good".to_string(), "👍 继续保持，你是编码界的一股清流，代码洁癖者的骄傲".to_string());
    m.insert("advice.moderate".to_string(), "🔧 建议：这代码像个叛逆期的青少年，需要适当管教才能成才".to_string());
    m.insert("advice.bad".to_string(), "🧨 建议：删库跑路是唯一出路，或者封印它，等下辈子再维护".to_string());

    // 改进建议优先级
    m.insert("advice.priority.high".to_string(), "高优先级".to_string());
    m.insert("advice.priority.medium".to_string(), "中优先级".to_string());
    m.insert("advice.priority.low".to_string(), "低优先级".to_string());

    // 良好代码的建议
    m.insert("advice.good.maintain".to_string(), "继续保持当前的代码质量标准".to_string());
    m.insert("advice.good.optimize".to_string(), "可以考虑进一步优化性能和可读性".to_string());
    m.insert("advice.good.document".to_string(), "完善文档和注释，便于团队协作".to_string());

    // 中等代码的建议
    m.insert("advice.moderate.refactor".to_string(), "重构复杂度过高的函数和模块".to_string());
    m.insert("advice.moderate.complexity".to_string(), "降低循环复杂度，简化控制流".to_string());
    m.insert("advice.moderate.naming".to_string(), "改善变量和函数命名规范".to_string());
    m.insert("advice.moderate.comments".to_string(), "增加代码注释覆盖率".to_string());
    m.insert("advice.moderate.duplication".to_string(), "消除重复代码，提取公共方法".to_string());
    m.insert("advice.moderate.structure".to_string(), "优化代码结构，减少嵌套层级".to_string());
    m.insert("advice.moderate.style".to_string(), "统一代码风格和格式".to_string());

    // 较差代码的建议
    m.insert("advice.bad.urgent_refactor".to_string(), "紧急重构过长函数，遵循单一职责原则".to_string());
    m.insert("advice.bad.complexity".to_string(), "大幅降低循环复杂度，拆分复杂逻辑".to_string());
    m.insert("advice.bad.error_handling".to_string(), "添加完善的错误处理机制".to_string());
    m.insert("advice.bad.naming".to_string(), "全面改善命名规范，避免使用无意义变量名".to_string());
    m.insert("advice.bad.duplication".to_string(), "彻底消除重复代码，建立代码复用机制".to_string());
    m.insert("advice.bad.comments".to_string(), "大幅增加代码注释，提高可读性".to_string());
    m.insert("advice.bad.structure".to_string(), "重新设计代码架构，改善整体结构".to_string());
    m.insert("advice.bad.style".to_string(), "建立并执行严格的代码规范".to_string());

    // 指标描述
    m.insert("metric.function_length.description".to_string(), "检测代码中状态变量的管理，良好的状态管理能提高代码可维护性和可预测性".to_string());
    m.insert("metric.comment_ratio.description".to_string(), "检测代码的注释覆盖率，良好的注释能提高代码可读性和可维护性".to_string());
    m.insert("metric.error_handling.description".to_string(), "检测代码中的错误处理情况，良好的错误处理能提高代码的健壮性".to_string());
    m.insert("metric.naming_convention.description".to_string(), "检测代码中的命名规范，良好的命名能提高代码可读性".to_string());
    m.insert("metric.code_duplication.description".to_string(), "评估代码中重复逻辑的比例，重复代码越多，越需要抽象和重构".to_string());
    m.insert("metric.structure_analysis.description".to_string(), "检测代码的嵌套深度和引用复杂度，评估结构清晰度".to_string());
    m.insert("metric.cyclomatic_complexity.description".to_string(), "测量函数的控制流复杂度，复杂度越高，代码越难理解和测试".to_string());

    // 质量等级描述
    m.insert("level.clean.description".to_string(), "代码洁净，令人赏心悦目".to_string());
    m.insert("level.mild.description".to_string(), "基本没事，但是有伤风化".to_string());
    m.insert("level.moderate.description".to_string(), "略有异味，建议适量通风".to_string());
    m.insert("level.bad.description".to_string(), "代码开始散发气味，谨慎维护".to_string());
    m.insert("level.terrible.description".to_string(), "臭味明显，开窗也救不了".to_string());
    m.insert("level.disaster.description".to_string(), "写的时候爽，改的时候哭".to_string());
    m.insert("level.disaster.severe.description".to_string(), "毒气弥漫，建议戴防毒面具".to_string());
    m.insert("level.disaster.very_bad.description".to_string(), "进去的程序员没有一个活着出来".to_string());
    m.insert("level.disaster.extreme.description".to_string(), "反人类罪行，建议火化".to_string());
    m.insert("level.disaster.worst.description".to_string(), "历代工程师共创的遗产，无法维护".to_string());
    m.insert("level.disaster.ultimate.description".to_string(), "写的时候热血澎湃，改的时候亲妈不认".to_string());

    // 总体评分评价
    m.insert("score.comment.0".to_string(), "如沐春风，仿佛被天使亲吻过".to_string());
    m.insert("score.comment.10".to_string(), "清新宜人，初闻像早晨的露珠".to_string());
    m.insert("score.comment.20".to_string(), "略带清香，偶尔飘过一丝酸爽".to_string());
    m.insert("score.comment.30".to_string(), "有点臭味，但还不至于熏死人".to_string());
    m.insert("score.comment.40".to_string(), "臭气扑鼻，建议佩戴口罩阅读".to_string());
    m.insert("score.comment.50".to_string(), "毒气缭绕，代码审查犹如酷刑".to_string());
    m.insert("score.comment.60".to_string(), "熏天臭气，维护者已开始咳血".to_string());
    m.insert("score.comment.70".to_string(), "生化危机，接手前请立好遗嘱".to_string());
    m.insert("score.comment.80".to_string(), "核废料现场，需穿防护服维护".to_string());
    m.insert("score.comment.90".to_string(), "厄难级毒瘤，看一眼减寿十年".to_string());

    // 错误消息
    m.insert("error.path_not_accessible".to_string(), "无法访问路径: %s".to_string());
    m.insert("error.file_read_failed".to_string(), "读取文件 %s 失败: %s".to_string());
    m.insert("error.code_parse_failed".to_string(), "解析代码 %s 失败: %s".to_string());
    m.insert("error.source_files_not_found".to_string(), "查找源文件失败: %s".to_string());
    m.insert("error.file_analysis_failed".to_string(), "分析文件 %s 失败: %s".to_string());

    // 警告和提示
    m.insert("warning.format".to_string(), "警告: %s\n".to_string());

    // 函数复杂度问题
    m.insert("issue.high_complexity".to_string(), "函数 %s 的循环复杂度过高 (%d)，考虑重构".to_string());
    m.insert("issue.medium_complexity".to_string(), "函数 %s 的循环复杂度较高 (%d)，建议简化".to_string());
    m.insert("issue.file_high_complexity".to_string(), "文件循环复杂度过高 (%d)，建议拆分为多个文件".to_string());
    m.insert("issue.file_medium_complexity".to_string(), "文件循环复杂度较高 (%d)，建议优化".to_string());

    // 函数长度问题
    m.insert("issue.function_very_long".to_string(), "函数 %s 代码行数过多 (%d 行)，极度建议拆分".to_string());
    m.insert("issue.function_long".to_string(), "函数 %s 代码行数较多 (%d 行)，建议拆分为多个小函数".to_string());
    m.insert("issue.function_medium".to_string(), "函数 %s 长度为 %d 行，考虑是否可以简化".to_string());
    m.insert("issue.file_very_long".to_string(), "文件代码行数过多 (%d 行)，建议拆分为多个文件".to_string());
    m.insert("issue.file_long".to_string(), "文件代码行数较多 (%d 行)，考虑是否可以优化结构".to_string());

    // 注释覆盖率问题
    m.insert("issue.comment_very_low".to_string(), "代码注释率极低 (%.2f%%)，几乎没有注释".to_string());
    m.insert("issue.comment_low".to_string(), "代码注释率较低 (%.2f%%)，建议增加注释".to_string());
    m.insert("issue.exported_func_no_comment".to_string(), "导出函数 %s 缺少文档注释".to_string());
    m.insert("issue.exported_type_no_comment".to_string(), "导出类型 %s 缺少文档注释".to_string());

    // 详细报告
    m.insert("verbose.basic_statistics".to_string(), "📊 基本统计:".to_string());
    m.insert("verbose.total_files".to_string(), "总文件数:".to_string());
    m.insert("verbose.total_lines".to_string(), "总代码行:".to_string());
    m.insert("verbose.total_issues".to_string(), "总问题数:".to_string());
    m.insert("verbose.metric_details".to_string(), "🔍 指标详细信息:".to_string());
    m.insert("verbose.weight".to_string(), "权重:".to_string());
    m.insert("verbose.description".to_string(), "描述:".to_string());
    m.insert("verbose.score".to_string(), "得分:".to_string());
    m.insert("verbose.all_files".to_string(), "全部代码文件分析".to_string());
    m.insert("verbose.no_files_found".to_string(), "🎉 没有找到需要分析的文件！".to_string());
    m.insert("verbose.file_good_quality".to_string(), "代码质量良好，没有明显问题".to_string());

    // 文件分析进度
    m.insert("report.analyzing_files".to_string(), "已分析文件".to_string());
    m.insert("report.files".to_string(), "个文件".to_string());

    m
});