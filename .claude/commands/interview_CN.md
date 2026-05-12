---
allowed-tools: AskUserQuestion, Read, Glob, Grep, Write, Edit
argument-hint: [plan-file]
description: 通过访谈来充实计划/规范
---

这是当前的计划：

@$ARGUMENTS

使用 AskUserQuestion 工具详细访谈我，询问任何事情：技术实现、UI 和 UX、关注点、权衡等，但确保问题不是显而易见的。

非常深入，持续访谈我直到完成，然后将规范写回 `$ARGUMENTS`。
