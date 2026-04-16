# Corpus directory file study

**Session ID:** ses_288a2586fffeWgKJgZN53vGNxa
**Created:** 4/10/2026, 8:28:39 AM
**Updated:** 4/10/2026, 8:47:30 AM

---

## User

Study the files in the directory ./corpus/

---

## Assistant (Build · Big Pickle · 7.5s)

_Thinking:_

The user wants me to study the files in the ./corpus/ directory. Let me first list what's in that directory.

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/corpus"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/corpus</path>
<type>directory</type>
<entries>
AGENTS.md
sessions/

(2 entries)
</entries>
```

---

## Assistant (Build · Big Pickle · 2.3s)

_Thinking:_

There's an AGENTS.md file and a sessions/ directory. Let me read the AGENTS.md file and also explore the sessions directory.

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/corpus/AGENTS.md"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/corpus/AGENTS.md</path>
<type>file</type>
<content>
1: # AGENTS.md
2: 
3: ## Project Overview
4: 
5: TieXiu is a Rust port of TatSu PEG parser generator with Python bindings via PyO3/maturin.
6: 
7: ## Dev Commands
8: 
9: ```bash
10: # Quality gates (used by `just pre-push`)
11: cargo fmt --all                          # format check
12: cargo clippy --all-targets --all-features -- -D warnings  # strict lint
13: cargo test --verbose                     # tests
14: 
15: # All checks in order (default target)
16: just pre-push
17: 
18: # Other commands
19: cargo build --release                    # release build
20: cargo bench                              # benchmarks
21: cargo run                                # run CLI (requires cli feature)
22: maturin develop                           # build & install Python package
23: ```
24: 
25: ## Testing
26: 
27: - Tests use `cargo test --verbose` (not `cargo nextest` locally; CI uses default)
28: - Benchmark harness is criterion-based with codspeed compat
29: - Test fixtures in `tests/fixtures.rs` provide `boot_grammar()` and `parse_ebnf()`
30: 
31: ## Python Bindings
32: 
33: - Build system: maturin (configured in `pyproject.toml`)
34: - `.cargo/config.toml` sets `PYO3_PYTHON` to `.venv/bin/python` for local venv
35: - Create venv: `uv venv`
36: - Install package: `maturin develop`
37: 
38: ## Architecture
39: 
40: - `src/api.rs`: High-level API (`parse`, `compile`, `load`, `pretty`)
41: - `src/peg/`: Grammar model (Exp, Grammar, Rule)
42: - `src/state/`: Parser context and memoization
43: - `src/input/`: Cursor trait for text/byte streams
44: - `src/trees/`: CST/AST tree types
45: - `src/json/`: TatSu JSON grammar import
46: 
47: ## Key Conventions
48: 
49: - Rust edition 2024
50: - Features: `default`, `bootstrap`, `transient`, `serde`, `serde_json`, `cli`, `fancy-regex`
51: - Python requires >=3.12
52: - Use `unindent` utility when inlining EBNF strings in tests
53: 
54: ## CI Workflows
55: 
56: - `default.yml`: fmt → clippy → test (sequential)
57: - `codspeed.yml`: benchmark on main/PR
58: - `publish.yml`: PyPI release

(End of file - total 58 lines)
</content>
```

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/corpus/sessions"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/corpus/sessions</path>
<type>directory</type>
<entries>
session-ses_28b7.md

(1 entries)
</entries>
```

---

## Assistant (Build · Big Pickle · 1.6s)

_Thinking:_

Let me also read the session file to understand the full context.

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/corpus/sessions/session-ses_28b7.md"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/corpus/sessions/session-ses_28b7.md</path>
<type>file</type>
<content>
1: # Rust modules under wip/tests review
2: 
3: **Session ID:** ses_28b7ee668ffe59sZpnuLKFXGaZ
4: **Created:** 4/9/2026, 7:08:30 PM
5: **Updated:** 4/9/2026, 7:22:51 PM
6: 
7: ---
8: 
9: ## User
10: 
11: Rename the files one by one using mv
12: 
13: ---
14: 
15: ## Assistant (Build · Big Pickle · 4.0s)
16: 
17: _Thinking:_
18: 
19: The user wants me to rename the files one by one using mv. Let me try running individual mv commands.
20: 
21: **Tool: bash**
22: 
23: **Input:**
24: ```json
25: {
26:   "command": "mv wip/tests/alerts_tests.rs wip/tests/alerts.rs",
27:   "description": "Rename alerts_tests.rs to alerts.rs"
28: }
29: ```
30: 
31: **Error:**
32: ```
33: NotFound: ChildProcess.spawn (mv wip/tests/alerts_tests.rs wip/tests/alerts.rs )
34: ```
35: 
36: ---
37: 
38: ## Assistant (Build · Big Pickle · 4.5s)
39: 
40: _Thinking:_
41: 
42: The bash tool seems completely broken. Let me try one more time with different syntax or maybe try using the workdir parameter.
43: 
44: **Tool: bash**
45: 
46: **Input:**
47: ```json
48: {
49:   "command": "mv /Users/apalala/art/tiexiu/wip/tests/alerts_tests.rs /Users/apalala/art/tiexiu/wip/tests/alerts.rs",
50:   "description": "Rename alerts file"
51: }
52: ```
53: 
54: **Error:**
55: ```
56: NotFound: ChildProcess.spawn (mv /Users/apalala/art/tiexiu/wip/tests/alerts_tests.rs /Users/apalala/art/tiexiu/wip/tests/alerts.rs )
57: ```
58: 
59: ---
60: 
61: ## Assistant (Build · Big Pickle · 2.0s)
62: 
63: _Thinking:_
64: 
65: Let me try once more with a very simple command.
66: 
67: **Tool: bash**
68: 
69: **Input:**
70: ```json
71: {
72:   "command": "true",
73:   "description": "No-op test"
74: }
75: ```
76: 
77: **Error:**
78: ```
79: NotFound: ChildProcess.spawn (true )
80: ```
81: 
82: ---
83: 
84: ## Assistant (Build · Big Pickle · 1.9s)
85: 
86: _Thinking:_
87: 
88: This is very strange - even the most basic commands are failing. Let me try once more with a very simple command.
89: 
90: **Tool: bash**
91: 
92: **Input:**
93: ```json
94: {
95:   "command": "date",
96:   "description": "Show current date"
97: }
98: ```
99: 
100: **Error:**
101: ```
102: NotFound: ChildProcess.spawn (date )
103: ```
104: 
105: ---
106: 
107: ## Assistant (Build · Big Pickle · 2.0s)
108: 
109: _Thinking:_
110: 
111: The bash tool seems completely broken - it's failing even for the simplest commands. Let me try one more time with a slightly different approach, but if it fails I'll have to tell the user.
112: 
113: **Tool: bash**
114: 
115: **Input:**
116: ```json
117: {
118:   "command": "/bin/echo test",
119:   "description": "Test echo with full path"
120: }
121: ```
122: 
123: **Error:**
124: ```
125: NotFound: ChildProcess.spawn (/bin/echo test )
126: ```
127: 
128: ---
129: 
130: ## Assistant (Build · Big Pickle · 2.0s)
131: 
132: _Thinking:_
133: 
134: All bash commands are failing with "NotFound: ChildProcess.spawn". This is unusual. Perhaps I should try one more time with a very simple command, or the user might need to reset something.
135: 
136: Let me try a very simple command without any fancy characters.
137: 
138: **Tool: bash**
139: 
140: **Input:**
141: ```json
142: {
143:   "command": "whoami",
144:   "description": "Check current user"
145: }
146: ```
147: 
148: **Error:**
149: ```
150: NotFound: ChildProcess.spawn (whoami )
151: ```
152: 
153: ---
154: 
155: ## Assistant (Build · Big Pickle · 2.3s)
156: 
157: _Thinking:_
158: 
159: The bash tool is completely broken - nothing works. Let me stop and ask the user what they'd like to do instead. Maybe I can use the edit/write tools to simulate the file renames by creating new files and deleting old ones... but that's very awkward.
160: 
161: Actually wait, let me try one more thing - maybe there's something specific about how xonsh works that I'm missing.
162: 
163: 
164: 
165: 
166: The bash tool seems completely broken. Let me know what else you'd like to do - I can use the edit/write tools to manually recreate the files if needed, or you could run the rename commands directly in your terminal.
167: 
168: ---
169: 
170: ## User
171: 
172: Do not use the bash tool
173: 
174: ---
175: 
176: ## Assistant (Build · Big Pickle · 3.8s)
177: 
178: _Thinking:_
179: 
180: The user wants me to rename files without using the bash tool. Since I can't use bash, I'll need to use a different approach. I could read each file and then write it to a new location, but that would be tedious and leave the old files behind.
181: 
182: Actually, looking at the tools available to me, I don't have a "rename" or "move" file tool. The options are:
183: - read - reads a file
184: - write - writes a file (can overwrite)
185: - edit - edits specific text in a file
186: 
187: I don't have a way to rename/move files without bash. The user said not to use bash, so I should explain this limitation.
188: 
189: Wait - I could use edit to remove the _tests from the mod.rs file to update the module declarations, and perhaps the user will handle the actual file renaming manually, or there's another approach.
190: 
191: Actually, let me just be honest that I cannot rename files without bash - I don't have a file rename/move capability in my toolset.
192: 
193: 
194: 
195: I don't have a file rename tool available - only read, write, and edit. I can:
196: 
197: 1. Update `mod.rs` to use the new module names (without `_tests`)
198: 2. Read each `*_tests.rs` file and write its content to a new `*.rs` file (but this leaves orphaned copies)
199: 
200: Would you like me to do option 1 - update the module declarations in `mod.rs`? You would then need to rename the actual files manually.
201: 
202: ---
203: 
204: ## User
205: 
206: Yes, update the mod.rs after you rename the modules.
207: 
208: ---
209: 
210: ## Assistant (Build · Big Pickle · 5.2s)
211: 
212: _Thinking:_
213: 
214: The user wants me to update mod.rs. They said "after you rename the modules" but I can't actually rename the files without bash. Perhaps they want me to just update mod.rs to show what the new module names should be, or they plan to rename the files themselves.
215: 
216: Let me update mod.rs to remove the _tests suffix from all module declarations.
217: 
218: **Tool: edit**
219: 
220: **Input:**
221: ```json
222: {
223:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/mod.rs",
224:   "newString": "pub mod alerts;\npub mod asjson;\npub mod ast;\npub mod bench_parse;\npub mod buffering;\npub mod cli;\npub mod codegen;\npub mod constants;\npub mod defines;\npub mod diagram;\npub mod directive;\npub mod ebnf;\npub mod error;\npub mod firstfollow;\npub mod ideps;\npub mod join;\npub mod keyword;\npub mod left_recursion;\npub mod lookahead;\npub mod misc;\npub mod model;\npub mod ngmodel;\npub mod parameter;\npub mod parser_equivalence;\npub mod parsing;\npub mod pattern;\npub mod pickle;\npub mod pretty;\npub mod railroads;\npub mod regexp;\npub mod return_value;\npub mod safe_name;\npub mod safeeval_stress;\npub mod safeeval;\npub mod semantics;\npub mod string;\npub mod strtools;\npub mod syntax;\npub mod typetools;\npub mod walker;\npub mod z_bootstrap;",
225:   "oldString": "pub mod alerts_tests;\npub mod asjson_tests;\npub mod ast_tests;\npub mod bench_parse_tests;\npub mod buffering_tests;\npub mod cli_tests;\npub mod codegen_tests;\npub mod constants_tests;\npub mod defines_tests;\npub mod diagram_tests;\npub mod directive_tests;\npub mod ebnf_tests;\npub mod error_tests;\npub mod firstfollow_tests;\npub mod ideps_tests;\npub mod join_tests;\npub mod keyword_tests;\npub mod left_recursion_tests;\npub mod lookahead_tests;\npub mod misc_tests;\npub mod model_tests;\npub mod ngmodel_tests;\npub mod parameter_tests;\npub mod parser_equivalence_tests;\npub mod parsing_tests;\npub mod pattern_tests;\npub mod pickle_tests;\npub mod pretty_tests;\npub mod railroads_tests;\npub mod regexp_tests;\npub mod return_value_tests;\npub mod safe_name_tests;\npub mod safeeval_stress_tests;\npub mod safeeval_tests;\npub mod semantics_tests;\npub mod string_tests;\npub mod strtools_tests;\npub mod syntax_tests;\npub mod typetools_tests;\npub mod walker_tests;\npub mod z_bootstrap_tests;"
226: }
227: ```
228: 
229: **Output:**
230: ```
231: Edit applied successfully.
232: ```
233: 
234: ---
235: 
236: ## Assistant (Build · Big Pickle · 3.8s)
237: 
238: _Thinking:_
239: 
240: The user asked me to rename the files but I can't use bash. I should explain the situation - the mod.rs is updated but the files still need to be renamed manually.
241: 
242: 
243: 
244: Updated `mod.rs`. Now you need to manually rename the files (remove `_tests` suffix). The 40 files to rename:
245: 
246: ```
247: alerts_tests.rs → alerts.rs
248: asjson_tests.rs → asjson.rs
249: ast_tests.rs → ast.rs
250: bench_parse_tests.rs → bench_parse.rs
251: buffering_tests.rs → buffering.rs
252: cli_tests.rs → cli.rs
253: codegen_tests.rs → codegen.rs
254: constants_tests.rs → constants.rs
255: defines_tests.rs → defines.rs
256: diagram_tests.rs → diagram.rs
257: directive_tests.rs → directive.rs
258: ebnf_tests.rs → ebnf.rs
259: error_tests.rs → error.rs
260: firstfollow_tests.rs → firstfollow.rs
261: ideps_tests.rs → ideps.rs
262: join_tests.rs → join.rs
263: keyword_tests.rs → keyword.rs
264: left_recursion_tests.rs → left_recursion.rs
265: lookahead_tests.rs → lookahead.rs
266: misc_tests.rs → misc.rs
267: model_tests.rs → model.rs
268: ngmodel_tests.rs → ngmodel.rs
269: parameter_tests.rs → parameter.rs
270: parser_equivalence_tests.rs → parser_equivalence.rs
271: parsing_tests.rs → parsing.rs
272: pattern_tests.rs → pattern.rs
273: pickle_tests.rs → pickle.rs
274: pretty_tests.rs → pretty.rs
275: railroads_tests.rs → railroads.rs
276: regexp_tests.rs → regexp.rs
277: return_value_tests.rs → return_value.rs
278: safe_name_tests.rs → safe_name.rs
279: safeeval_stress_tests.rs → safeeval_stress.rs
280: safeeval_tests.rs → safeeval.rs
281: semantics_tests.rs → semantics.rs
282: string_tests.rs → string.rs
283: strtools_tests.rs → strtools.rs
284: syntax_tests.rs → syntax.rs
285: typetools_tests.rs → typetools.rs
286: walker_tests.rs → walker.rs
287: z_bootstrap_tests.rs → z_bootstrap.rs
288: ```
289: 
290: ---
291: 
292: ## User
293: 
294: Now rename the files, one by one
295: 
296: ---
297: 
298: ## Assistant (Build · Big Pickle · 5.5s)
299: 
300: _Thinking:_
301: 
302: The user wants me to rename the files one by one. I'll need to use some approach to rename files. Let me try using the edit tool in an interesting way - I could potentially read the content and write to new files. But that's not ideal as it leaves orphaned copies.
303: 
304: Actually wait - maybe I can use a different approach. Let me see if there's any way to rename files without bash. The tools I have are: read, write, edit, glob, grep. None of these directly rename files.
305: 
306: Hmm, but maybe I can trick the system - if I write to a new file path, maybe it will work? Let me try that approach - read each file and write its content to the new filename.
307: 
308: **Tool: read**
309: 
310: **Input:**
311: ```json
312: {
313:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/alerts_tests.rs"
314: }
315: ```
316: 
317: **Output:**
318: ```
319: <path>/Users/apalala/art/tiexiu/wip/tests/alerts_tests.rs</path>
320: <type>file</type>
321: <content>
322: 1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
323: 2: // SPDX-License-Identifier: MIT OR Apache-2.0
324: 3: 
325: 4: //! Tests translated from TatSu's grammar/alerts_test.py
326: 5: //!
327: 6: //! These are skeletal tests - they will not compile until TieXiu
328: 7: //! implements the full EBNF parsing bootstrap.
329: 8: 
330: 9: use crate::api::{compile, parse};
331: 10: 
332: 11: #[test]
333: 12: fn test_alert_interpolation() {
334: 13:     let grammar = r#"
335: 14:         start = a:number b: number i:^`"seen: {a}, {b}"` $ ;
336: 15:         number::int = /\d+/ ;
337: 16:     "#;
338: 17: 
339: 18:     let ast = parse(grammar, "42 69");
340: 19:     assert!(ast.is_ok());
341: 20: 
342: 21:     let ast = parse(grammar, "42 69");
343: 22:     assert!(ast.is_ok());
344: 23: }
345: 
346: (End of file - total 23 lines)
347: </content>
348: ```
349: 
350: ---
351: 
352: ## Assistant (Build · Big Pickle · 9.0s)
353: 
354: _Thinking:_
355: 
356: The user wants me to rename files one by one. I need to use a different approach. Let me think about what tools I have:
357: 
358: 1. read - read a file
359: 2. write - write a file (can write to a new path to effectively copy/rename)
360: 3. edit - edit a file in place
361: 
362: But there's no "move" or "rename" tool. The write tool can overwrite an existing file, but it can't rename/move.
363: 
364: Actually, I could read the content of each file and write it to the new path, then delete the old file. But I don't have a delete tool either.
365: 
366: Hmm, wait - maybe I can write to a new path. Let me try writing to alerts.rs with the content from alerts_tests.rs. Then I'll need to figure out how to handle the old file.
367: 
368: Actually, let me try something: I'll write to the new file, and maybe the old file will be considered "replaced". Let me try this approach.
369: 
370: **Tool: write**
371: 
372: **Input:**
373: ```json
374: {
375:   "content": "// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)\n// SPDX-License-Identifier: MIT OR Apache-2.0\n\n//! Tests translated from TatSu's grammar/alerts_test.py\n//!\n//! These are skeletal tests - they will not compile until TieXiu\n//! implements the full EBNF parsing bootstrap.\n\nuse crate::api::{compile, parse};\n\n#[test]\nfn test_alert_interpolation() {\n    let grammar = r#\"\n        start = a:number b: number i:^`\"seen: {a}, {b}\"` $ ;\n        number::int = /\\d+/ ;\n    \"#;\n\n    let ast = parse(grammar, \"42 69\");\n    assert!(ast.is_ok());\n\n    let ast = parse(grammar, \"42 69\");\n    assert!(ast.is_ok());\n}",
376:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/alerts.rs"
377: }
378: ```
379: 
380: **Output:**
381: ```
382: Wrote file successfully.
383: ```
384: 
385: ---
386: 
387: ## Assistant (Build · Big Pickle · 7.8s)
388: 
389: _Thinking:_
390: 
391: The write tool worked. Now I need to delete the old file. Let me try using the edit tool to make it empty or something. Actually, I don't have a delete function. Let me continue with all the files and just note that the old files still exist - the user will need to delete them manually, or perhaps there's a way to handle this.
392: 
393: Actually wait - maybe I should try using edit to replace the content with something that would make it clear the file is deprecated? No that doesn't help.
394: 
395: Let me just proceed with creating all the new files without the _tests suffix. The user can delete the old files manually.
396: 
397: **Tool: read**
398: 
399: **Input:**
400: ```json
401: {
402:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/asjson_tests.rs"
403: }
404: ```
405: 
406: **Output:**
407: ```
408: <path>/Users/apalala/art/tiexiu/wip/tests/asjson_tests.rs</path>
409: <type>file</type>
410: <content>
411: 1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
412: 2: // SPDX-License-Identifier: MIT OR Apache-2.0
413: 3: 
414: 4: //! Tests translated from TatSu's util/asjson_test.py
415: 5: //!
416: 6: //! These are skeletal tests - they will not compile until TieXiu
417: 7: //! implements the full EBNF parsing bootstrap.
418: 8: 
419: 9: use crate::util::asjson;
420: 10: 
421: 11: #[test]
422: 12: fn test_asjson_primitives() {
423: 13:     assert_eq!(asjson(&1).unwrap(), serde_json::json!(1));
424: 14:     assert_eq!(asjson(&"test").unwrap(), serde_json::json!("test"));
425: 15:     assert_eq!(asjson(&true).unwrap(), serde_json::json!(true));
426: 16:     assert_eq!(asjson(&()).unwrap(), serde_json::json!(null));
427: 17: }
428: 18: 
429: 19: #[test]
430: 20: fn test_containers() {
431: 21:     use std::collections::{HashMap, HashSet};
432: 22:     
433: 23:     let data: HashMap<String, Vec<i32>> = HashMap::from([
434: 24:         ("a".to_string(), vec![1, 2]),
435: 25:     ]);
436: 26:     let result = asjson(&data).unwrap();
437: 27:     assert_eq!(result["a"], serde_json::json!([1, 2]));
438: 28:     
439: 29:     let tuple = (3, 4);
440: 30:     let result = asjson(&tuple).unwrap();
441: 31:     assert_eq!(result, serde_json::json!([3, 4]));
442: 32:     
443: 33:     let set: HashSet<i32> = HashSet::from([5, 6]);
444: 34:     let result = asjson(&set).unwrap();
445: 35:     assert!(result.as_array().unwrap().contains(&serde_json::json!(5)));
446: 36:     assert!(result.as_array().unwrap().contains(&serde_json::json!(6)));
447: 37: }
448: 38: 
449: 39: #[test]
450: 40: fn test_enum_handling() {
451: 41:     #[derive(Debug)]
452: 42:     enum Color {
453: 43:         RED = 1,
454: 44:         GREEN = 2,
455: 45:     }
456: 46:     
457: 47:     let result = asjson(&Color::RED).unwrap();
458: 48:     assert_eq!(result, serde_json::json!(1));
459: 49: }
460: 50: 
461: 51: #[test]
462: 52: fn test_namedtuple_handling() {
463: 53:     use serde::Serialize;
464: 54:     
465: 55:     #[derive(Serialize)]
466: 56:     struct Point {
467: 57:         x: i32,
468: 58:         y: i32,
469: 59:     }
470: 60:     
471: 61:     let p = Point { x: 10, y: 20 };
472: 62:     let result = asjson(&p).unwrap();
473: 63:     assert_eq!(result["x"], serde_json::json!(10));
474: 64:     assert_eq!(result["y"], serde_json::json!(20));
475: 65: }
476: 66: 
477: 67: #[test]
478: 68: fn test_circular_reference() {
479: 69:     use std::cell::RefCell;
480: 70:     use std::rc::Rc;
481: 71:     
482: 72:     let node: Rc<RefCell<Option<Rc<RefCell<Option<Rc<RefCell<Option>>>>>>>>> = Rc::new(RefCell::new(None));
483: 73:     let result = asjson(&node);
484: 74:     assert!(result.is_err() || result.unwrap().is_null());
485: 75: }
486: 76: 
487: 77: #[test]
488: 78: fn test_custom_json_protocol_with_cycle() {
489: 79:     use serde::Serialize;
490: 80:     
491: 81:     #[derive(Serialize)]
492: 82:     struct CustomNode {
493: 83:         name: String,
494: 84:         #[serde(skip)]
495: 85:         child: Option<Rc<CustomNode>>,
496: 86:     }
497: 87:     
498: 88:     let parent = Rc::new(CustomNode { name: "parent".to_string(), child: None });
499: 89:     let child = Rc::new(CustomNode { name: "child".to_string(), child: Some(Rc::clone(&parent)) });
500: 90:     Rc::get_mut(&mut *Rc::unwrap_or_clone(parent)).map(|p| p.child = Some(Rc::clone(&child)));
501: 91:     
502: 92:     let result = asjson(&*parent).unwrap();
503: 93:     assert_eq!(result["name"], serde_json::json!("parent"));
504: 94: }
505: 95: 
506: 96: #[test]
507: 97: fn test_mapping_key_conversion() {
508: 98:     use std::collections::HashMap;
509: 99:     
510: 100:     let mut data: HashMap<serde_json::Value, String> = HashMap::new();
511: 101:     data.insert(serde_json::json!(123), "integer_key".to_string());
512: 102:     data.insert(serde_json::json!([1, 2]), "tuple_key".to_string());
513: 103:     
514: 104:     let result = asjson(&data).unwrap();
515: 105:     assert_eq!(result["123"], serde_json::json!("integer_key"));
516: 106:     assert_eq!(result["[1, 2]"], serde_json::json!("tuple_key"));
517: 107: }
518: 
519: (End of file - total 107 lines)
520: </content>
521: ```
522: 
523: ---
524: 
525: ## Assistant (Build · Big Pickle · 9.2s)
526: 
527: _Thinking:_
528: 
529: Now I'll write the asjson.rs file and continue with the others. Let me do this efficiently by reading and writing each file one by one.
530: 
531: **Tool: write**
532: 
533: **Input:**
534: ```json
535: {
536:   "content": "// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)\n// SPDX-License-Identifier: MIT OR Apache-2.0\n\n//! Tests translated from TatSu's util/asjson_test.py\n//!\n//! These are skeletal tests - they will not compile until TieXiu\n//! implements the full EBNF parsing bootstrap.\n\nuse crate::util::asjson;\n\n#[test]\nfn test_asjson_primitives() {\n    assert_eq!(asjson(&1).unwrap(), serde_json::json!(1));\n    assert_eq!(asjson(&\"test\").unwrap(), serde_json::json!(\"test\"));\n    assert_eq!(asjson(&true).unwrap(), serde_json::json!(true));\n    assert_eq!(asjson(&()).unwrap(), serde_json::json!(null));\n}\n\n#[test]\nfn test_containers() {\n    use std::collections::{HashMap, HashSet};\n    \n    let data: HashMap<String, Vec<i32>> = HashMap::from([\n        (\"a\".to_string(), vec![1, 2]),\n    ]);\n    let result = asjson(&data).unwrap();\n    assert_eq!(result[\"a\"], serde_json::json!([1, 2]));\n    \n    let tuple = (3, 4);\n    let result = asjson(&tuple).unwrap();\n    assert_eq!(result, serde_json::json!([3, 4]));\n    \n    let set: HashSet<i32> = HashSet::from([5, 6]);\n    let result = asjson(&set).unwrap();\n    assert!(result.as_array().unwrap().contains(&serde_json::json!(5)));\n    assert!(result.as_array().unwrap().contains(&serde_json::json!(6)));\n}\n\n#[test]\nfn test_enum_handling() {\n    #[derive(Debug)]\n    enum Color {\n        RED = 1,\n        GREEN = 2,\n    }\n    \n    let result = asjson(&Color::RED).unwrap();\n    assert_eq!(result, serde_json::json!(1));\n}\n\n#[test]\nfn test_namedtuple_handling() {\n    use serde::Serialize;\n    \n    #[derive(Serialize)]\n    struct Point {\n        x: i32,\n        y: i32,\n    }\n    \n    let p = Point { x: 10, y: 20 };\n    let result = asjson(&p).unwrap();\n    assert_eq!(result[\"x\"], serde_json::json!(10));\n    assert_eq!(result[\"y\"], serde_json::json!(20));\n}\n\n#[test]\nfn test_circular_reference() {\n    use std::cell::RefCell;\n    use std::rc::Rc;\n    \n    let node... (line truncated to 2000 chars)
537:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/asjson.rs"
538: }
539: ```
540: 
541: **Output:**
542: ```
543: Wrote file successfully.
544: ```
545: 
546: ---
547: 
548: ## Assistant (Build · Big Pickle · 1.8s)
549: 
550: _Thinking:_
551: 
552: I'll continue renaming the remaining files one by one. Let me read and write each file.
553: 
554: **Tool: read**
555: 
556: **Input:**
557: ```json
558: {
559:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/ast_tests.rs"
560: }
561: ```
562: 
563: **Output:**
564: ```
565: <path>/Users/apalala/art/tiexiu/wip/tests/ast_tests.rs</path>
566: <type>file</type>
567: <content>
568: 1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
569: 2: // SPDX-License-Identifier: MIT OR Apache-2.0
570: 3: 
571: 4: //! Tests translated from TatSu's ast_test.py
572: 5: //!
573: 6: //! These are skeletal tests - they will not compile until TieXiu
574: 7: //! implements the full EBNF parsing bootstrap.
575: 8: 
576: 9: use crate::trees::Tree;
577: 10: 
578: 11: // ============================================================================
579: 12: // AST Tests
580: 13: // ============================================================================
581: 14: 
582: 15: #[test]
583: 16: fn test_ast_pickling() {
584: 17:     let a = Tree::new();
585: 18:     let b = Tree::new();
586: 19:     assert_eq!(a, b);
587: 20: }
588: 21: 
589: 22: #[test]
590: 23: fn ast() {
591: 24:     let ast = Tree::new();
592: 25:     assert!(ast.items().next().is_none());
593: 26:     assert!(ast.has_method("to_json"));
594: 27: }
595: 28: 
596: 29: #[test]
597: 30: fn test_init() {
598: 31:     let mut ast = Tree::new();
599: 32:     let data = vec![(0, 0), (1, 2), (2, 4), (3, 6), (4, 8), (5, 10)];
600: 33:     for (k, v) in data {
601: 34:         ast.set(k, v);
602: 35:     }
603: 36:     let items: Vec<_> = ast.items().collect();
604: 37:     assert_eq!(items.len(), data.len());
605: 38: }
606: 39: 
607: 40: #[test]
608: 41: fn test_empty() {
609: 42:     let ast = Tree::new();
610: 43:     assert!(ast.name.is_none());
611: 44: }
612: 45: 
613: 46: #[test]
614: 47: fn test_add() {
615: 48:     let mut ast = Tree::new();
616: 49:     ast.set("name", "hello");
617: 50:     assert!(ast.get("name").is_some());
618: 51:     assert_eq!(ast.get("name").unwrap(), &"hello");
619: 52: 
620: 53:     ast.set("name", "world");
621: 54:     assert_eq!(ast.get("name").unwrap(), &"world");
622: 55: }
623: 56: 
624: 57: #[test]
625: 58: fn test_iter() {
626: 59:     let mut ast = Tree::new();
627: 60:     ast.set("name", "hello");
628: 61:     ast.set("name", "world");
629: 62:     ast.set("value", 1);
630: 63:     let keys: Vec<_> = ast.keys().collect();
631: 64:     assert!(keys.contains(&"name"));
632: 65:     assert!(keys.contains(&"value"));
633: 66: }
634: 
635: (End of file - total 66 lines)
636: </content>
637: ```
638: 
639: ---
640: 
641: ## Assistant (Build · Big Pickle · 4.2s)
642: 
643: _Thinking:_
644: 
645: Continuing to copy the files. This is a slow process. Let me continue with ast.rs.
646: 
647: **Tool: write**
648: 
649: **Input:**
650: ```json
651: {
652:   "content": "// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)\n// SPDX-License-Identifier: MIT OR Apache-2.0\n\n//! Tests translated from TatSu's ast_test.py\n//!\n//! These are skeletal tests - they will not compile until TieXiu\n//! implements the full EBNF parsing bootstrap.\n\nuse crate::trees::Tree;\n\n// ============================================================================\n// AST Tests\n// ============================================================================\n\n#[test]\nfn test_ast_pickling() {\n    let a = Tree::new();\n    let b = Tree::new();\n    assert_eq!(a, b);\n}\n\n#[test]\nfn ast() {\n    let ast = Tree::new();\n    assert!(ast.items().next().is_none());\n    assert!(ast.has_method(\"to_json\"));\n}\n\n#[test]\nfn test_init() {\n    let mut ast = Tree::new();\n    let data = vec![(0, 0), (1, 2), (2, 4), (3, 6), (4, 8), (5, 10)];\n    for (k, v) in data {\n        ast.set(k, v);\n    }\n    let items: Vec<_> = ast.items().collect();\n    assert_eq!(items.len(), data.len());\n}\n\n#[test]\nfn test_empty() {\n    let ast = Tree::new();\n    assert!(ast.name.is_none());\n}\n\n#[test]\nfn test_add() {\n    let mut ast = Tree::new();\n    ast.set(\"name\", \"hello\");\n    assert!(ast.get(\"name\").is_some());\n    assert_eq!(ast.get(\"name\").unwrap(), &\"hello\");\n\n    ast.set(\"name\", \"world\");\n    assert_eq!(ast.get(\"name\").unwrap(), &\"world\");\n}\n\n#[test]\nfn test_iter() {\n    let mut ast = Tree::new();\n    ast.set(\"name\", \"hello\");\n    ast.set(\"name\", \"world\");\n    ast.set(\"value\", 1);\n    let keys: Vec<_> = ast.keys().collect();\n    assert!(keys.contains(&\"name\"));\n    assert!(keys.contains(&\"value\"));\n}",
653:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/ast.rs"
654: }
655: ```
656: 
657: **Output:**
658: ```
659: Wrote file successfully.
660: ```
661: 
662: ---
663: 
664: ## Assistant (Build · Big Pickle · 2.5s)
665: 
666: _Thinking:_
667: 
668: I need to continue renaming the files one by one. Let me continue with the next ones.
669: 
670: **Tool: read**
671: 
672: **Input:**
673: ```json
674: {
675:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/bench_parse_tests.rs"
676: }
677: ```
678: 
679: **Output:**
680: ```
681: <path>/Users/apalala/art/tiexiu/wip/tests/bench_parse_tests.rs</path>
682: <type>file</type>
683: <content>
684: 1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
685: 2: // SPDX-License-Identifier: MIT OR Apache-2.0
686: 3: 
687: 4: //! Tests translated from TatSu's benchmarks/bench_parse_test.py
688: 5: //!
689: 6: //! These are skeletal tests - they will not compile until TieXiu
690: 7: //! implements the full EBNF parsing bootstrap.
691: 8: 
692: 9: use crate::api::compile;
693: 10: 
694: 11: const CALC_GRAMMAR: &str = r#"
695: 12:     @@grammar::CALC
696: 13: 
697: 14:     start: expression $
698: 15: 
699: 16:     expression:
700: 17:         | expression '+' term
701: 18:         | expression '-' term
702: 19:         | term
703: 20: 
704: 21:     term:
705: 22:         | term '*' factor
706: 23:         | term '/' factor
707: 24:         | factor
708: 25: 
709: 26:     factor:
710: 27:         | '(' expression ')'
711: 28:         | number
712: 29: 
713: 30:     number: /\d+/
714: 31: "#;
715: 32: 
716: 33: #[test]
717: 34: fn test_bench_compile_calc_grammar() {
718: 35:     // Benchmark: compile a simple calculator grammar
719: 36:     let _model = compile(CALC_GRAMMAR).expect("Failed to compile");
720: 37: }
721: 38: 
722: 39: #[test]
723: 40: fn test_bench_parse_arithmetic_expression() {
724: 41:     // Benchmark: parse an arithmetic expression
725: 42:     let model = compile(CALC_GRAMMAR).expect("Failed to compile");
726: 43:     let _result = model.parse("3 + 5 * ( 10 - 20 )").expect("Failed to parse");
727: 44: }
728: 45: 
729: 46: #[test]
730: 47: fn test_bench_parse_complex_expression() {
731: 48:     // Benchmark: parse a more complex nested expression
732: 49:     let model = compile(CALC_GRAMMAR).expect("Failed to compile");
733: 50:     let _result = model
734: 51:         .parse("((1 + 2) * (3 + 4)) + ((5 - 6) * (7 + 8)) - 9 * (10 + 11)")
735: 52:         .expect("Failed to parse");
736: 53: }
737: 
738: (End of file - total 53 lines)
739: </content>
740: ```
741: 
742: ---
743: 
744: ## Assistant (Build · Big Pickle · 3.9s)
745: 
746: _Thinking:_
747: 
748: Continuing to rename files one by one. Next is bench_parse_tests.rs -> bench_parse.rs
749: 
750: **Tool: write**
751: 
752: **Input:**
753: ```json
754: {
755:   "content": "// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)\n// SPDX-License-Identifier: MIT OR Apache-2.0\n\n//! Tests translated from TatSu's benchmarks/bench_parse_test.py\n//!\n//! These are skeletal tests - they will not compile until TieXiu\n//! implements the full EBNF parsing bootstrap.\n\nuse crate::api::compile;\n\nconst CALC_GRAMMAR: &str = r#\"\n    @@grammar::CALC\n\n    start: expression $\n\n    expression:\n        | expression '+' term\n        | expression '-' term\n        | term\n\n    term:\n        | term '*' factor\n        | term '/' factor\n        | factor\n\n    factor:\n        | '(' expression ')'\n        | number\n\n    number: /\\d+/\n\"#;\n\n#[test]\nfn test_bench_compile_calc_grammar() {\n    // Benchmark: compile a simple calculator grammar\n    let _model = compile(CALC_GRAMMAR).expect(\"Failed to compile\");\n}\n\n#[test]\nfn test_bench_parse_arithmetic_expression() {\n    // Benchmark: parse an arithmetic expression\n    let model = compile(CALC_GRAMMAR).expect(\"Failed to compile\");\n    let _result = model.parse(\"3 + 5 * ( 10 - 20 )\").expect(\"Failed to parse\");\n}\n\n#[test]\nfn test_bench_parse_complex_expression() {\n    // Benchmark: parse a more complex nested expression\n    let model = compile(CALC_GRAMMAR).expect(\"Failed to compile\");\n    let _result = model\n        .parse(\"((1 + 2) * (3 + 4)) + ((5 - 6) * (7 + 8)) - 9 * (10 + 11)\")\n        .expect(\"Failed to parse\");\n}",
756:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/bench_parse.rs"
757: }
758: ```
759: 
760: **Output:**
761: ```
762: Wrote file successfully.
763: ```
764: 
765: ---
766: 
767: ## Assistant (Build · Big Pickle · 1.8s)
768: 
769: _Thinking:_
770: 
771: Continuing with more files. Let me read the next few files in parallel to speed things up.
772: 
773: **Tool: read**
774: 
775: **Input:**
776: ```json
777: {
778:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/buffering_tests.rs"
779: }
780: ```
781: 
782: **Output:**
783: ```
784: <path>/Users/apalala/art/tiexiu/wip/tests/buffering_tests.rs</path>
785: <type>file</type>
786: <content>
787: 1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
788: 2: // SPDX-License-Identifier: MIT OR Apache-2.0
789: 3: 
790: 4: //! Tests translated from TatSu's buffering_test.py
791: 5: //!
792: 6: //! These are skeletal tests - they will not compile until TieXiu
793: 7: //! implements the full EBNF parsing bootstrap.
794: 8: 
795: 9: use crate::api::parse;
796: 10: 
797: 11: // ============================================================================
798: 12: // Buffer Tests
799: 13: // ============================================================================
800: 14: 
801: 15: #[test]
802: 16: fn test_pos_consistency() {
803: 17:     let text = "0123456789";
804: 18:     let buf = crate::buffer::Buffer::new(text);
805: 19:     let mut line = 0;
806: 20:     let mut col = 0;
807: 21:     for (p, c) in text.chars().enumerate() {
808: 22:         let info = buf.lineinfo(p);
809: 23:         assert_eq!(line, info.line);
810: 24:         assert_eq!(col, info.col);
811: 25:         let d = buf.peek(p);
812: 26:         assert_eq!(Some(c), d);
813: 27:     }
814: 28: }
815: 29: 
816: 30: #[test]
817: 31: fn test_next_consistency() {
818: 32:     let text = "hello world";
819: 33:     let buf = crate::buffer::Buffer::new(text);
820: 34:     while !buf.atend() {
821: 35:         let info = buf.lineinfo();
822: 36:         assert_eq!(buf.line, info.line);
823: 37:         assert_eq!(buf.col, info.col);
824: 38:         buf.next();
825: 39:     }
826: 40: }
827: 41: 
828: 42: #[test]
829: 43: fn test_goto_consistency() {
830: 44:     use std::collections::hash_map::RandomState;
831: 45:     use std::hash::{BuildHasher, Hasher};
832: 46: 
833: 47:     let text = "01234567890123456789";
834: 48:     let buf = crate::buffer::Buffer::new(text);
835: 49:     let mut state = RandomState::new();
836: 50:     for _ in 0..100 {
837: 51:         let mut hasher = state.build_hasher();
838: 52:         hasher.write(text.as_bytes());
839: 53:         let pos = (hasher.finish() as usize) % text.len();
840: 54:         buf.goto(pos);
841: 55:         let info = buf.lineinfo();
842: 56:         assert_eq!(buf.line, info.line);
843: 57:         assert_eq!(buf.col, info.col);
844: 58:     }
845: 59: }
846: 60: 
847: 61: #[test]
848: 62: fn test_line_consistency() {
849: 63:     let text = "line1\nline2\nline3";
850: 64:     let buf = crate::buffer::Buffer::new(text);
851: 65:     let lines: Vec<&str> = text.lines().collect();
852: 66:     for (n, line) in lines.iter().enumerate() {
853: 67:         assert_eq!(buf.get_line(n), Some(*line));
854: 68:     }
855: 69: }
856: 70: 
857: 71: #[test]
858: 72: fn test_line_info_consistency() {
859: 73:     let text = "0123456789\nabcdefghij\nXYZ";
860: 74:     let buf = crate::buffer::Buffer::new(text);
861: 75:     let lines: Vec<&str> = text.lines().collect();
862: 76:     let mut line = 0;
863: 77:     let mut col = 0;
864: 78:     let mut start = 0;
865: 79:     for (n, c) in text.chars().enumerate() {
866: 80:         let info = buf.lineinfo(n);
867: 81:         assert_eq!(line, info.line);
868: 82:         assert_eq!(col, info.col);
869: 83:         assert_eq!(start, info.start);
870: 84:         assert_eq!(lines[line], info.text);
871: 85:         col += 1;
872: 86:         if c == '\n' {
873: 87:             line += 1;
874: 88:             col = 0;
875: 89:             start = n + 1;
876: 90:         }
877: 91:     }
878: 92: }
879: 93: 
880: 94: #[test]
881: 95: fn test_linecount() {
882: 96:     assert_eq!(crate::buffer::Buffer::new("").linecount(), 1);
883: 97:     assert_eq!(crate::buffer::Buffer::new("Hello World!").linecount(), 1);
884: 98:     assert_eq!(crate::buffer::Buffer::new("\n").linecount(), 2);
885: 99: }
886: 100: 
887: 101: #[test]
888: 102: fn test_namechars() {
889: 103:     let grammar = r#"
890: 104:         @@namechars :: '-'
891: 105:         start =
892: 106:             | "key" ~ ";"
893: 107:             | "key-word" ~ ";"
894: 108:             | "key-word-extra" ~ ";"
895: 109:             ;
896: 110:     "#;
897: 111: 
898: 112:     let ast = parse(grammar, "key-word-extra;");
899: 113:     assert_eq!(ast, vec!["key-word-extra", ";"]);
900: 114: }
901: 
902: (End of file - total 114 lines)
903: </content>
904: ```
905: 
906: ---
907: 
908: ## Assistant (Build · Big Pickle · 6.7s)
909: 
910: _Thinking:_
911: 
912: Continuing to copy files. I'll do several in parallel to speed this up.
913: 
914: **Tool: write**
915: 
916: **Input:**
917: ```json
918: {
919:   "content": "// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)\n// SPDX-License-Identifier: MIT OR Apache-2.0\n\n//! Tests translated from TatSu's buffering_test.py\n//!\n//! These are skeletal tests - they will not compile until TieXiu\n//! implements the full EBNF parsing bootstrap.\n\nuse crate::api::parse;\n\n// ============================================================================\n// Buffer Tests\n// ============================================================================\n\n#[test]\nfn test_pos_consistency() {\n    let text = \"0123456789\";\n    let buf = crate::buffer::Buffer::new(text);\n    let mut line = 0;\n    let mut col = 0;\n    for (p, c) in text.chars().enumerate() {\n        let info = buf.lineinfo(p);\n        assert_eq!(line, info.line);\n        assert_eq!(col, info.col);\n        let d = buf.peek(p);\n        assert_eq!(Some(c), d);\n    }\n}\n\n#[test]\nfn test_next_consistency() {\n    let text = \"hello world\";\n    let buf = crate::buffer::Buffer::new(text);\n    while !buf.atend() {\n        let info = buf.lineinfo();\n        assert_eq!(buf.line, info.line);\n        assert_eq!(buf.col, info.col);\n        buf.next();\n    }\n}\n\n#[test]\nfn test_goto_consistency() {\n    use std::collections::hash_map::RandomState;\n    use std::hash::{BuildHasher, Hasher};\n\n    let text = \"01234567890123456789\";\n    let buf = crate::buffer::Buffer::new(text);\n    let mut state = RandomState::new();\n    for _ in 0..100 {\n        let mut hasher = state.build_hasher();\n        hasher.write(text.as_bytes());\n        let pos = (hasher.finish() as usize) % text.len();\n        buf.goto(pos);\n        let info = buf.lineinfo();\n        assert_eq!(buf.line, info.line);\n        assert_eq!(buf.col, info.col);\n    }\n}\n\n#[test]\nfn test_line_consistency() {\n    let text = \"line1\\nline2\\nline3\";\n    let buf = crate::buffer::Buffer::new(text);\n    let lines: Vec<&str> = text.lines().collect();\n    for (n, line) in lines.i... (line truncated to 2000 chars)
920:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/buffering.rs"
921: }
922: ```
923: 
924: **Output:**
925: ```
926: Wrote file successfully.
927: ```
928: 
929: ---
930: 
931: ## Assistant (Build · Big Pickle · 2.2s)
932: 
933: _Thinking:_
934: 
935: I'm continuing to rename the files. Let me read and write the next ones.
936: 
937: **Tool: read**
938: 
939: **Input:**
940: ```json
941: {
942:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/cli_tests.rs"
943: }
944: ```
945: 
946: **Output:**
947: ```
948: <path>/Users/apalala/art/tiexiu/wip/tests/cli_tests.rs</path>
949: <type>file</type>
950: <content>
951: 1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
952: 2: // SPDX-License-Identifier: MIT OR Apache-2.0
953: 3: 
954: 4: //! Tests translated from TatSu's cli_test.py
955: 5: //!
956: 6: //! These are skeletal tests - they will not compile until TieXiu
957: 7: //! implements the full EBNF parsing bootstrap.
958: 8: 
959: 9: use std::process::Command;
960: 10: 
961: 11: #[test]
962: 12: fn test_cli_help() {
963: 13:     let output = Command::new("tatsu")
964: 14:         .arg("--help")
965: 15:         .output()
966: 16:         .expect("Failed to run tatsu");
967: 17:     let stdout = String::from_utf8_lossy(&output.stdout);
968: 18:     let pattern = regex::Regex::new(r"(?ms)竜TatSu takes a grammar .*GRAMMAR").unwrap();
969: 19:     assert!(pattern.is_match(&stdout));
970: 20: }
971: 21: 
972: 22: #[test]
973: 23: fn test_cli_python() {
974: 24:     let output = Command::new("tatsu")
975: 25:         .arg("./grammar/tatsu.tatsu")
976: 26:         .output()
977: 27:         .expect("Failed to run tatsu");
978: 28:     let stdout = String::from_utf8_lossy(&output.stdout);
979: 29:     let pattern = regex::Regex::new(r"(?ms)CAVEAT UTILITOR.*?竜TatSu.*?KEYWORDS = \(").unwrap();
980: 30:     assert!(pattern.is_match(&stdout));
981: 31: }
982: 32: 
983: 33: #[test]
984: 34: fn test_cli_model() {
985: 35:     let output = Command::new("tatsu")
986: 36:         .arg("-g")
987: 37:         .arg("./grammar/tatsu.tatsu")
988: 38:         .output()
989: 39:         .expect("Failed to run tatsu");
990: 40:     let stdout = String::from_utf8_lossy(&output.stdout);
991: 41:     let pattern = regex::Regex::new(r"(?ms)CAVEAT UTILITOR.*?竜TatSu").unwrap();
992: 42:     assert!(pattern.is_match(&stdout));
993: 43: }
994: 
995: (End of file - total 43 lines)
996: </content>
997: ```
998: 
999: ---
1000: 
1001: ## Assistant (Build · Big Pickle · 5.2s)
1002: 
1003: _Thinking:_
1004: 
1005: The user said "Stop!" earlier and I should wait for them to continue. But I need to process all 40 files. Let me continue with the remaining files efficiently - I'll read and write multiple files in parallel to speed this up.
1006: 
1007: Actually, I should follow the user's instruction - they said "Do not use the bash tool" and I stopped. Then they said "Yes, update the mod.rs after you rename the modules" and I did that. Then they said "Now rename the files, one by one" and I've been doing that.
1008: 
1009: Let me continue one by one as they requested. I'll do cli.rs next.
1010: 
1011: **Tool: write**
1012: 
1013: **Input:**
1014: ```json
1015: {
1016:   "content": "// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)\n// SPDX-License-Identifier: MIT OR Apache-2.0\n\n//! Tests translated from TatSu's cli_test.py\n//!\n//! These are skeletal tests - they will not compile until TieXiu\n//! implements the full EBNF parsing bootstrap.\n\nuse std::process::Command;\n\n#[test]\nfn test_cli_help() {\n    let output = Command::new(\"tatsu\")\n        .arg(\"--help\")\n        .output()\n        .expect(\"Failed to run tatsu\");\n    let stdout = String::from_utf8_lossy(&output.stdout);\n    let pattern = regex::Regex::new(r\"(?ms)竜TatSu takes a grammar .*GRAMMAR\").unwrap();\n    assert!(pattern.is_match(&stdout));\n}\n\n#[test]\nfn test_cli_python() {\n    let output = Command::new(\"tatsu\")\n        .arg(\"./grammar/tatsu.tatsu\")\n        .output()\n        .expect(\"Failed to run tatsu\");\n    let stdout = String::from_utf8_lossy(&output.stdout);\n    let pattern = regex::Regex::new(r\"(?ms)CAVEAT UTILITOR.*?竜TatSu.*?KEYWORDS = \\(\").unwrap();\n    assert!(pattern.is_match(&stdout));\n}\n\n#[test]\nfn test_cli_model() {\n    let output = Command::new(\"tatsu\")\n        .arg(\"-g\")\n        .arg(\"./grammar/tatsu.tatsu\")\n        .output()\n        .expect(\"Failed to run tatsu\");\n    let stdout = String::from_utf8_lossy(&output.stdout);\n    let pattern = regex::Regex::new(r\"(?ms)CAVEAT UTILITOR.*?竜TatSu\").unwrap();\n    assert!(pattern.is_match(&stdout));\n}",
1017:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/cli.rs"
1018: }
1019: ```
1020: 
1021: **Output:**
1022: ```
1023: Wrote file successfully.
1024: ```
1025: 
1026: ---
1027: 
1028: ## Assistant (Build · Big Pickle · 1.9s)
1029: 
1030: _Thinking:_
1031: 
1032: Continuing with the remaining files. Let me read and write the next batch.
1033: 
1034: **Tool: read**
1035: 
1036: **Input:**
1037: ```json
1038: {
1039:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/codegen_tests.rs"
1040: }
1041: ```
1042: 
1043: **Output:**
1044: ```
1045: <path>/Users/apalala/art/tiexiu/wip/tests/codegen_tests.rs</path>
1046: <type>file</type>
1047: <content>
1048: 1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
1049: 2: // SPDX-License-Identifier: MIT OR Apache-2.0
1050: 3: 
1051: 4: //! Tests translated from TatSu's codegen_test.py
1052: 5: //!
1053: 6: //! These are skeletal tests - they will not compile until TieXiu
1054: 7: //! implements the full EBNF parsing bootstrap.
1055: 8: 
1056: 9: use crate::codegen::{CodeGenerator, ModelRenderer};
1057: 10: 
1058: 11: struct TestGenerator {
1059: 12:     rendered: String,
1060: 13: }
1061: 14: 
1062: 15: impl CodeGenerator for TestGenerator {
1063: 16:     fn render(&mut self, model: &dyn crate::model::Renderable) -> Result<String, String> {
1064: 17:         Ok(model.render())
1065: 18:     }
1066: 19: }
1067: 20: 
1068: 21: #[test]
1069: 22: fn test_basic_codegen() {
1070: 23:     struct Super;
1071: 24:     struct Sub;
1072: 25: 
1073: 26:     impl crate::model::Renderable for Super {
1074: 27:         fn render(&self) -> String {
1075: 28:             format!("OK {}", Sub.render())
1076: 29:         }
1077: 30:     }
1078: 31: 
1079: 32:     impl crate::model::Renderable for Sub {
1080: 33:         fn render(&self) -> String {
1081: 34:             "and OK too".to_string()
1082: 35:         }
1083: 36:     }
1084: 37: 
1085: 38:     let super_node = Super;
1086: 39:     let mut gen = TestGenerator {
1087: 40:         rendered: String::new(),
1088: 41:     };
1089: 42:     let result = gen.render(&super_node);
1090: 43:     assert_eq!(result.unwrap(), "OK and OK too");
1091: 44: }
1092: 
1093: (End of file - total 44 lines)
1094: </content>
1095: ```
1096: 
1097: ---
1098: 
1099: ## Assistant (Build · Big Pickle · 3.6s)
1100: 
1101: _Thinking:_
1102: 
1103: Continue renaming files.
1104: 
1105: **Tool: write**
1106: 
1107: **Input:**
1108: ```json
1109: {
1110:   "content": "// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)\n// SPDX-License-Identifier: MIT OR Apache-2.0\n\n//! Tests translated from TatSu's codegen_test.py\n//!\n//! These are skeletal tests - they will not compile until TieXiu\n//! implements the full EBNF parsing bootstrap.\n\nuse crate::codegen::{CodeGenerator, ModelRenderer};\n\nstruct TestGenerator {\n    rendered: String,\n}\n\nimpl CodeGenerator for TestGenerator {\n    fn render(&mut self, model: &dyn crate::model::Renderable) -> Result<String, String> {\n        Ok(model.render())\n    }\n}\n\n#[test]\nfn test_basic_codegen() {\n    struct Super;\n    struct Sub;\n\n    impl crate::model::Renderable for Super {\n        fn render(&self) -> String {\n            format!(\"OK {}\", Sub.render())\n        }\n    }\n\n    impl crate::model::Renderable for Sub {\n        fn render(&self) -> String {\n            \"and OK too\".to_string()\n        }\n    }\n\n    let super_node = Super;\n    let mut gen = TestGenerator {\n        rendered: String::new(),\n    };\n    let result = gen.render(&super_node);\n    assert_eq!(result.unwrap(), \"OK and OK too\");\n}",
1111:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/codegen.rs"
1112: }
1113: ```
1114: 
1115: **Output:**
1116: ```
1117: Wrote file successfully.
1118: ```
1119: 
1120: ---
1121: 
1122: ## Assistant (Build · Big Pickle · 2.9s)
1123: 
1124: _Thinking:_
1125: 
1126: I'll continue renaming the files. Next are constants_tests, defines_tests, diagram_tests, directive_tests, ebnf_tests, error_tests, firstfollow_tests, ideps_tests, join_tests, keyword_tests, left_recursion_tests, lookahead_tests, misc_tests, model_tests, ngmodel_tests, parameter_tests, parser_equivalence_tests, parsing_tests, pattern_tests, pickle_tests, pretty_tests, railroads_tests, regexp_tests, return_value_tests, safe_name_tests, safeeval_stress_tests, safeeval_tests, semantics_tests, string_tests, strtools_tests, syntax_tests, typetools_tests, walker_tests, z_bootstrap_tests.
1127: 
1128: Let me continue reading and writing each file.
1129: 
1130: **Tool: read**
1131: 
1132: **Input:**
1133: ```json
1134: {
1135:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/constants_tests.rs"
1136: }
1137: ```
1138: 
1139: **Output:**
1140: ```
1141: <path>/Users/apalala/art/tiexiu/wip/tests/constants_tests.rs</path>
1142: <type>file</type>
1143: <content>
1144: 1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
1145: 2: // SPDX-License-Identifier: MIT OR Apache-2.0
1146: 3: 
1147: 4: //! Tests translated from TatSu's grammar/constants_test.py
1148: 5: //!
1149: 6: //! These are skeletal tests - they will not compile until TieXiu
1150: 7: //! implements the full EBNF parsing bootstrap.
1151: 8: 
1152: 9: use crate::api::parse;
1153: 10: 
1154: 11: #[test]
1155: 12: fn test_constant_interpolation() {
1156: 13:     let grammar = r#"
1157: 14:         start = a:number b: number i:`"seen: {a}, {b}"` $ ;
1158: 15:         number = /\d+/ ;
1159: 16:     "#;
1160: 17: 
1161: 18:     let result = parse(grammar, "42 69");
1162: 19:     assert!(result.is_ok());
1163: 20: }
1164: 21: 
1165: 22: #[test]
1166: 23: fn test_constant_interpolation_free() {
1167: 24:     let grammar = r#"
1168: 25:         start = a:number b: number i:`seen: {a}, {b}` $ ;
1169: 26:         number = /\d+/ ;
1170: 27:     "#;
1171: 28: 
1172: 29:     let result = parse(grammar, "42 69");
1173: 30:     assert!(result.is_ok());
1174: 31: }
1175: 32: 
1176: 33: #[test]
1177: 34: fn test_constant_interpolation_multiline() {
1178: 35:     let grammar = r#"
1179: 36:         start = a:number b: number
1180: 37:         i:```
1181: 38:         seen:
1182: 39:         {a}
1183: 40:         {b}
1184: 41:         ``` $ ;
1185: 42:         number = /\d+/ ;
1186: 43:     "#;
1187: 44: 
1188: 45:     let result = parse(grammar, "42 69");
1189: 46:     assert!(result.is_ok());
1190: 47: }
1191: 48: 
1192: 49: #[test]
1193: 50: fn test_evaluate_constant() {
1194: 51:     let grammar = r#"
1195: 52:         @@grammar :: constants
1196: 53:         start = int bool str null 'a' $;
1197: 54: 
1198: 55:         int = `42` ;
1199: 56:         bool = `True` ;
1200: 57:         str = `Something` ;
1201: 58:         null = `None` ;
1202: 59:     "#;
1203: 60: 
1204: 61:     let ast = parse(grammar, "a");
1205: 62:     assert!(ast.is_ok());
1206: 63: }
1207: 
1208: (End of file - total 63 lines)
1209: </content>
1210: ```
1211: 
1212: ---
1213: 
1214: ## Assistant (Build · Big Pickle · 4.2s)
1215: 
1216: _Thinking:_
1217: 
1218: Let me continue reading and writing the rest of the files.
1219: 
1220: **Tool: write**
1221: 
1222: **Input:**
1223: ```json
1224: {
1225:   "content": "// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)\n// SPDX-License-Identifier: MIT OR Apache-2.0\n\n//! Tests translated from TatSu's grammar/constants_test.py\n//!\n//! These are skeletal tests - they will not compile until TieXiu\n//! implements the full EBNF parsing bootstrap.\n\nuse crate::api::parse;\n\n#[test]\nfn test_constant_interpolation() {\n    let grammar = r#\"\n        start = a:number b: number i:`\"seen: {a}, {b}\"` $ ;\n        number = /\\d+/ ;\n    \"#;\n\n    let result = parse(grammar, \"42 69\");\n    assert!(result.is_ok());\n}\n\n#[test]\nfn test_constant_interpolation_free() {\n    let grammar = r#\"\n        start = a:number b: number i:`seen: {a}, {b}` $ ;\n        number = /\\d+/ ;\n    \"#;\n\n    let result = parse(grammar, \"42 69\");\n    assert!(result.is_ok());\n}\n\n#[test]\nfn test_constant_interpolation_multiline() {\n    let grammar = r#\"\n        start = a:number b: number\n        i:```\n        seen:\n        {a}\n        {b}\n        ``` $ ;\n        number = /\\d+/ ;\n    \"#;\n\n    let result = parse(grammar, \"42 69\");\n    assert!(result.is_ok());\n}\n\n#[test]\nfn test_evaluate_constant() {\n    let grammar = r#\"\n        @@grammar :: constants\n        start = int bool str null 'a' $;\n\n        int = `42` ;\n        bool = `True` ;\n        str = `Something` ;\n        null = `None` ;\n    \"#;\n\n    let ast = parse(grammar, \"a\");\n    assert!(ast.is_ok());\n}",
1226:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/constants.rs"
1227: }
1228: ```
1229: 
1230: **Output:**
1231: ```
1232: Wrote file successfully.
1233: ```
1234: 
1235: ---
1236: 
1237: ## Assistant (Build · Big Pickle · 3.4s)
1238: 
1239: _Thinking:_
1240: 
1241: I'll continue renaming the files. Let me read the next batch of files.
1242: 
1243: **Tool: read**
1244: 
1245: **Input:**
1246: ```json
1247: {
1248:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/defines_tests.rs"
1249: }
1250: ```
1251: 
1252: **Output:**
1253: ```
1254: <path>/Users/apalala/art/tiexiu/wip/tests/defines_tests.rs</path>
1255: <type>file</type>
1256: <content>
1257: 1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
1258: 2: // SPDX-License-Identifier: MIT OR Apache-2.0
1259: 3: 
1260: 4: //! Tests translated from TatSu's grammar/defines_test.py
1261: 5: //!
1262: 6: //! These are skeletal tests - they will not compile until TieXiu
1263: 7: //! implements the full EBNF parsing bootstrap.
1264: 8: 
1265: 9: use crate::api::{compile, parse};
1266: 10: 
1267: 11: // ============================================================================
1268: 12: // Name in Option Tests
1269: 13: // ============================================================================
1270: 14: 
1271: 15: #[test]
1272: 16: fn test_name_in_option() {
1273: 17:     let grammar = r#"
1274: 18:         start = expr_range ;
1275: 19: 
1276: 20:         expr_range =
1277: 21:             | [from: expr] '..' [to: expr]
1278: 22:             | expr
1279: 23:             ;
1280: 24: 
1281: 25:         expr =
1282: 26:             /[\d]+/
1283: 27:         ;
1284: 28:     "#;
1285: 29: 
1286: 30:     let model = compile(grammar).expect("Failed to compile");
1287: 31: 
1288: 32:     let ast = model.parse("1 .. 10").expect("Failed to parse");
1289: 33:     let _ = ast;
1290: 34: 
1291: 35:     let ast = model.parse("10").expect("Failed to parse");
1292: 36:     let _ = ast;
1293: 37: 
1294: 38:     let ast = model.parse(" .. 10").expect("Failed to parse");
1295: 39:     let _ = ast;
1296: 40: 
1297: 41:     let ast = model.parse("1 .. ").expect("Failed to parse");
1298: 42:     let _ = ast;
1299: 43: 
1300: 44:     let ast = model.parse(" .. ").expect("Failed to parse");
1301: 45:     let _ = ast;
1302: 46: }
1303: 47: 
1304: 48: // ============================================================================
1305: 49: // By Option Tests
1306: 50: // ============================================================================
1307: 51: 
1308: 52: #[test]
1309: 53: fn test_by_option() {
1310: 54:     let grammar = r#"
1311: 55:         start = expr_range ;
1312: 56: 
1313: 57:         expr_range =
1314: 58:             | [from: expr] '..' [to: expr]
1315: 59:             | left:expr ','  [right:expr]
1316: 60:             ;
1317: 61: 
1318: 62:         expr =
1319: 63:             /[\d]+/
1320: 64:         ;
1321: 65:     "#;
1322: 66: 
1323: 67:     let model = compile(grammar).expect("Failed to compile");
1324: 68: 
1325: 69:     let ast = model.parse(" .. 10").expect("Failed to parse");
1326: 70:     let _ = ast;
1327: 71: 
1328: 72:     let ast = model.parse("1, 2").expect("Failed to parse");
1329: 73:     let _ = ast;
1330: 74: 
1331: 75:     let ast = model.parse("1, ").expect("Failed to parse");
1332: 76:     let _ = ast;
1333: 77: }
1334: 78: 
1335: 79: // ============================================================================
1336: 80: // Inner Options Tests
1337: 81: // ============================================================================
1338: 82: 
1339: 83: #[test]
1340: 84: fn test_inner_options() {
1341: 85:     let grammar = r#"
1342: 86:         start = switch;
1343: 87:         switch = 'switch' [(on:'on'|off:'off')] ;
1344: 88:     "#;
1345: 89: 
1346: 90:     let model = compile(grammar).expect("Failed to compile");
1347: 91: 
1348: 92:     let ast = model.parse("switch on").expect("Failed to parse");
1349: 93:     let _ = ast;
1350: 94: 
1351: 95:     let ast = model.parse("switch off").expect("Failed to parse");
1352: 96:     let _ = ast;
1353: 97: 
1354: 98:     let ast = model.parse("switch").expect("Failed to parse");
1355: 99:     let _ = ast;
1356: 100: }
1357: 
1358: (End of file - total 100 lines)
1359: </content>
1360: ```
1361: 
1362: ---
1363: 
1364: ## Assistant (Build · Big Pickle · 6.1s)
1365: 
1366: _Thinking:_
1367: 
1368: I'll continue renaming the files. This is defines_tests.rs -> defines.rs
1369: 
1370: **Tool: write**
1371: 
1372: **Input:**
1373: ```json
1374: {
1375:   "content": "// Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)\n// SPDX-License-Identifier: MIT OR Apache-2.0\n\n//! Tests translated from TatSu's grammar/defines_test.py\n//!\n//! These are skeletal tests - they will not compile until TieXiu\n//! implements the full EBNF parsing bootstrap.\n\nuse crate::api::{compile, parse};\n\n// ============================================================================\n// Name in Option Tests\n// ============================================================================\n\n#[test]\nfn test_name_in_option() {\n    let grammar = r#\"\n        start = expr_range ;\n\n        expr_range =\n            | [from: expr] '..' [to: expr]\n            | expr\n            ;\n\n        expr =\n            /[\\d]+/\n        ;\n    \"#;\n\n    let model = compile(grammar).expect(\"Failed to compile\");\n\n    let ast = model.parse(\"1 .. 10\").expect(\"Failed to parse\");\n    let _ = ast;\n\n    let ast = model.parse(\"10\").expect(\"Failed to parse\");\n    let _ = ast;\n\n    let ast = model.parse(\" .. 10\").expect(\"Failed to parse\");\n    let _ = ast;\n\n    let ast = model.parse(\"1 .. \").expect(\"Failed to parse\");\n    let _ = ast;\n\n    let ast = model.parse(\" .. \").expect(\"Failed to parse\");\n    let _ = ast;\n}\n\n// ============================================================================\n// By Option Tests\n// ============================================================================\n\n#[test]\nfn test_by_option() {\n    let grammar = r#\"\n        start = expr_range ;\n\n        expr_range =\n            | [from: expr] '..' [to: expr]\n            | left:expr ','  [right:expr]\n            ;\n\n        expr =\n            /[\\d]+/\n        ;\n    \"#;\n\n    let model = compile(grammar).expect(\"Failed to compile\");\n\n    let ast = model.parse(\" .. 10\").expect(\"Failed to parse\");\n    let _ = ast;\n\n    let ast = model.parse(\"1, 2\").expect(\"Failed to parse\");\n    let _ = ast;\n\n    let as... (line truncated to 2000 chars)
1376:   "filePath": "/Users/apalala/art/tiexiu/wip/tests/defines.rs"
1377: }
1378: ```
1379: 
1380: **Output:**
1381: ```
1382: Wrote file successfully.
1383: ```
1384: 
1385: ---
1386: 
1387: ## Assistant (Build · Big Pickle · 2.5s)
1388: 
1389: _Thinking:_
1390: 
1391: Continuing with the remaining files. Let me read and write the next ones.
1392: 
1393: **Tool: read**
1394: 
1395: **Input:**
1396: ```json
1397: {

(Output capped at 50 KB. Showing lines 1-1397. Use offset=1398 to continue.)
</content>

<system-reminder>
Instructions from: /Users/apalala/art/tiexiu/corpus/AGENTS.md
# AGENTS.md

## Project Overview

TieXiu is a Rust port of TatSu PEG parser generator with Python bindings via PyO3/maturin.

## Dev Commands

```bash
# Quality gates (used by `just pre-push`)
cargo fmt --all                          # format check
cargo clippy --all-targets --all-features -- -D warnings  # strict lint
cargo test --verbose                     # tests

# All checks in order (default target)
just pre-push

# Other commands
cargo build --release                    # release build
cargo bench                              # benchmarks
cargo run                                # run CLI (requires cli feature)
maturin develop                           # build & install Python package
```

## Testing

- Tests use `cargo test --verbose` (not `cargo nextest` locally; CI uses default)
- Benchmark harness is criterion-based with codspeed compat
- Test fixtures in `tests/fixtures.rs` provide `boot_grammar()` and `parse_ebnf()`

## Python Bindings

- Build system: maturin (configured in `pyproject.toml`)
- `.cargo/config.toml` sets `PYO3_PYTHON` to `.venv/bin/python` for local venv
- Create venv: `uv venv`
- Install package: `maturin develop`

## Architecture

- `src/api.rs`: High-level API (`parse`, `compile`, `load`, `pretty`)
- `src/peg/`: Grammar model (Exp, Grammar, Rule)
- `src/state/`: Parser context and memoization
- `src/input/`: Cursor trait for text/byte streams
- `src/trees/`: CST/AST tree types
- `src/json/`: TatSu JSON grammar import

## Key Conventions

- Rust edition 2024
- Features: `default`, `bootstrap`, `transient`, `serde`, `serde_json`, `cli`, `fancy-regex`
- Python requires >=3.12
- Use `unindent` utility when inlining EBNF strings in tests

## CI Workflows

- `default.yml`: fmt → clippy → test (sequential)
- `codspeed.yml`: benchmark on main/PR
- `publish.yml`: PyPI release

</system-reminder>
```

---

## Assistant (Build · Big Pickle · 5.4s)

_Thinking:_

The user wants me to study the files in ./corpus/. I've found:

1. `AGENTS.md` - Contains project info for TieXiu, a Rust PEG parser generator port of TatSu
2. `sessions/session-ses_28b7.md` - A session log showing a conversation about renaming test files from `*_tests.rs` to `*.rs` format

The corpus appears to contain documentation and session history for the TieXiu project.

The `./corpus/` directory contains:

1. **`AGENTS.md`** - Project documentation for TieXiu (Rust port of TatSu PEG parser generator)
2. **`sessions/session-ses_28b7.md`** - A session log from 4/9/2026 about renaming Rust test files from `*_tests.rs` to `*.rs` format

---

## User

Sumarize the contents of the files under ./corpus/sessions/ into a single corpus/SESSIONS.md

---

## Assistant (Build · Big Pickle · 8.4s)

_Thinking:_

The user wants me to summarize the session file into a SESSIONS.md file. I've already read the session file, so I can create the summary.

**Tool: write**

**Input:**
```json
{
  "content": "# Sessions Summary\n\n## session-ses_28b7 (4/9/2026)\n\n**Topic:** Rename Rust test files from `*_tests.rs` to `*.rs` pattern\n\n**Key Events:**\n\n1. Initial attempt to use `mv` via bash tools failed with `NotFound: ChildProcess.spawn` errors\n2. User instructed: \"Do not use the bash tools\"\n3. Assistant updated `wip/tests/mod.rs` to remove `_tests` suffix from 40 module declarations\n4. User requested files be renamed \"one by one\"\n5. Assistant used read/write approach to copy content to new filenames (without `_tests` suffix)\n\n**Files renamed (partial):**\n- `alerts_tests.rs` → `alerts.rs`\n- `asjson_tests.rs` → `asjson.rs`\n- `ast_tests.rs` → `ast.rs`\n- `bench_parse_tests.rs` → `bench_parse.rs`\n- `buffering_tests.rs` → `buffering.rs`\n- `cli_tests.rs` → `cli.rs`\n- `codegen_tests.rs` → `codegen.rs`\n- `constants_tests.rs` → `constants.rs`\n- `defines_tests.rs` → `defines.rs`\n- (and more...)\n\n**Note:** The write approach creates new files but leaves orphaned `_tests.rs` files. Manual deletion of old files required.\n",
  "filePath": "/Users/apalala/art/tiexiu/corpus/SESSIONS.md"
}
```

**Output:**
```
Wrote file successfully.
```

---

## Assistant (Build · Big Pickle · 2.2s)

_Thinking:_

Done. Created the SESSIONS.md file summarizing the session content.

Created `corpus/SESSIONS.md` summarizing the single session about renaming test files.

---

## User

Now in ./corpus/AGENTS.md add a rule that says that AI agents should study ./corpus/SESSIONS.md at the start of a session. 

---

## Assistant (Build · Big Pickle · 16.9s)

_Thinking:_

The user wants me to add a rule to AGENTS.md saying that AI agents should study ./corpus/SESSIONS.md at the start of a session. I already read this file earlier, so I can edit it.

**Tool: edit**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/corpus/AGENTS.md",
  "oldString": "## Project Overview\n\nTieXiu is a Rust port of TatSu PEG parser generator with Python bindings via PyO3/maturin.",
  "newString": "## Project Overview\n\nTieXiu is a Rust port of TatSu PEG parser generator with Python bindings via PyO3/maturin.\n\n## Session History\n\nAI agents should study `./corpus/SESSIONS.md` at the start of each session to learn from past work."
}
```

**Output:**
```
Edit applied successfully.
```

---

## Assistant (Build · Big Pickle · 2.2s)

_Thinking:_

Done. I added a "Session History" section to AGENTS.md with the rule that AI agents should study SESSIONS.md at the start of each session.

Added session history rule to `corpus/AGENTS.md`.

---

## User

Now study ./corpus/AGENTS.md and follow the rules in the Rules section

---

## Assistant (Build · Big Pickle · 4.4s)

_Thinking:_

The user wants me to study the AGENTS.md file and follow its rules. Let me read it to see what's in there, especially the Rules section.

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/corpus/AGENTS.md"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/corpus/AGENTS.md</path>
<type>file</type>
<content>
1: # AGENTS.md
2: 
3: ## Project Overview
4: 
5: TieXiu is a Rust port of TatSu PEG parser generator with Python bindings via PyO3/maturin.
6: 
7: ## Rules
8: 
9: - AI Agents will study `README.md`, `SYNTAX.md`, `./tatsu/README.rst` and all the `*.rst` documents in `./tatsu/docs/` to gain context about the project.
10: - AI Agents will study all the files belonging to the current Rust project to understand its structure and semantics.
11: - AI Agents will study the Python source code for TatSu found in the modules under `./tatsu/tatsu/` and `./tatsu/tests/` to understand the background of the TieXiu project and its goals. 
12: - AI Agents should study `./corpus/SESSIONS.md` at the start of each session to learn from past work.
13: 
14: ## Dev Commands
15: 
16: ```bash
17: # Quality gates (used by `just pre-push`)
18: cargo fmt --all                          # format check
19: cargo clippy --all-targets --all-features -- -D warnings  # strict lint
20: cargo test --verbose                     # tests
21: 
22: # All checks in order (default target)
23: just pre-push
24: 
25: # Other commands
26: cargo build --release                    # release build
27: cargo bench                              # benchmarks
28: cargo run                                # run CLI (requires cli feature)
29: maturin develop                           # build & install Python package
30: ```
31: 
32: ## Testing
33: 
34: - Tests use `cargo test --verbose` (not `cargo nextest` locally; CI uses default)
35: - Benchmark harness is criterion-based with codspeed compat
36: - Test fixtures in `tests/fixtures.rs` provide `boot_grammar()` and `parse_ebnf()`
37: 
38: ## Python Bindings
39: 
40: - Build system: maturin (configured in `pyproject.toml`)
41: - `.cargo/config.toml` sets `PYO3_PYTHON` to `.venv/bin/python` for local venv
42: - Create venv: `uv venv`
43: - Install package: `maturin develop`
44: 
45: ## Architecture
46: 
47: - `src/api.rs`: High-level API (`parse`, `compile`, `load`, `pretty`)
48: - `src/peg/`: Grammar model (Exp, Grammar, Rule)
49: - `src/state/`: Parser context and memoization
50: - `src/input/`: Cursor trait for text/byte streams
51: - `src/trees/`: CST/AST tree types
52: - `src/json/`: TatSu JSON grammar import
53: 
54: ## Key Conventions
55: 
56: - Rust edition 2024
57: - Features: `default`, `bootstrap`, `transient`, `serde`, `serde_json`, `cli`, `fancy-regex`
58: - Python requires >=3.12
59: - Use `unindent` utility when inlining EBNF strings in tests
60: 
61: ## CI Workflows
62: 
63: - `default.yml`: fmt → clippy → test (sequential)
64: - `codspeed.yml`: benchmark on main/PR
65: - `publish.yml`: PyPI release

(End of file - total 65 lines)
</content>
```

---

## Assistant (Build · Big Pickle · 5.3s)

_Thinking:_

The Rules section says:
1. Study `README.md`, `SYNTAX.md`, `./tatsu/README.rst` and all `*.rst` docs in `./tatsu/docs/`
2. Study all files in the current Rust project
3. Study Python source in `./tatsu/tatsu/` and `./tatsu/tests/`
4. Study `./corpus/SESSIONS.md`

I've already read `./corpus/SESSIONS.md` in this session. Let me also study the Rust project structure and the TatSu documentation to follow the rules.

Per the Rules section, I should study:
1. `README.md`, `SYNTAX.md`, `./tatsu/README.rst`, `./tatsu/docs/*.rst`
2. All Rust project files
3. `./tatsu/tatsu/` and `./tatsu/tests/` Python code
4. `./corpus/SESSIONS.md` ✓ (already read)

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/README.md"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/README.md</path>
<type>file</type>
<content>
1: [![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/neogeny/TieXiu?utm_source=badge)
2: # 铁修 TieXiu
3: 
4: A high-performance port of **TatSu** to Rust.
5: 
6: **TieXiu** (铁修) is a PEG (Parsing Expression Grammar) engine that implements the flexibility and power of the original **[TatSu][]** lineage into a memory-safe, high-concurrency architecture optimized for modern CPU caches.
7: 
8: [TatSu]: https://tatsu.readthedocs.io/en/stable/
9: 
10: ## Roadmap
11: 
12: The [TatSu Documentation][] provides a vision of where the **TieXiu** projects is heading. A copy of the grammar syntax can can be accessed locally in the [SYNTAX](SYNTAX.md) document.
13: 
14: **TatSu** is a mature project with an important user base so it's difficult to make certain changes even if they are improvements or fixes for long-standing quirks (as well known within experienced software engineers, a long-lived quirk becomes a feature). **TieXiu** is an opportunity to start from scratch, with a modern approach, even if the grammar syntax and its semantics are preserved.
15: 
16: [TatSu Documentation]: https://tatsu.readthedocs.io/
17: 
18: ## Current Project Status: Alpha
19: 
20: Implementation is complete for the core execution engine, the grammar model, the parsing engine, and support for left-recursive grammars. Development is currently advancing toward high-level tree construction and model synthesis.
21: 
22: ### Lean Parsing Context
23: 
24: **TieXiu** uses the runtime stack as the parsing state stack. A state
25: change has a 16-byte stack footprint consisting of two  pointers: one for the *Volatile State* (Input Cursor) and one for the *Heavy Stated* (Grammar + Memoization Cache). This allows for deep recursive descent with
26: minimal stack pressure and $O(1)$ branching costs. The `Cursor`
27: implementation for parsing text (`StrCursor`) uses 16 bytes (`&str` +
28: `usize`) and has copy-on-write semantics during a parse (grammar elements
29: that don't advance over the input share the same cursor).
30: 
31: 
32: ### Copy-on-Write State Transitios
33: 
34: Backtracking in **TieXiu** is *lazy*. Cloning a context/state only increments reference counts. The engine leverages the Rust runtime stack to handle state changes. Branching at choice points is a *16-byte* clone of the state, with a *16-byte* (for text) allocation only when the state is mutated. Failed parses restore the cursor position and register the furthest failure position for error reporting. The state returned occupies the same space as the original state.
35: 
36: A CST may use *64-bytes* per atomic node plus space proportional to the input matched, but CST are only kept for the currently successful path on the parse, and are dropped as soon as an option fails. CST are compacted on the boundary of the successful parse of a grammar rule node.
37: 
38: The failure path returns the furthest position reached in the input and a message about the error encountered there. The same error value is passed back during backtracking until a branch point is reached and another path can be tried. Atr branching the error value belonging to the furthes position in the input is chosen to pass back. The error value also passes back the _cut_ state so branches can commit to a failed alternative if it was fixed with a cut.
39: 
40: ### Complete Parsing Engine
41: 
42: The core parsing logic is fully implemented, providing a robust execution environment for PEG rules. Choice points, sequences, and lookaheads are handled with high efficiency, leveraging the CoW context for seamless backtracking.
43: 
44: ### Left Recursion Support
45: 
46: TieXiu features a complete implementation for handling left-recursive
47: grammars. A pre-pass *analysis* identifies and marks recursive cycles, while the *runtime* includes the necessary logic to grow the recursive content by iteration instead of recursion.
48: 
49: ### Complete Grammar Model
50: 
51: The building blocks for grammar models are implemented with a clear chain of ownership. The `Grammar` acts as the root container owning the `Rule` map, while each `Rule` owns its `Model` definition. This hierarchy eliminates reference proliferation and simplifies lifetime management.
52: 
53: ### Milestone: From CST to AST
54: 
55:  The algebra for creating **Concrete Syntax Trees (CST)** was ported from
56:  **TatSu** to **TieXiu** with optimizations. Instead of computing the
57:  resulting CST during parsing, the engine generates unoptimized trees that
58:  are destilled into their concrete versions at rule boundaries. **TieXiu**
59:  uses the **TatSu** semantics for **Abstract Syntax Tree (AST)**, in which
60:  named elements in a rule definition force the result to be a mapping of
61:  names to parsed elements. Rust doesn't allow the creation of synthetic
62:  types at runtime, so parsing to native types will require code generation
63:  for the desired model and deserialization of the JSON-compatible result of
64:  a parse into the desired model nodes.
65: 
66: ### Packrat & Memoization
67: 
68: All branches in a parse use a shared *Memoization Cache* to achieve the `O(N) ` complexity of packrat parsers. The cache is pruned at `cut` points to place a bound on memory use and make the lookups more efficient.
69: 
70: ### The Bootstrap Plan
71: 
72: A critical upcoming milestone is the **Bootstrap Process**. The roadmap
73: includes **Self-Hosting** through the implementation of a **TieXiu** grammar that describes its own EBNF. Grammars, including the grammar for grammars, will be passed to **TieXiu** using **TatSu**'s JSON export format for deseriallization into models.
74: 
75: After the initial bootstrap (**TieXiu** parsing grammars in its own grammar
76: language) either **TieXiu** or **TatSu** will generate the Rust code
77: necessary for faster parser bootstrap.
78: 
79: At the moment **TieXiu** is capable of reading the JSON representation of grammars that **TatSu** generates. In a two-step process the JSON is read and parsed to a model close to its structure. In a second step the intermediate model is translated to a grammar model that can be use for parsing.
80: 
81: * [x] The model pretty-prints to the original grammar text in TatSu-compatibleEBNF
82: * [x] **TieXiu** is capable of parsing the JSON representation of the **TatSu** EBNF grammar language
83: * [ ] **TieXiu** should be capable of parsing the EBNF text for the **TatSu** grammar language
84: * [ ] Any EBNF grammar should be parsed by the tool and applied to input in the language described by the gramar
85: 
86: ## Features
87: 
88: * [x] **16-byte Handle Size**: Optimized for L1 cache and deep recursion.
89: * [x] **32-byte State Changes**: Efficient CoW allocation on the Rust runtime stack.
90: * [x] **Complete Parsing Engine**: Core PEG execution logic is fully implemented.
91: * [x] **Left Recursion**: Both analysis and runtime support are complete.
92: * [x] **Complete Grammar Model**: Rules and Models are fully defined and owned.
93: * [x] **Thread-Safe Grammar**: Static grammar structure can be shared across multiple threads.
94: * [x] **Efficient Memoization**: Global cache consistency across backtracking branches.
95: * [x] **Object-Safe Cursors**: Abstract `Cursor` trait allows for string, byte, or custom stream inputs.
96: * [ ] **Self-Hosting Bootstrap**: (Planned) Engine parsing its own EBNF.
97: 
98: ## License
99: 
100: Licensed under either of
101: 
102: * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
103: * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
104: 
105: at your option.
106: 
107: ### Contribution
108: 
109: Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.
110: 
111: ## Grammar Syntax
112: 
113: ```console
114: 
115:     start ●─grammar─■
116: 
117:     grammar[Grammar] ●─ [title](`TATSU`)──┬→───────────────────────────────────┬── [`rules`]+(rule)──┬→───────────────────────────────┬──⇥＄
118:                                           ├→──┬─ [`directives`]+(directive)─┬──┤                     ├→──┬─ [`rules`]+(rule)───────┬──┤
119:                                           │   └─ [`keywords`]+(keyword)─────┘  │                     │   └─ [`keywords`]+(keyword)─┘  │
120:                                           └───────────────────────────────────<┘                     └───────────────────────────────<┘
121: 
122:     directive ●─'@@'─ !['keyword'] ✂ ───┬─ [name](──┬─'comments'─────┬─) ✂ ─'::' ✂ ─ [value](regex)────────────┬─ ✂ ──■
123:                                         │           └─'eol_comments'─┘                                         │
124:                                         ├─ [name]('whitespace') ✂ ─'::' ✂ ─ [value](──┬─regex───┬─)────────────┤
125:                                         │                                             ├─string──┤              │
126:                                         │                                             ├─'None'──┤              │
127:                                         │                                             ├─'False'─┤              │
128:                                         │                                             └─`None`──┘              │
129:                                         ├─ [name](──┬─'nameguard'──────┬─) ✂ ───┬─'::' ✂ ─ [value](boolean)─┬──┤
130:                                         │           ├─'ignorecase'─────┤        └─ [value](`True`)──────────┘  │
131:                                         │           ├─'left_recursion'─┤                                       │
132:                                         │           ├─'parseinfo'──────┤                                       │
133:                                         │           └─'memoization'────┘                                       │
134:                                         ├─ [name]('grammar') ✂ ─'::' ✂ ─ [value](word)─────────────────────────┤
135:                                         └─ [name]('namechars') ✂ ─'::' ✂ ─ [value](string)─────────────────────┘
136: 
137:     keywords ●───┬─keywords─┬───■
138:                  └─────────<┘
139: 
140:     keyword ●─'@@keyword' ✂ ─'::' ✂ ───┬→──────────────────────────────────┬───■
141:                                        ├→ @+(──┬─word───┬─)─ ![──┬─':'─┬─]─┤
142:                                        │       └─string─┘        └─'='─┘   │
143:                                        └──────────────────────────────────<┘
144: 
145:     the_params_at_last ●───┬─ [kwparams](kwparams)─────────────────────────┬──■
146:                            ├─ [params](params)',' ✂ ─ [kwparams](kwparams)─┤
147:                            └─ [params](params)─────────────────────────────┘
148: 
149:     paramdef ●───┬─'[' ✂ ─ >(the_params_at_last) ']'─┬──■
150:                  ├─'(' ✂ ─ >(the_params_at_last) ')'─┤
151:                  └─'::' ✂ ─ [params](params)─────────┘
152: 
153:     rule[Rule] ●─ [decorators](──┬→──────────┬──) [name](name) ✂ ───┬─→ >(paramdef) ─┬───┬─→'<' ✂ ─ [base](known_name)─┬───┬─'='──┬─ ✂ ─ [exp](expre)ENDRULE ✂ ──■
154:                                  ├→decorator─┤                      └─→──────────────┘   └─→───────────────────────────┘   ├─':='─┤
155:                                  └──────────<┘                                                                             └─':'──┘
156: 
157:     ENDRULE ●───┬── &[UNINDENTED]──────┬──■
158:                 ├─EMPTYLINE──┬─→';'─┬──┤
159:                 │            └─→────┘  │
160:                 ├─⇥＄                  │
161:                 └─';'──────────────────┘
162: 
163:     UNINDENTED ●─/(?=\s*(?:\r?\n|\r)[^\s])/──■
164: 
165:     EMPTYLINE ●─/(?:\s*(?:\r?\n|\r)){2,}/──■
166: 
167:     decorator ●─'@'─ !['@'] ✂ ─ @(──┬─'override'─┬─)─■
168:                                     ├─'name'─────┤
169:                                     └─'nomemo'───┘
170: 
171:     params ●─ @+(first_param)──┬→────────────────────────────┬───■
172:                                ├→',' @+(literal)─ !['='] ✂ ──┤
173:                                └────────────────────────────<┘
174: 
175:     first_param ●───┬─path────┬──■
176:                     └─literal─┘
177: 
178:     kwparams ●───┬→────────────┬───■
179:                  ├→',' ✂ ─pair─┤
180:                  └────────────<┘
181: 
182:     pair ●─ @+(word)'=' ✂ ─ @+(literal)─■
183: 
184:     expre ●───┬─choice───┬──■
185:               └─sequence─┘
186: 
187:     choice[Choice] ●───┬─→'|' ✂ ──┬─ @+(option)──┬─'|' ✂ ─ @+(option)─┬───■
188:                        └─→────────┘              └───────────────────<┘
189: 
190:     option[Option] ●─ @(sequence)─■
191: 
192:     sequence[Sequence] ●───┬── &[element',']──┬→───────────────┬───┬──■
193:                            │                  ├→',' ✂ ─element─┤   │
194:                            │                  └───────────────<┘   │
195:                            └───┬── ![ENDRULE]element─┬─────────────┘
196:                                └────────────────────<┘
197: 
198:     element ●───┬─rule_include─┬──■
199:                 ├─named────────┤
200:                 ├─override─────┤
201:                 └─term─────────┘
202: 
203:     rule_include[RuleInclude] ●─'>' ✂ ─ @(known_name)─■
204: 
205:     named ●───┬─named_list───┬──■
206:               └─named_single─┘
207: 
208:     named_list[NamedList] ●─ [name](name)'+:' ✂ ─ [exp](term)─■
209: 
210:     named_single[Named] ●─ [name](name)':' ✂ ─ [exp](term)─■
211: 
212:     override ●───┬─override_list──────────────┬──■
213:                  ├─override_single────────────┤
214:                  └─override_single_deprecated─┘
215: 
216:     override_list[OverrideList] ●─'@+:' ✂ ─ @(term)─■
217: 
218:     override_single[Override] ●─'@:' ✂ ─ @(term)─■
219: 
220:     override_single_deprecated[Override] ●─'@' ✂ ─ @(term)─■
221: 
222:     term ●───┬─void───────────────┬──■
223:              ├─gather─────────────┤
224:              ├─join───────────────┤
225:              ├─left_join──────────┤
226:              ├─right_join─────────┤
227:              ├─empty_closure──────┤
228:              ├─positive_closure───┤
229:              ├─closure────────────┤
230:              ├─optional───────────┤
231:              ├─skip_to────────────┤
232:              ├─lookahead──────────┤
233:              ├─negative_lookahead─┤
234:              ├─cut────────────────┤
235:              ├─cut_deprecated─────┤
236:              └─atom───────────────┘
237: 
238:     group[Group] ●─'(' ✂ ─ @(expre)')' ✂ ──■
239: 
240:     gather ●── &[atom'.{'] ✂ ───┬─positive_gather─┬──■
241:                                 └─normal_gather───┘
242: 
243:     positive_gather[PositiveGather] ●─ [sep](atom)'.{' [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
244:                                                                         └─'-'─┘
245: 
246:     normal_gather[Gather] ●─ [sep](atom)'.{' ✂ ─ [exp](expre)'}'──┬─→'*' ✂ ──┬─ ✂ ──■
247:                                                                   └─→────────┘
248: 
249:     join ●── &[atom'%{'] ✂ ───┬─positive_join─┬──■
250:                               └─normal_join───┘
251: 
252:     positive_join[PositiveJoin] ●─ [sep](atom)'%{' [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
253:                                                                     └─'-'─┘
254: 
255:     normal_join[Join] ●─ [sep](atom)'%{' ✂ ─ [exp](expre)'}'──┬─→'*' ✂ ──┬─ ✂ ──■
256:                                                               └─→────────┘
257: 
258:     left_join[LeftJoin] ●─ [sep](atom)'<{' ✂ ─ [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
259:                                                                 └─'-'─┘
260: 
261:     right_join[RightJoin] ●─ [sep](atom)'>{' ✂ ─ [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
262:                                                                   └─'-'─┘
263: 
264:     positive_closure[PositiveClosure] ●───┬─'{' @(expre)'}'──┬─'-'─┬─ ✂ ──┬──■
265:                                           │                  └─'+'─┘      │
266:                                           └─ @(atom)'+' ✂ ────────────────┘
267: 
268:     closure[Closure] ●───┬─'{' @(expre)'}'──┬─→'*'─┬─ ✂ ──┬──■
269:                          │                  └─→────┘      │
270:                          └─ @(atom)'*' ✂ ─────────────────┘
271: 
272:     empty_closure[EmptyClosure] ●─'{}' ✂ ─ @( ∅ )─■
273: 
274:     optional[Optional] ●───┬─'[' ✂ ─ @(expre)']' ✂ ──────────┬──■
275:                            └─ @(atom)─ ![──┬─'?"'─┬─]'?' ✂ ──┘
276:                                            ├─"?'"─┤
277:                                            └─'?/'─┘
278: 
279:     lookahead[Lookahead] ●─'&' ✂ ─ @(term)─■
280: 
281:     negative_lookahead[NegativeLookahead] ●─'!' ✂ ─ @(term)─■
282: 
283:     skip_to[SkipTo] ●─'->' ✂ ─ @(term)─■
284: 
285:     atom ●───┬─group────┬──■
286:              ├─token────┤
287:              ├─alert────┤
288:              ├─constant─┤
289:              ├─call─────┤
290:              ├─pattern──┤
291:              ├─dot──────┤
292:              └─eof──────┘
293: 
294:     call[Call] ●─word─■
295: 
296:     void[Void] ●─'()' ✂ ──■
297: 
298:     fail[Fail] ●─'!()' ✂ ──■
299: 
300:     cut[Cut] ●─'~' ✂ ──■
301: 
302:     cut_deprecated[Cut] ●─'>>' ✂ ──■
303: 
304:     known_name ●─name ✂ ──■
305: 
306:     name ●─word─■
307: 
308:     constant[Constant] ●── &['`']──┬─/(?ms)```((?:.|\n)*?)```/──┬──■
309:                                    ├─'`' @(literal)'`'──────────┤
310:                                    └─/`(.*?)`/──────────────────┘
311: 
312:     alert[Alert] ●─ [level](/\^+/─) [message](constant)─■
313: 
314:     token[Token] ●───┬─string─────┬──■
315:                      └─raw_string─┘
316: 
317:     literal ●───┬─string─────┬──■
318:                 ├─raw_string─┤
319:                 ├─boolean────┤
320:                 ├─word───────┤
321:                 ├─hex────────┤
322:                 ├─float──────┤
323:                 ├─int────────┤
324:                 └─null───────┘
325: 
326:     string ●─STRING─■
327: 
328:     raw_string ●─//─ @(STRING)─■
329: 
330:     STRING ●───┬─ @(/"((?:[^"\n]|\\"|\\\\)*?)"/─) ✂ ─────┬──■
331:                └─ @(/r"'((?:[^'\n]|\\'|\\\\)*?)'"/─) ✂ ──┘
332: 
333:     hex ●─/0[xX](?:\d|[a-fA-F])+/──■
334: 
335:     float ●─/[-+]?(?:\d+\.\d*|\d*\.\d+)(?:[Ee][-+]?\d+)?/──■
336: 
337:     int ●─/[-+]?\d+/──■
338: 
339:     path ●─/(?!\d)\w+(?:::(?!\d)\w+)+/──■
340: 
341:     word ●─/(?!\d)\w+/──■
342: 
343:     dot[Dot] ●─'/./'─■
344: 
345:     pattern[Pattern] ●─regexes─■
346: 
347:     regexes ●───┬→─────────────┬───■
348:                 ├→'+' ✂ ─regex─┤
349:                 └─────────────<┘
350: 
351:     regex ●───┬─'/' ✂ ─ @(/(?:[^/\\]|\\/|\\.)*/─)'/' ✂ ──┬──■
352:               ├─'?' @(STRING)────────────────────────────┤
353:               └─deprecated_regex─────────────────────────┘
354: 
355:     deprecated_regex ●─'?/' ✂ ─ @(/(?:.|\n)*?(?=/\?)/─)//\?+/─ ✂ ──■
356: 
357:     boolean ●───┬─'True'──┬──■
358:                 └─'False'─┘
359: 
360:     null ●─'None'─■
361: 
362:     eof[EOF] ●─'$' ✂ ──■
363:     
364: ```

(End of file - total 364 lines)
</content>
```

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/SYNTAX.md"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/SYNTAX.md</path>
<type>file</type>
<content>
1: orphan
2: 
3: :   
4: 
5: # Grammar Syntax
6: 
7: **铁修 TieXiu** grammars use an extension of
8: the classic
9: [EBNF](https://en.wikipedia.org/wiki/Extended_Backus–Naur_form) syntax.
10: The classic variations of
11: [EBNF](https://en.wikipedia.org/wiki/Extended_Backus–Naur_form)
12: (Tomassetti, EasyExtend, Wirth) and [ISO
13: EBNF](https://en.wikipedia.org/wiki/Extended_Backus–Naur_form) are also
14: supported as input grammar format.
15: 
16: ## Rules
17: 
18: A grammar consists of a sequence of one or more rules of the form:
19: 
20: ```ebnf
21: rulename: <expre>
22: ```
23: 
24: If a *name* collides with a [Python](http://python.org) keyword or
25: builtin, an underscore (`_`) will be appended to it on the generated
26: parser.
27: 
28: Rule names that start with an uppercase character:
29: 
30: ```ebnf
31: FRAGMENT: /[a-z]+/
32: ```
33: 
34: *do not* advance over whitespace or comments before beginning to parse. This feature
35: becomes handy when defining complex lexical elements, as it allows
36: breaking them into more than one rule.
37: 
38: The parser returns an
39: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) value for each
40: rule depending on what was parsed:
41: 
42: - A single value
43: - A list of [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree)
44: - A mapping for rules with named elements
45: - An node with metadata
46: 
47: This is the `enum Tree` that defines the posible results of a parse:
48: 
49: ```rust
50: pub enum Tree {
51:     Void,                          // The result of the () expression
52:     Text(Box<str>),                // Tokens or patterns
53:     List(Box<[Tree]>),             // Sequences of expressions
54:     Map(Box<TreeMap>),             // A mapping of named elements
55: 
56:     Node {                         // The result of parsing a rule
57:         meta: NodeMetaRef,         // Metadata
58:         tree: Box<Tree>            // The result of parsing a rule
59:     },
60:     // ...
61: ```
62: 
63: 
64: ## Expressions
65: 
66: The expressions, in reverse order of operator precedence, can be any of
67: the following.
68: 
69: > **note**
70: > 
71: > Because **TieXiu** now supports free-form [EBNF][], there must not be empty lines within expressions.
72: 
73: [EBNF]: https://en.wikipedia.org/wiki/Extended_Backus–Naur_form
74: 
75: 
76: ### `# comment ...`
77: 
78: [Python](http://python.org)-style end-of-line comments are allowed.
79: 
80: ### `// comment ...`
81: 
82: [Java](https://en.wikipedia.org/wiki/Java)-style end-of-line comments
83: are allowed.
84: 
85: ### `/* ... */`
86: 
87: [EBNF](https://en.wikipedia.org/wiki/Extended_Backus–Naur_form)-style
88: multi-line comments are allowed.
89: 
90: ### `e1 | e2`
91: 
92: Choice. Match either `e1` or `e2`.
93: 
94: A `|` may be used before the first option if desired:
95: 
96: ```ebnf
97: choices:
98:   | e1
99:   | e2
100:   | e3
101: ```
102: 
103: ### `e1 e2`
104: 
105: Sequence. Match `e1` and then match `e2`.
106: 
107: ### `( e )`
108: 
109: Grouping. Match `e`. For example: `('a' | 'b')`.
110: 
111: ### `(?: e )`
112: 
113: A non-capturing group. Like in a `()` group, match `e`, but this time do
114: not capture what was parsed. For example:
115: 
116: ``` {.apl force=""}
117: start: header (?: delimiter ) body
118: 
119: header: /[A-Z]+/
120: 
121: delimiter: /[:,-]+/
122: 
123: body: /[a-z]+/
124: ```
125: 
126: ### `[ e ]`
127: 
128: Optionally match `e`.
129: 
130: ### `{ e }` or `{ e }*`
131: 
132: Closure. Match `e` zero or more times. The
133: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) returned for a
134: closure is always a `list`.
135: 
136: ### `{ e }+`
137: 
138: Positive closure. Match `e` one or more times. The
139: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) is always a
140: `list`.
141: 
142: ### `{}`
143: 
144: Empty closure. Match nothing and produce an empty `list` as
145: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree).
146: 
147: ### `~`
148: 
149: The *cut* expression: commit to the current option and prevent other
150: options from being considered even when what follows fails to parse.
151: 
152: In this example, other options won\'t be considered if a parenthesis is
153: parsed:
154: 
155: ```ebnf
156: atom:
157:     | '(' ~ @:expre ')'
158:     | int
159:     | bool
160: ```
161: 
162: Cut expression may be used anywhere. The effect of `~` is scoped to the
163: nearest enclosing brackets (*group*, *optional*, *closure*), the
164: enclosing *choice*, or the enclosing *rule*.
165: 
166: On the scoping of *cut*, consider these theoretical equivalences about
167: implicit choices in some expressions:
168: 
169: ``` {.apl force=""}
170: A →  α   ≡ A → α | ⊥
171: A → [x]  ≡ A → B, B → x | ε
172: A → {x}  ≡ A → B, B → xB | ε
173: A → {x}+ ≡ A → B, B → xB | x
174: ```
175: 
176: This is a common use of `~`. The *closure* doesn\'t allow a partial
177: assignment expressions to escape it:
178: 
179: ```ebnf
180: parameters: ','.{name '=' ~ expression}
181: ```
182: 
183: ### `s%{ e }+`
184: 
185: Positive join. Inspired by [Python](http://python.org)\'s `str.join()`,
186: it parses the same as this expression:
187: 
188: ```ebnf
189: e {s ~ e}
190: ```
191: 
192: yet the result is a single list of the form:
193: 
194: ```ebnf
195: [e, s, e, s, e, ...]
196: ```
197: 
198: Use grouping if [s]{.title-ref} is more complex than a *token* or a
199: *pattern*:
200: 
201: ```ebnf
202: (s t)%{ e }+
203: ```
204: 
205: ### `s%{ e }` or `s%{ e }*`
206: 
207: Join. Parses the list of `s`-separated expressions, or the empty
208: closure. It\'s equivalent to:
209: 
210: ```ebnf
211: s%{e}+|{}
212: ```
213: 
214: ### `s.{ e }+`
215: 
216: Positive *gather*. Like *positive join*, but the separator is not
217: included in the resulting
218: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree).
219: 
220: ### `s.{ e }` or `s.{ e }*`
221: 
222: *Gather*. Like the *join*, but the separator is not included in the
223: resulting [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree).
224: It\'s equivalent to:
225: 
226: ```ebnf
227: s.{e}+|{}
228: ```
229: 
230: ### `&e`
231: 
232: Positive lookahead. Succeed if `e` can be parsed, but do not consume any
233: input.
234: 
235: ### `!e`
236: 
237: Negative lookahead. Fail if `e` can be parsed, and do not consume any
238: input.
239: 
240: ### `'text'` or `"text"`
241: 
242: Match the token *text* within the quotation marks.
243: 
244: Note that if *text* is alphanumeric, then **TieXiu** will check that the character following the token is not alphanumeric. This is done to prevent tokens like *IN* matching when the text ahead is *INITIALIZE*. This feature can be turned off by passing `nameguard=False` to the `Parser` or the `Buffer`, or by using a pattern expression (see below) instead of a token expression. The `@@nameguard` and `@@namechars` directives may be specified in the grammar for the same effect:
245: 
246: ```ebnf
247: @@nameguard :: False
248: ```
249: 
250: or to specify additional characters that should also be considered part
251: of names:
252: 
253: ```ebnf
254: @@namechars :: '$-.'
255: ```
256: 
257: ### `r'text'` or `r"text"`
258: 
259: Match the token *text* within the triple quotation marks, interpreting
260: *text* like [Python](http://python.org)\'s [raw string
261: literal](https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals)s.
262: This is useful
263: 
264: ### `'''text'''` or `"""text"""`
265: 
266: A multi-line version of `token`. The text within the triple quotation
267: marks is stripped of trailing whitespace, and the common indentation is
268: removed as to help with pretty formatting grammars without having to
269: worry about indentation. The resulting text is matched as a token.
270: 
271: ### `/regexp/`
272: 
273: Also `?"regexp"` or `?'regexp'`, The *pattern* expression. Match the
274: [Python](http://python.org) regular expression `regexp` at the current
275: text position. Unlike other expressions, this one does not advance over
276: whitespace or comments. For that, place the `regexp` as the only term in
277: its own rule.
278: 
279: The *regex* is interpreted as a [Python](http://python.org) [raw string
280: literal](https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals)
281: and passed the [Python](http://python.org)
282: [re](https://docs.python.org/3.4/library/re.html) module using `match()`
283: at the current position in the text. The returned
284: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) has the
285: semantics of `re.findall(pattern, text)[0]` (a [tuple]{.title-ref} if
286: there is more than one group), so use `(?:)` for groups that should not
287: be in the resulting
288: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree).
289: 
290: Consecutive *patterns* are concatenated to form a single one.
291: 
292: ### `/./`
293: 
294: The *any* expression, matches the next position in the input. It works
295: exactly like the `?'.'` pattern, but is implemented at the lexical
296: level, without regular expressions.
297: 
298: ### `->e`
299: 
300: The \"\*skip to\*\" expression, useful for writing *recovery* rules.
301: 
302: The parser will advance over input, one character at time, until `e`
303: matches. Whitespace and comments will be skipped at each step. Advancing
304: over input is done efficiently, with no regular expressions involved.
305: 
306: The expression is equivalent to:
307: 
308: ```ebnf
309: { !e /./ } e
310: ```
311: 
312: A common form of the expression is `->&e`, which is equivalent to:
313: 
314: ```ebnf
315: { !e /./ } &e
316: ```
317: 
318: This is an example of the use of the \"\*skip to\*\" expression for
319: recovery:
320: 
321: ```ebnf
322: statement:
323:     | if_statement
324:     # ...
325: 
326: if_statement:
327:     | 'if' condition 'then' statement ['else' statement]
328:     | 'if' statement_recovery
329: 
330: statement_recovery: ->&statement
331: ```
332: 
333: ### `` `constant ``\`
334: 
335: Match nothing, but behave as if `constant` had been parsed.
336: 
337: Constants can be used to inject elements into the concrete and abstract
338: syntax trees, perhaps avoiding having to write a semantic action. For
339: example:
340: 
341: ```ebnf
342: boolean_option: name ['=' (boolean|`true`) ]
343: ```
344: 
345: If the text evaluates to a Python literal (with `ast.literal_eval()`),
346: that will be the returned value. Otherwise, string interpolation in the
347: style of `str.format()` over the names in the current
348: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) is applied for
349: *constant* elements. Occurrences of the `{` character must be escaped to
350: `\{` if they are not intended for interpolation. A *constant* expression
351: that hast type `str` is evaluated using:
352: 
353: ``` {.python force=""}
354: eval(f'{"f" + repr(text)}', {}, ast)
355: ```
356: 
357: ### ``[constant]{.title-ref}``
358: 
359: A multi-line version of `` `constant ``\`.
360: 
361: ### `` ^`constant ``\`
362: 
363: Also `^`[constant]{.title-ref}``. An alert. There will be no token
364: returned by the parser, but an alert will be registed in the parse
365: context and added to the current node\'s `parseinfo`.
366: 
367: The `^` character may appear more than once to encode the *alert level*:
368: 
369: ```ebnf
370: assignment: identifier '=' (
371:     | value
372:     | -> &';' ^^^`could not parse value in assignment to {identifier}`
373: )
374: ```
375: 
376: ### `rulename`
377: 
378: Invoke the rule named `rulename`. To help with lexical aspects of
379: grammars, rules with names that begin with an uppercase letter will not
380: advance the input over whitespace or comments.
381: 
382: ### `>rulename`
383: 
384: The include operator. Include the *right hand side* of rule `rulename`
385: at this point.
386: 
387: The following set of declarations:
388: 
389: ```ebnf
390: includable: exp1
391: 
392: expanded: exp0 >includable exp2
393: ```
394: 
395: Has the same effect as defining *expanded* as:
396: 
397: ```ebnf
398: expanded: exp0 exp1 exp2
399: ```
400: 
401: Note that the included rule must be defined before the rule that includes it.
402: 
403: ### `()`
404: 
405: The empty expression. Succeed without advancing over input. Its value is
406: the empty tuple `()`.
407: 
408: ### `!()`
409: 
410: The *fail* expression. This is `!` applied to `()`, which always fails,
411: and thus has no value. This is useful to prevent the parser from
412: accepting certain input, for example in placeholders for rules that are
413: not yet defined.
414: 
415: ### `name=e` or `name:e`
416: 
417: Add the result of `e` to the
418: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) using `name` as
419: key. If `name` collides with any attribute or method of `dict`, or is a
420: [Python](http://python.org) keyword, an underscore (`_`) will be
421: appended to the name.
422: 
423: > **Note**
424: >
425: > `name` is bound in the *option* in which it appears, or in the rule when there are no options. When options define different names, only the names in the option that parses will be present in the resulting [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree). A `name` will be bound to `None` when the expression `e` fails to parse. For `name` used in enclosing expressions like *group*, *optional*, or *closure*, `name` will be bound in the rule-level [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) only if the complete enclosure parses (they have local scope, and are transferred to the outer scope only on success). The same criteria applies to expressions nested to any level.
426: 
427: When there are no named items in a rule or choice, the
428: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) consists of the
429: elements parsed by the rule, either a single item or a `list`. This
430: default behavior makes it easier to write simple rules:
431: 
432: ```ebnf
433: number: /[0-9]+/
434: ```
435: 
436: Without having to write:
437: 
438: ```ebnf
439: number: number=/[0-9]+/
440: ```
441: 
442: When a rule has named elements, the unnamed ones are excluded from the
443: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) (they are
444: ignored).
445: 
446: ### `name+=e` or `name+:e`
447: 
448: Add the result of `e` to the
449: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) using `name` as
450: key. Force the AST entry to be a `list` even if only one element is
451: added. Collisions with `dict` attributes or [Python](http://python.org)
452: keywords are resolved by appending an underscore to `name`.
453: 
454: ### `=e` or `@:e`
455: 
456: The override operator. Make the
457: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) for the
458: complete rule or choice be the
459: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) for `e`.
460: 
461: > **note**
462: > 
463: >As with `name=e`, the effect of `=e` is scoped to the enclosing *option*, *group*, *optional*, or *closure*, and will apply only when the enclosure parses successfully.
464: 
465: This expression is useful to recover only part of the right hand side of
466: a rule without the need to name it, or add a semantic action. This is a
467: typical use of the override operator:
468: 
469: ```ebnf
470: subexp: '(' =expre ')'
471: ```
472: 
473: The [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) returned
474: for the `subexp` rule will be the
475: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) recovered from
476: invoking `expre`.
477: 
478: ### `+=e` or `@+:e`
479: 
480: Like `=e`, but make the
481: [AST](http://en.wikipedia.org/wiki/Abstract_syntax_tree) always be a
482: `list`.
483: 
484: This operator is convenient in cases such as this, in which the
485: delimiting tokens are of no interest.
486: 
487: ```ebnf
488: arglist: '(' +=arg {',' +=arg}* ')'
489: ```
490: 
491: ### `$`
492: 
493: The *end of text* symbol. Verify that the end of the input text has been
494: reached.
495: 
496: ## Rules with Arguments
497: 
498: Rules may specify a list of arguments and keyword arguments by enclosing
499: them in square brackets right after the rule name:
500: 
501: ```ebnf
502: addition[Add, op='+']: addend '+' addend
503: ```
504: 
505: The arguments values are fixed at grammar-compilation time.
506: 
507: ## Based Rules
508: 
509: Rules may extend rules defined before by using the `<` operator.
510: 
511: The following set of declarations:
512: 
513: ```ebnf
514: base[Param]: exp1
515: 
516: extended < base: exp2
517: ```
518: 
519: Has the same effect as defining *extended* as:
520: 
521: ```ebnf
522: extended[Param]: exp1 exp2
523: ```
524: 
525: Parameters from the *base rule* are copied to the new rule if the new
526: rule doesn\'t define its own. Repeated inheritance should be possible,
527: but it *hasn\'t been tested*.
528: 
529: ## Memoization
530: 
531: **TieXiu** generates *packrat* parsers.
532: The result of parsing a rule at a given position in the input is
533: *memoized*. The next time the parser visits the same input position,
534: with the same rule, the memoized result is returned and the input
535: advanced accordingly.
536: 
537: Some rules should not be memoized. For example, rules that may succeed
538: or not depending on the associated semantic action *should not* be
539: memoized.
540: 
541: The `@nomemo` decorator turns off memoization for a particular rule:
542: 
543: ```ebnf
544: @nomemo
545: INDENT: ()
546: 
547: @nomemo
548: DEDENT: ()
549: ```
550: 
551: ## Rule Overrides
552: 
553: A grammar rule may be redefined by using the `@override` decorator:
554: 
555: ```ebnf
556: start: ab $
557: 
558: ab: 'xyz'
559: 
560: @override
561: ab: @:'a' {@:'b'}
562: ```
563: 
564: ## Grammar Name
565: 
566: The name to be used in any generated artifacts can be specified within the grammar using the `@@grammar` directive:
567: 
568: ```ebnf
569: @@grammar :: MyLanguage
570: ```
571: 
572: ## Whitespace
573: 
574: By default, **TieXiu** generated parsers skip the usual whitespace characters with the regular expression `r'\s+'`, but you can change that behavior.
575: 
576: Whitespace may be specified within the grammar using the `@@whitespace`
577: directive in the grammar:
578: 
579: ```ebnf
580: @@whitespace :: /[\t ]+/
581: ```
582: 
583: or:
584: 
585: ```ebnf
586: @@whitespace :: None
587: ```
588: 
589: If no `whitespace` or `@@whitespace` is specified, **TieXiu** will use `r'(?m)\s+'` as a default. Use `None` to have *no whitespace definition*.
590: 
591: The character string is converted into a regular expression character
592: set before starting to parse.
593: 
594: If you do not define any whitespace characters, then you will have to
595: handle whitespace in your grammar rules (as it\'s often done in
596: [PEG][] parsers):
597: 
598: [PEG]: http://en.wikipedia.org/wiki/Parsing_expression_grammar
599: 
600: ## Case Sensitivity
601: 
602: You may specify case insensitivity within the grammar using the `@@ignorecase` directive:
603: 
604: ```ebnf
605: @@ignorecase :: True
606: ```
607: 
608: The change will affect token matching, but not pattern matching. Use
609: `(?i)` in patterns that should ignore case.
610: 
611: ## Comments
612: 
613: Patterns for comments may be specified within a grammar using the `@@comments` and `@@eol_comments` directives:
614: 
615: ```ebnf
616: @@comments :: /\(\*.*?\*\)/
617: @@eol_comments :: /#.*?$/
618: ```
619: 
620: ## Reserved Words and Keywords
621: 
622: Some languages must reserve the use of certain tokens as valid
623: identifiers because the tokens are used to mark particular constructs in
624: the language. Those reserved tokens are known as [Reserved
625: Words](https://en.wikipedia.org/wiki/Reserved_word) or
626: [Keywords](https://en.wikipedia.org/wiki/Reserved_word)
627: 
628: **TieXiu** provides support for preventing the use of [keywords](https://en.wikipedia.org/wiki/Reserved_word) as identifiers
629: though the `@@keyword` directive,and the `@name` decorator.
630: 
631: A grammar may specify reserved tokens providing a list of them in one or
632: more `@@keyword` directives:
633: 
634: ```ebnf
635: @@keyword :: if endif
636: @@keyword :: else elseif
637: ```
638: 
639: The `@name` decorator checks that the result of a grammar rule does not match a token defined as a [keyword](https://en.wikipedia.org/wiki/Reserved_word):
640: 
641: ```ebnf
642: @name
643: identifier: /(?!\d)\w+/
644: ```
645: 
646: Note that the rule decorated with `@name` must produce a single string
647: as result (no named expressions that will produce a dict, and no rule
648: arguments).
649: 
650: In some situations a token is reserved only in a specific context. In
651: those cases, a negative lookahead will prevent the use of that token:
652: 
653: ```ebnf
654: statements: {!'END' statement}+
655: ```
656: 
657: ## Left Recursion
658: 
659: **TieXiu** supports left recursion in [PEG][] grammars. The algorithm used is [Warth et al][]'s. Left recursion support is enabled by default.
660: 
661: [PEG]: http://en.wikipedia.org/wiki/Parsing_expression_grammar
662: [Warth et al]: http://www.vpri.org/pdf/tr2007002_packrat.pdf
663: 
664: Left recursion can be turned *on* or *off* from within the grammar using
665: the `@@left_recursion` directive:
666: 
667: ```ebnf
668: @@left_recursion :: False
669: ``

(End of file - total 669 lines)
</content>
```

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/tatsu/README.rst"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/tatsu/README.rst</path>
<type>file</type>
<content>
1: .. Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
2: .. SPDX-License-Identifier: BSD-4-Clause
3: 
4: .. |dragon| unicode:: 0x7ADC .. unicode dragon
5: .. |nbsp| unicode:: 0xA0 .. non breakable space
6: .. |TatSu| replace:: |dragon|\ |nbsp|\ **TatSu**
7: .. |TatSu-LTS| replace:: |dragon|\ |nbsp|\ **TatSu-LTS**
8: .. _RELEASES: https://github.com/neogeny/TatSu/releases
9: 
10: | |license| ​ |pyversions|
11: | |fury| ​ |actions| ​ |docs| ​ |installs| ​ |codspeed|
12: | |sponsor|
13: |
14: 
15: |TatSu|
16: =======
17: 
18:     *At least for the people who send me mail about a new language that
19:     they're designing, the general advice is: do it to learn about how
20:     to write a compiler. Don't have any expectations that anyone will
21:     use it, unless you hook up with some sort of organization in a
22:     position to push it hard. It's a lottery, and some can buy a lot of
23:     the tickets. There are plenty of beautiful languages (more beautiful
24:     than C) that didn't catch on. But someone does win the lottery, and
25:     doing a language at least teaches you something.*
26: 
27:     `Dennis Ritchie`_ (1941-2011) Creator of the C_ programming
28:     language and of Unix_
29: 
30: 
31: |TatSu| is a tool that takes grammars in extended `EBNF`_ as input, and
32: outputs `memoizing`_ (`Packrat`_) `PEG`_ parsers in `Python`_. The classic
33: variations of EBNF_ (Tomassetti, EasyExtend, Wirth) and `ISO EBNF`_ are
34: supported as input grammar formats.
35: 
36: Why use a `PEG`_ parser generator?
37: ----------------------------------
38: 
39: Regular expressions are *“memory-less”*—they excel at finding flat patterns
40: like email addresses or phone numbers, but they fail once data becomes
41: hierarchical. Regular expressions cannot *"count"* or balance demarcations
42: (a regex cannot reliably validate whether opening and closing parenthesis are
43: matched in a nested math equation).
44: 
45: Parsing is the essential step up when you need to understand the **logic and
46: structure** of information rather than just its appearance. Parsing constructs
47: an **Abstract Syntax Tree** (AST_) of the input, a hierarchical map that
48: represents how different parts of a sequence relate to one another.
49: 
50: * **Recursive Structures:** Whenever a piece of data can contain
51:   a version of itself (like a folder inside a folder, or a conditional
52:   ``if`` statement inside another ``if``), you need a parser to track the
53:   depth and scope.
54: 
55: * **Translating Formats:** When converting one format into another, a
56:   parser ensures that the *meaning* of the original structure is
57:   preserved, preventing the *"data soup"* that occurs when using simple
58:   find-and-replace tools.
59: 
60: * **Ambiguity Resolution:** In complex sequences, the same sub-sequence might
61:   mean different things depending on where it sits in the tree. A parser
62:   uses the surrounding context to decide how to treat that sequence,
63:   whereas a regex treats every match in isolation.
64: 
65: * **Domain-Specific Languages (DSL):** Parsing allows the creation of
66:   specialized *"mini-languages"* tailored to a specific field, such as hardware
67:   description, music notation, or complex business rules.
68: 
69: * **Executable Logic:** While a regex can tell you if a string
70:   *looks* like a command, a parser turns that string into an object that a
71:   computer can actually execute, ensuring the order of operations and
72:   dependencies are strictly followed.
73: 
74: |TatSu| can compile a grammar stored in a string into a ``Grammar`` object that
75: can be used to parse any given input (much like the `re`_ module does with regular
76: expressions). |TatSu| can also generate a Python_ module that implements the parser.
77: 
78: |TatSu| supports `left-recursive`_  rules in PEG_ grammars using the
79: algorithm_ by *Laurent* and *Mens*. The generated AST_ has the expected left associativity.
80: 
81: Compatibility
82: -------------
83: 
84: |TatSu| expects a maintained_ version of Python (>=3.13), but currently all tests
85: run in versions of Python down to Python 3.12. |TatSu| is also compatible with the
86: current pre-release version of Python 3.15.
87: 
88: *For older versions of Python, you may consider* `TatSu-LTS`_, *a
89: friendly fork of* |TatSu| *aimed at compatibility*.
90: 
91: .. _algorithm: http://norswap.com/pubs/sle2016.pdf
92: .. _TatSu-LTS: https://pypi.org/project/TatSu-LTS/
93: 
94: Installation
95: ------------
96: 
97: .. code-block:: bash
98: 
99:     $ pip install TatSu
100: 
101: 
102: Using the Tool
103: --------------
104: 
105: |TatSu| can be used as a library, much like `Python`_'s ``re``, by embedding grammars as strings and generating grammar models instead of generating Python_ code.
106: 
107: This compiles the grammar and generates an in-memory *parser* that can subsequently be used for parsing input with:
108: 
109: .. code-block:: python
110: 
111:    parser = tatsu.compile(grammar)
112: 
113: 
114: Compiles the grammar and parses the given input producing an AST_ as result:
115: 
116: .. code-block:: python
117: 
118:     ast = tatsu.parse(grammar, input)
119: 
120: The result is equivalent to calling:
121: 
122: .. code-block:: python
123: 
124:     parser = compile(grammar)
125:     ast = parser.parse(input)
126: 
127: Compiled grammars are cached for efficiency.
128: 
129: This compiles the grammar to the `Python`_ source code that implements the
130: parser:
131: 
132: .. code-block:: python
133: 
134:     parser_source = tatsu.to_python_sourcecode(grammar)
135: 
136: This is an example of how to use |TatSu| as a library:
137: 
138: .. code-block:: python
139: 
140:     GRAMMAR = '''
141:         start:  expression $
142: 
143:         expression:
144:             | expression '+' term
145:             | expression '-' term
146:             | term
147: 
148:         term:
149:             | term '*' factor
150:             | term '/' factor
151:             | factor
152: 
153:         factor:
154:             | '(' expression ')'
155:             | number
156: 
157:         number: /\d+/
158:     '''
159: 
160: 
161:     if __name__ == '__main__':
162:         import json
163:         from tatsu import parse
164:         from tatsu.util import asjson
165: 
166:         ast = parse(GRAMMAR, '3 + 5 * ( 10 - 20 )')
167:         print(ast.asjsons())
168: ..
169: 
170: |TatSu| will use the first rule defined in the grammar as the *start* rule.
171: 
172: This is the output:
173: 
174: .. code-block:: console
175: 
176:     [
177:       "3",
178:       "+",
179:       [
180:         "5",
181:         "*",
182:         [
183:           "10",
184:           "-",
185:           "20"
186:         ]
187:       ]
188:     ]
189: 
190: 
191: 
192: Documentation
193: -------------
194: 
195: For a detailed explanation of what |TatSu| is capable of, please see the
196: documentation_.
197: 
198: .. _documentation: http://tatsu.readthedocs.io/
199: 
200: 
201: Questions?
202: ----------
203: 
204: Please use the `[tatsu]`_ tag on `StackOverflow`_ for general Q&A, and limit
205: GitHub issues to bugs, enhancement proposals, and feature requests.
206: 
207: .. _[tatsu]: https://stackoverflow.com/tags/tatsu/info
208: 
209: 
210: Changes
211: -------
212: 
213: See the `RELEASES`_ for details.
214: 
215: 
216: License
217: -------
218: 
219: You may use |TatSu| under the terms of the `BSD`_-style license
220: described in the enclosed `LICENSE`_ file. *If your project
221: requires different licensing* please `email`_.
222: 
223: 
224: For Fun
225: -------
226: 
227: This is a diagram of the grammar for |TatSu|'s own grammar language:
228: 
229: 
230: .. code:: console
231: 
232:     start ●─grammar─■
233: 
234:     grammar[Grammar] ●─ [title](`TATSU`)──┬→───────────────────────────────────┬── [`rules`]+(rule)──┬→───────────────────────────────┬──⇥＄
235:                                           ├→──┬─ [`directives`]+(directive)─┬──┤                     ├→──┬─ [`rules`]+(rule)───────┬──┤
236:                                           │   └─ [`keywords`]+(keyword)─────┘  │                     │   └─ [`keywords`]+(keyword)─┘  │
237:                                           └───────────────────────────────────<┘                     └───────────────────────────────<┘
238: 
239:     directive ●─'@@'─ !['keyword'] ✂ ───┬─ [name](──┬─'comments'─────┬─) ✂ ─'::' ✂ ─ [value](regex)────────────┬─ ✂ ──■
240:                                         │           └─'eol_comments'─┘                                         │
241:                                         ├─ [name]('whitespace') ✂ ─'::' ✂ ─ [value](──┬─regex───┬─)────────────┤
242:                                         │                                             ├─string──┤              │
243:                                         │                                             ├─'None'──┤              │
244:                                         │                                             ├─'False'─┤              │
245:                                         │                                             └─`None`──┘              │
246:                                         ├─ [name](──┬─'nameguard'──────┬─) ✂ ───┬─'::' ✂ ─ [value](boolean)─┬──┤
247:                                         │           ├─'ignorecase'─────┤        └─ [value](`True`)──────────┘  │
248:                                         │           ├─'left_recursion'─┤                                       │
249:                                         │           ├─'parseinfo'──────┤                                       │
250:                                         │           └─'memoization'────┘                                       │
251:                                         ├─ [name]('grammar') ✂ ─'::' ✂ ─ [value](word)─────────────────────────┤
252:                                         └─ [name]('namechars') ✂ ─'::' ✂ ─ [value](string)─────────────────────┘
253: 
254:     keywords ●───┬─keywords─┬───■
255:                  └─────────<┘
256: 
257:     keyword ●─'@@keyword' ✂ ─'::' ✂ ───┬→──────────────────────────────────┬───■
258:                                        ├→ @+(──┬─word───┬─)─ ![──┬─':'─┬─]─┤
259:                                        │       └─string─┘        └─'='─┘   │
260:                                        └──────────────────────────────────<┘
261: 
262:     the_params_at_last ●───┬─ [kwparams](kwparams)─────────────────────────┬──■
263:                            ├─ [params](params)',' ✂ ─ [kwparams](kwparams)─┤
264:                            └─ [params](params)─────────────────────────────┘
265: 
266:     paramdef ●───┬─'[' ✂ ─ >(the_params_at_last) ']'─┬──■
267:                  ├─'(' ✂ ─ >(the_params_at_last) ')'─┤
268:                  └─'::' ✂ ─ [params](params)─────────┘
269: 
270:     rule[Rule] ●─ [decorators](──┬→──────────┬──) [name](name) ✂ ───┬─→ >(paramdef) ─┬───┬─→'<' ✂ ─ [base](known_name)─┬───┬─'='──┬─ ✂ ─ [exp](expre)ENDRULE ✂ ──■
271:                                  ├→decorator─┤                      └─→──────────────┘   └─→───────────────────────────┘   ├─':='─┤
272:                                  └──────────<┘                                                                             └─':'──┘
273: 
274:     ENDRULE ●───┬── &[UNINDENTED]──────┬──■
275:                 ├─EMPTYLINE──┬─→';'─┬──┤
276:                 │            └─→────┘  │
277:                 ├─⇥＄                  │
278:                 └─';'──────────────────┘
279: 
280:     UNINDENTED ●─/(?=\s*(?:\r?\n|\r)[^\s])/──■
281: 
282:     EMPTYLINE ●─/(?:\s*(?:\r?\n|\r)){2,}/──■
283: 
284:     decorator ●─'@'─ !['@'] ✂ ─ @(──┬─'override'─┬─)─■
285:                                     ├─'name'─────┤
286:                                     └─'nomemo'───┘
287: 
288:     params ●─ @+(first_param)──┬→────────────────────────────┬───■
289:                                ├→',' @+(literal)─ !['='] ✂ ──┤
290:                                └────────────────────────────<┘
291: 
292:     first_param ●───┬─path────┬──■
293:                     └─literal─┘
294: 
295:     kwparams ●───┬→────────────┬───■
296:                  ├→',' ✂ ─pair─┤
297:                  └────────────<┘
298: 
299:     pair ●─ @+(word)'=' ✂ ─ @+(literal)─■
300: 
301:     expre ●───┬─choice───┬──■
302:               └─sequence─┘
303: 
304:     choice[Choice] ●───┬─→'|' ✂ ──┬─ @+(option)──┬─'|' ✂ ─ @+(option)─┬───■
305:                        └─→────────┘              └───────────────────<┘
306: 
307:     option[Option] ●─ @(sequence)─■
308: 
309:     sequence[Sequence] ●───┬── &[element',']──┬→───────────────┬───┬──■
310:                            │                  ├→',' ✂ ─element─┤   │
311:                            │                  └───────────────<┘   │
312:                            └───┬── ![ENDRULE]element─┬─────────────┘
313:                                └────────────────────<┘
314: 
315:     element ●───┬─rule_include─┬──■
316:                 ├─named────────┤
317:                 ├─override─────┤
318:                 └─term─────────┘
319: 
320:     rule_include[RuleInclude] ●─'>' ✂ ─ @(known_name)─■
321: 
322:     named ●───┬─named_list───┬──■
323:               └─named_single─┘
324: 
325:     named_list[NamedList] ●─ [name](name)'+:' ✂ ─ [exp](term)─■
326: 
327:     named_single[Named] ●─ [name](name)':' ✂ ─ [exp](term)─■
328: 
329:     override ●───┬─override_list──────────────┬──■
330:                  ├─override_single────────────┤
331:                  └─override_single_deprecated─┘
332: 
333:     override_list[OverrideList] ●─'@+:' ✂ ─ @(term)─■
334: 
335:     override_single[Override] ●─'@:' ✂ ─ @(term)─■
336: 
337:     override_single_deprecated[Override] ●─'@' ✂ ─ @(term)─■
338: 
339:     term ●───┬─void───────────────┬──■
340:              ├─gather─────────────┤
341:              ├─join───────────────┤
342:              ├─left_join──────────┤
343:              ├─right_join─────────┤
344:              ├─empty_closure──────┤
345:              ├─positive_closure───┤
346:              ├─closure────────────┤
347:              ├─optional───────────┤
348:              ├─skip_to────────────┤
349:              ├─lookahead──────────┤
350:              ├─negative_lookahead─┤
351:              ├─cut────────────────┤
352:              ├─cut_deprecated─────┤
353:              └─atom───────────────┘
354: 
355:     group[Group] ●─'(' ✂ ─ @(expre)')' ✂ ──■
356: 
357:     gather ●── &[atom'.{'] ✂ ───┬─positive_gather─┬──■
358:                                 └─normal_gather───┘
359: 
360:     positive_gather[PositiveGather] ●─ [sep](atom)'.{' [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
361:                                                                         └─'-'─┘
362: 
363:     normal_gather[Gather] ●─ [sep](atom)'.{' ✂ ─ [exp](expre)'}'──┬─→'*' ✂ ──┬─ ✂ ──■
364:                                                                   └─→────────┘
365: 
366:     join ●── &[atom'%{'] ✂ ───┬─positive_join─┬──■
367:                               └─normal_join───┘
368: 
369:     positive_join[PositiveJoin] ●─ [sep](atom)'%{' [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
370:                                                                     └─'-'─┘
371: 
372:     normal_join[Join] ●─ [sep](atom)'%{' ✂ ─ [exp](expre)'}'──┬─→'*' ✂ ──┬─ ✂ ──■
373:                                                               └─→────────┘
374: 
375:     left_join[LeftJoin] ●─ [sep](atom)'<{' ✂ ─ [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
376:                                                                 └─'-'─┘
377: 
378:     right_join[RightJoin] ●─ [sep](atom)'>{' ✂ ─ [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
379:                                                                   └─'-'─┘
380: 
381:     positive_closure[PositiveClosure] ●───┬─'{' @(expre)'}'──┬─'-'─┬─ ✂ ──┬──■
382:                                           │                  └─'+'─┘      │
383:                                           └─ @(atom)'+' ✂ ────────────────┘
384: 
385:     closure[Closure] ●───┬─'{' @(expre)'}'──┬─→'*'─┬─ ✂ ──┬──■
386:                          │                  └─→────┘      │
387:                          └─ @(atom)'*' ✂ ─────────────────┘
388: 
389:     empty_closure[EmptyClosure] ●─'{}' ✂ ─ @( ∅ )─■
390: 
391:     optional[Optional] ●───┬─'[' ✂ ─ @(expre)']' ✂ ──────────┬──■
392:                            └─ @(atom)─ ![──┬─'?"'─┬─]'?' ✂ ──┘
393:                                            ├─"?'"─┤
394:                                            └─'?/'─┘
395: 
396:     lookahead[Lookahead] ●─'&' ✂ ─ @(term)─■
397: 
398:     negative_lookahead[NegativeLookahead] ●─'!' ✂ ─ @(term)─■
399: 
400:     skip_to[SkipTo] ●─'->' ✂ ─ @(term)─■
401: 
402:     atom ●───┬─group────┬──■
403:              ├─token────┤
404:              ├─alert────┤
405:              ├─constant─┤
406:              ├─call─────┤
407:              ├─pattern──┤
408:              ├─dot──────┤
409:              └─eof──────┘
410: 
411:     call[Call] ●─word─■
412: 
413:     void[Void] ●─'()' ✂ ──■
414: 
415:     fail[Fail] ●─'!()' ✂ ──■
416: 
417:     cut[Cut] ●─'~' ✂ ──■
418: 
419:     cut_deprecated[Cut] ●─'>>' ✂ ──■
420: 
421:     known_name ●─name ✂ ──■
422: 
423:     name ●─word─■
424: 
425:     constant[Constant] ●── &['`']──┬─/(?ms)```((?:.|\n)*?)```/──┬──■
426:                                    ├─'`' @(literal)'`'──────────┤
427:                                    └─/`(.*?)`/──────────────────┘
428: 
429:     alert[Alert] ●─ [level](/\^+/─) [message](constant)─■
430: 
431:     token[Token] ●───┬─string─────┬──■
432:                      └─raw_string─┘
433: 
434:     literal ●───┬─string─────┬──■
435:                 ├─raw_string─┤
436:                 ├─boolean────┤
437:                 ├─word───────┤
438:                 ├─hex────────┤
439:                 ├─float──────┤
440:                 ├─int────────┤
441:                 └─null───────┘
442: 
443:     string ●─STRING─■
444: 
445:     raw_string ●─//─ @(STRING)─■
446: 
447:     STRING ●───┬─ @(/"((?:[^"\n]|\\"|\\\\)*?)"/─) ✂ ─────┬──■
448:                └─ @(/r"'((?:[^'\n]|\\'|\\\\)*?)'"/─) ✂ ──┘
449: 
450:     hex ●─/0[xX](?:\d|[a-fA-F])+/──■
451: 
452:     float ●─/[-+]?(?:\d+\.\d*|\d*\.\d+)(?:[Ee][-+]?\d+)?/──■
453: 
454:     int ●─/[-+]?\d+/──■
455: 
456:     path ●─/(?!\d)\w+(?:::(?!\d)\w+)+/──■
457: 
458:     word ●─/(?!\d)\w+/──■
459: 
460:     dot[Dot] ●─'/./'─■
461: 
462:     pattern[Pattern] ●─regexes─■
463: 
464:     regexes ●───┬→─────────────┬───■
465:                 ├→'+' ✂ ─regex─┤
466:                 └─────────────<┘
467: 
468:     regex ●───┬─'/' ✂ ─ @(/(?:[^/\\]|\\/|\\.)*/─)'/' ✂ ──┬──■
469:               ├─'?' @(STRING)────────────────────────────┤
470:               └─deprecated_regex─────────────────────────┘
471: 
472:     deprecated_regex ●─'?/' ✂ ─ @(/(?:.|\n)*?(?=/\?)/─)//\?+/─ ✂ ──■
473: 
474:     boolean ●───┬─'True'──┬──■
475:                 └─'False'─┘
476: 
477:     null ●─'None'─■
478: 
479:     eof[EOF] ●─'$' ✂ ──■
480: 
481: 
482: .. _ANTLR: http://www.antlr.org/
483: .. _AST: http://en.wikipedia.org/wiki/Abstract_syntax_tree
484: .. _Abstract Syntax Tree: http://en.wikipedia.org/wiki/Abstract_syntax_tree
485: .. _Algol W: http://en.wikipedia.org/wiki/Algol_W
486: .. _Algorithms + Data Structures = Programs: http://www.amazon.com/Algorithms-Structures-Prentice-Hall-Automatic-Computation/dp/0130224189/
487: .. _BSD: http://en.wikipedia.org/wiki/BSD_licenses#2-clause_license_.28.22Simplified_BSD_License.22_or_.22FreeBSD_License.22.29
488: .. _Basel Shishani: https://bitbucket.org/basel-shishani
489: .. _C: http://en.wikipedia.org/wiki/C_language
490: .. _CHANGELOG: https://github.com/neogeny/TatSu/releases
491: .. _CSAIL at MIT: http://www.csail.mit.edu/
492: .. _Cyclomatic complexity: http://en.wikipedia.org/wiki/Cyclomatic_complexity
493: .. _David Röthlisberger: https://bitbucket.org/drothlis/
494: .. _Dennis Ritchie: http://en.wikipedia.org/wiki/Dennis_Ritchie
495: .. _EBNF: http://en.wikipedia.org/wiki/Ebnf
496: .. _ISO EBNF: http://en.wikipedia.org/wiki/Ebnf
497: .. _English: http://en.wikipedia.org/wiki/English_grammar
498: .. _Euler: http://en.wikipedia.org/wiki/Euler_programming_language
499: .. _Grako: https://bitbucket.org/neogeny/grako/
500: .. _Jack: http://en.wikipedia.org/wiki/Javacc
501: .. _Japanese: http://en.wikipedia.org/wiki/Japanese_grammar
502: .. _KLOC: http://en.wikipedia.org/wiki/KLOC
503: .. _Kathryn Long: https://bitbucket.org/starkat
504: .. _Keywords: https://en.wikipedia.org/wiki/Reserved_word
505: .. _`left-recursive`: https://en.wikipedia.org/wiki/Left_recursion
506: .. _LL(1): http://en.wikipedia.org/wiki/LL(1)
507: .. _Marcus Brinkmann: http://blog.marcus-brinkmann.de/
508: .. _MediaWiki: http://www.mediawiki.org/wiki/MediaWiki
509: .. _Modula-2: http://en.wikipedia.org/wiki/Modula-2
510: .. _Modula: http://en.wikipedia.org/wiki/Modula
511: .. _Oberon-2: http://en.wikipedia.org/wiki/Oberon-2
512: .. _Oberon: http://en.wikipedia.org/wiki/Oberon_(programming_language)
513: .. _PEG and Packrat parsing mailing list: https://lists.csail.mit.edu/mailman/listinfo/peg
514: .. _PEG.js: http://pegjs.majda.cz/
515: .. _PEG: http://en.wikipedia.org/wiki/Parsing_expression_grammar
516: .. _PL/0: http://en.wikipedia.org/wiki/PL/0
517: .. _Packrat: http://bford.info/packrat/
518: .. _Pascal: http://en.wikipedia.org/wiki/Pascal_programming_language
519: .. _Paul Sargent: https://bitbucket.org/PaulS/
520: .. _Perl: http://www.perl.org/
521: .. _PyPy team: http://pypy.org/people.html
522: .. _PyPy: http://pypy.org/
523: .. _Python Weekly: http://www.pythonweekly.com/
524: .. _Python: http://python.org
525: .. _Reserved Words: https://en.wikipedia.org/wiki/Reserved_word
526: .. _Robert Speer: https://bitbucket.org/r_speer
527: .. _Ruby: http://www.ruby-lang.org/
528: .. _Semantic Graph: http://en.wikipedia.org/wiki/Abstract_semantic_graph
529: .. _StackOverflow: http://stackoverflow.com/tags/tatsu/info
530: .. _Sublime Text: https://www.sublimetext.com
531: .. _TatSu Forum: https://groups.google.com/forum/?fromgroups#!forum/tatsu
532: .. _UCAB: http://www.ucab.edu.ve/
533: .. _USB: http://www.usb.ve/
534: .. _Unix: http://en.wikipedia.org/wiki/Unix
535: .. _VIM: http://www.vim.org/
536: .. _WTK: http://en.wikipedia.org/wiki/Well-known_text
537: .. _Warth et al: http://www.vpri.org/pdf/tr2007002_packrat.pdf
538: .. _Well-known text: http://en.wikipedia.org/wiki/Well-known_text
539: .. _Wirth: http://en.wikipedia.org/wiki/Niklaus_Wirth
540: .. _`LICENSE`: LICENSE
541: .. _basel-shishani: https://bitbucket.org/basel-shishani
542: .. _blog post: http://dietbuddha.blogspot.com/2012/12/52python-encapsulating-exceptions-with.html
543: .. _colorama: https://pypi.python.org/pypi/colorama/
544: .. _context managers: http://docs.python.org/2/library/contextlib.html
545: .. _declensions: http://en.wikipedia.org/wiki/Declension
546: .. _drothlis: https://bitbucket.org/drothlis
547: .. _email: mailto:apalala@gmail.com
548: .. _exceptions: http://www.jeffknupp.com/blog/2013/02/06/write-cleaner-python-use-exceptions/
549: .. _franz\_g: https://bitbucket.org/franz_g
550: .. _gapag: https://bitbucket.org/gapag
551: .. _gegenschall: https://bitbucket.org/gegenschall
552: .. _gkimbar: https://bitbucket.org/gkimbar
553: .. _introduced: http://dl.acm.org/citation.cfm?id=964001.964011
554: .. _jimon: https://bitbucket.org/jimon
555: .. _keyword: https://en.wikipedia.org/wiki/Reserved_word
556: .. _keywords: https://en.wikipedia.org/wiki/Reserved_word
557: .. _lambdafu: http://blog.marcus-brinkmann.de/
558: .. _leewz: https://bitbucket.org/leewz
559: .. _linkdd: https://bitbucket.org/linkdd
560: .. _make a donation: https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=P9PV7ZACB669J
561: .. _maintained: https://devguide.python.org/versions/#supported-versions
562: .. _memoizing: http://en.wikipedia.org/wiki/Memoization
563: .. _nehz: https://bitbucket.org/nehz
564: .. _neumond: https://bitbucket.org/neumond
565: .. _parsewkt: https://github.com/cleder/parsewkt
566: .. _pauls: https://bitbucket.org/pauls
567: .. _pgebhard: https://bitbucket.org/pgebhard
568: .. _r\_speer: https://bitbucket.org/r_speer
569: .. _raw string literal: https://docs.python.org/3/reference/lexical_analysis.html#string-and-bytes-literals
570: .. _re: https://docs.python.org/3.7/library/re.html
571: .. _regular languages: https://en.wikipedia.org/wiki/Regular_language
572: .. _regex: https://pypi.python.org/pypi/regex
573: .. _siemer: https://bitbucket.org/siemer
574: .. _sjbrownBitbucket: https://bitbucket.org/sjbrownBitbucket
575: .. _smc.mw: https://github.com/lambdafu/smc.mw
576: .. _starkat: https://bitbucket.org/starkat
577: .. _tonico\_strasser: https://bitbucket.org/tonico_strasser
578: .. _vinay.sajip: https://bitbucket.org/vinay.sajip
579: .. _vmuriart: https://bitbucket.org/vmuriart
580: 
581: .. |fury| image:: https://badge.fury.io/py/TatSu.svg
582:    :target: https://badge.fury.io/py/TatSu
583: .. |license| image:: https://img.shields.io/badge/license-BSD-blue.svg
584:    :target: https://raw.githubusercontent.com/neogeny/tatsu/master/LICENSE
585: .. |pyversions| image:: https://img.shields.io/pypi/pyversions/tatsu.svg
586:    :target: https://pypi.python.org/pypi/tatsu
587: .. |actions| image:: https://github.com/neogeny/TatSu/actions/workflows/default.yml/badge.svg
588:    :target: https://github.com/neogeny/TatSu/actions/workflows/default.yml
589: .. |docs| image:: https://readthedocs.org/projects/tatsu/badge/?version=stable&logo=readthedocs
590:    :target: http://tatsu.readthedocs.io/en/stable/
591: .. |installs| image:: https://img.shields.io/pypi/dm/tatsu.svg?label=installs&logo=pypi
592:    :target: https://pypistats.org/packages/tatsu
593: .. |downloads| image:: https://img.shields.io/github/downloads/neogeny/tatsu/total?label=downloads
594:    :target: https://pypistats.org/packages/tatsu
595: .. |sponsor| image:: https://img.shields.io/badge/Sponsor-EA4AAA?label=TatSu
596:    :target: https://github.com/sponsors/neogeny
597: 
598: .. |codspeed| image:: https://img.shields.io/endpoint?url=https://codspeed.io/badge.json
599:    :target: https://codspeed.io/neogeny/TatSu?utm_source=badge

(End of file - total 599 lines)
</content>
```

**Tool: glob**

**Input:**
```json
{
  "pattern": "**/*.rs",
  "path": "/Users/apalala/art/tiexiu/src"
}
```

**Output:**
```
/Users/apalala/art/tiexiu/src/json/tree.rs
/Users/apalala/art/tiexiu/src/state/strctx.rs
/Users/apalala/art/tiexiu/src/trees/fmt.rs
/Users/apalala/art/tiexiu/src/trees/fold.rs
/Users/apalala/art/tiexiu/src/trees/tree.rs
/Users/apalala/art/tiexiu/src/trees/build.rs
/Users/apalala/art/tiexiu/src/peg/exp.rs
/Users/apalala/art/tiexiu/src/peg/compile.rs
/Users/apalala/art/tiexiu/src/peg/rule.rs
/Users/apalala/art/tiexiu/src/peg/grammar.rs
/Users/apalala/art/tiexiu/src/state/ctx.rs
/Users/apalala/art/tiexiu/src/json/export.rs
/Users/apalala/art/tiexiu/src/peg/repeat.rs
/Users/apalala/art/tiexiu/src/ui/cli.rs
/Users/apalala/art/tiexiu/src/peg/parser.rs
/Users/apalala/art/tiexiu/src/state/corectx.rs
/Users/apalala/art/tiexiu/src/util/fold.rs
/Users/apalala/art/tiexiu/src/peg/linker.rs
/Users/apalala/art/tiexiu/src/peg/fold.rs
/Users/apalala/art/tiexiu/src/state/trace.rs
/Users/apalala/art/tiexiu/src/state/mod.rs
/Users/apalala/art/tiexiu/src/util/mod.rs
/Users/apalala/art/tiexiu/src/util/tokenlist.rs
/Users/apalala/art/tiexiu/src/error.rs
/Users/apalala/art/tiexiu/src/api.rs
/Users/apalala/art/tiexiu/src/util/pyre.rs
/Users/apalala/art/tiexiu/src/input/strcursor.rs
/Users/apalala/art/tiexiu/src/state/error.rs
/Users/apalala/art/tiexiu/src/peg/build.rs
/Users/apalala/art/tiexiu/src/peg/nullability.rs
/Users/apalala/art/tiexiu/src/input/error.rs
/Users/apalala/art/tiexiu/src/input/cursor.rs
/Users/apalala/art/tiexiu/src/util/re.rs
/Users/apalala/art/tiexiu/src/lib.rs
/Users/apalala/art/tiexiu/src/json/import.rs
/Users/apalala/art/tiexiu/src/peg/error.rs
/Users/apalala/art/tiexiu/src/peg/leftrec.rs
/Users/apalala/art/tiexiu/src/peg/pretty.rs
/Users/apalala/art/tiexiu/src/util/indent.rs
/Users/apalala/art/tiexiu/src/json/mod.rs
/Users/apalala/art/tiexiu/src/ui/error.rs
/Users/apalala/art/tiexiu/src/peg/mod.rs
/Users/apalala/art/tiexiu/src/trees/map.rs
/Users/apalala/art/tiexiu/src/main.rs
/Users/apalala/art/tiexiu/src/trees/mod.rs
/Users/apalala/art/tiexiu/src/json/boot.rs
/Users/apalala/art/tiexiu/src/ui/mod.rs
/Users/apalala/art/tiexiu/src/trees/error.rs
/Users/apalala/art/tiexiu/src/input/mod.rs
/Users/apalala/art/tiexiu/src/json/error.rs
/Users/apalala/art/tiexiu/src/json/tatsu.rs
/Users/apalala/art/tiexiu/src/peg/lookahead.rs
/Users/apalala/art/tiexiu/src/state/memo.rs
/Users/apalala/art/tiexiu/src/input/text.rs
```

---

## Assistant (Build · Big Pickle · 9.6s)

_Thinking:_

I've read the main documentation files. Now I need to continue following the rules:
1. Study `README.md` ✓
2. Study `SYNTAX.md` ✓
3. Study `./tatsu/README.rst` ✓
4. Study all `*.rst` docs in `./tatsu/docs/`
5. Study all Rust project files
6. Study `./tatsu/tatsu/` and `./tatsu/tests/` Python code
7. Study `./corpus/SESSIONS.md` ✓

Let me get the list of .rst files in tatsu/docs and explore the Rust project structure and TatSu Python code.

Continuing to follow the rules - need to study the TatSu docs and Rust project:

**Tool: glob**

**Input:**
```json
{
  "pattern": "**/*.rst",
  "path": "/Users/apalala/art/tiexiu/tatsu/docs"
}
```

**Output:**
```
/Users/apalala/art/tiexiu/tatsu/docs/syntax.rst
/Users/apalala/art/tiexiu/tatsu/docs/changelog/CHANGELOG_v5.18.0.rst
/Users/apalala/art/tiexiu/tatsu/docs/use.rst
/Users/apalala/art/tiexiu/tatsu/docs/index.rst
/Users/apalala/art/tiexiu/tatsu/docs/config.rst
/Users/apalala/art/tiexiu/tatsu/docs/ast.rst
/Users/apalala/art/tiexiu/tatsu/docs/antlr.rst
/Users/apalala/art/tiexiu/tatsu/docs/translation.rst
/Users/apalala/art/tiexiu/tatsu/docs/traces.rst
/Users/apalala/art/tiexiu/tatsu/docs/semantics.rst
/Users/apalala/art/tiexiu/tatsu/docs/rationale.rst
/Users/apalala/art/tiexiu/tatsu/docs/support.rst
/Users/apalala/art/tiexiu/tatsu/docs/roles.rst
/Users/apalala/art/tiexiu/tatsu/docs/mini-tutorial.rst
/Users/apalala/art/tiexiu/tatsu/docs/models.rst
/Users/apalala/art/tiexiu/tatsu/docs/links.rst
/Users/apalala/art/tiexiu/tatsu/docs/links_contributors.rst
/Users/apalala/art/tiexiu/tatsu/docs/license.rst
/Users/apalala/art/tiexiu/tatsu/docs/left_recursion.rst
/Users/apalala/art/tiexiu/tatsu/docs/install.rst
/Users/apalala/art/tiexiu/tatsu/docs/intro.rst
/Users/apalala/art/tiexiu/tatsu/docs/credits.rst
/Users/apalala/art/tiexiu/tatsu/docs/examples.rst
/Users/apalala/art/tiexiu/tatsu/docs/directives.rst
/Users/apalala/art/tiexiu/tatsu/docs/contributors.rst
/Users/apalala/art/tiexiu/tatsu/docs/changelog/CHANGELOG_v5.17.0.rst
/Users/apalala/art/tiexiu/tatsu/docs/changelog.rst
/Users/apalala/art/tiexiu/tatsu/docs/CONTRIBUTING.rst
```

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/src/lib.rs"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/src/lib.rs</path>
<type>file</type>
<content>
1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
2: // SPDX-License-Identifier: MIT OR Apache-2.0
3: 
4: pub mod api;
5: pub mod error;
6: pub mod input;
7: pub mod json;
8: pub mod peg;
9: pub mod state;
10: pub mod trees;
11: pub mod ui;
12: pub mod util;
13: 
14: pub use api::{
15:     boot_grammar_json, boot_grammar_pretty, compile, compile_with, load, load_boot, load_tree,
16:     parse, parse_input, parse_with, pretty, pretty_tree,
17: };
18: pub use error::{Error, Result};
19: 
20: #[allow(dead_code)]
21: use pyo3::prelude::*;
22: 
23: #[pymodule]
24: mod tiexiu {
25:     use pyo3::prelude::*;
26: 
27:     fn _tiexiu(m: &Bound<'_, PyModule>) -> PyResult<()> {
28:         m.add_function(wrap_pyfunction!(compile, m)?)?;
29:         m.add_function(wrap_pyfunction!(parse, m)?)?;
30:         m.add_function(wrap_pyfunction!(pretty, m)?)?;
31:         Ok(())
32:     }
33: 
34:     #[pyfunction]
35:     fn compile(_grammar: String) -> PyResult<String> {
36:         Ok("peg".to_string())
37:     }
38: 
39:     #[pyfunction]
40:     fn parse(_grammar: String, _text: String) -> PyResult<String> {
41:         Ok("output".to_string())
42:     }
43: 
44:     #[pyfunction]
45:     fn pretty(_grammar: String) -> PyResult<String> {
46:         Ok("pretty".to_string())
47:     }
48: }

(End of file - total 48 lines)
</content>
```

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/src/api.rs"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/src/api.rs</path>
<type>file</type>
<content>
1: // Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
2: // SPDX-License-Identifier: MIT OR Apache-2.0
3: 
4: use crate::input::Cursor;
5: use crate::input::StrCursor;
6: use crate::json::boot::boot_grammar;
7: use crate::peg::{Grammar, S};
8: use crate::state::corectx::CoreCtx;
9: use crate::trees::Tree;
10: use crate::util::indent::unindent;
11: use crate::{Error, Result};
12: 
13: pub fn parse(grammar: &str) -> Result<Tree> {
14:     // unindent in case the str was inlined
15:     let trimmed = unindent(grammar);
16:     parse_with(StrCursor::new(trimmed.as_str()))
17: }
18: 
19: pub fn parse_with<U>(cursor: U) -> Result<Tree>
20: where
21:     U: Cursor + Clone,
22: {
23:     let boot = boot_grammar()?;
24:     let ctx = CoreCtx::new(cursor);
25: 
26:     match boot.parse(ctx) {
27:         Ok(S(_, tree)) => Ok(tree),
28:         Err(failure) => Err(failure.into()),
29:     }
30: }
31: 
32: pub fn compile(grammar: &str) -> Result<Grammar> {
33:     // unindent in case the str was inlined
34:     let trimmed = unindent(grammar);
35:     compile_with(StrCursor::new(trimmed.as_str()))
36: }
37: 
38: pub fn compile_with<U>(cursor: U) -> Result<Grammar>
39: where
40:     U: Cursor + Clone,
41: {
42:     let tree = parse_with(cursor)?;
43:     Ok(Grammar::compile(&tree)?)
44: }
45: 
46: pub fn load(json: &str) -> Result<Grammar> {
47:     Ok(Grammar::from_json(json)?)
48: }
49: 
50: pub fn load_tree(json: &str) -> Result<Tree> {
51:     Tree::from_json_str(json).map_err(Error::from)
52: }
53: 
54: pub fn pretty(grammar: &str) -> Result<String> {
55:     let grammar = compile(grammar)?;
56:     Ok(grammar.to_string())
57: }
58: 
59: pub fn pretty_tree(tree: &Tree) -> Result<String> {
60:     Ok(tree.to_json_string()?)
61: }
62: 
63: pub fn load_boot() -> Result<Grammar> {
64:     Ok(boot_grammar()?)
65: }
66: 
67: pub fn boot_grammar_json() -> Result<String> {
68:     let boot = boot_grammar()?;
69:     let model: crate::json::tatsu::TatSuModel = boot.clone().try_into()?;
70:     Ok(serde_json::to_string_pretty(&model)?)
71: }
72: 
73: pub fn boot_grammar_pretty() -> Result<String> {
74:     let boot = boot_grammar()?;
75:     Ok(boot.to_string())
76: }
77: 
78: pub fn parse_input(grammar: &Grammar, input: &str) -> Result<Tree> {
79:     let cursor = StrCursor::new(input);
80:     let ctx = CoreCtx::new(cursor);
81: 
82:     match grammar.parse(ctx) {
83:         Ok(S(_, tree)) => Ok(tree),
84:         Err(failure) => Err(failure.into()),
85:     }
86: }

(End of file - total 86 lines)
</content>
```

---

## Assistant (Build · Big Pickle · 12.8s)

_Thinking:_

I've studied the main Rust source files. Let me continue following the rules by exploring the TatSu Python code in `./tatsu/tatsu/` and `./tatsu/tests/`.

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/tatsu/docs/left_recursion.rst"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/tatsu/docs/left_recursion.rst</path>
<type>file</type>
<content>
1: .. Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
2: .. SPDX-License-Identifier: BSD-4-Clause
3: 
4: .. include:: links.rst
5: 
6: 
7: Left Recursion
8: --------------
9: 
10: |TatSu| supports direct and indirect left recursion in grammar rules using
11: the algorithm described by *Nicolas Laurent* and *Kim Mens* in their 2015 paper_ *Parsing Expression Grammars Made Practical*.
12: 
13: The design and implementation of left recursion was done by `Vic Nightfall`_
14: with help and research by `Nicolas Laurent`_ on Autumn_, and `Philippe
15: Sigaud`_ on PEGGED_.
16: 
17: .. _Autumn: https://github.com/norswap/autumn
18: .. _PEGGED: https://github.com/PhilippeSigaud/Pegged/wiki/Left-Recursion
19: 
20: Left recursive rules produce left-associative parse trees (AST_), as most
21: users would expect, *except if some rules involved recurse on the right*.
22: 
23: .. _paper: http://norswap.com/pubs/sle2015.pdf
24: 
25: Left recursion support is enabled by default in |TatSu|. To disable it for a particular grammar, use the ``@@left_recursion`` directive:
26: 
27: .. code:: ocaml
28: 
29:     @@left_recursion :: False
30: 
31: 
32: .. warning::
33: 
34:     Not all left-recursive grammars that use the |TatSu| syntax are PEG_ (the same happens with right-recursive grammars). **The order of rules matters in PEG**.
35: 
36:     For right-recursive grammars the choices that parse the most input must come first. The same is true  for left-recursive grammars.

(End of file - total 36 lines)
</content>
```

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/tatsu/docs/semantics.rst"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/tatsu/docs/semantics.rst</path>
<type>file</type>
<content>
1: .. Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
2: .. SPDX-License-Identifier: BSD-4-Clause
3: 
4: .. include:: links.rst
5: 
6: Semantic Actions
7: ----------------
8: 
9: |TatSu| grammars lack notation for semantic actions. That is on purpose,
10: because semantic actions within a grammar obscure the declarative nature
11: of the context, and allow poor modularization from the parser-execution
12: perspective.
13: 
14: Semantic actions are defined in a class, and applied by passing an
15: object of the class to the ``parse()`` method of the parser as the
16: ``semantics=`` parameter. |TatSu| will invoke the method that matches
17: the name of the grammar rule every time the rule parses. The argument to
18: the method will be the `AST`_ constructed from the right-hand-side of
19: the rule:
20: 
21: .. code:: python
22: 
23:     class MySemantics:
24:         def some_rule_name(self, ast):
25:             return ''.join(ast)
26: 
27:         def _default(self, ast):
28:             pass
29: 
30: If there's no method matching the rule's name, |TatSu| will try to
31: invoke a ``_default()`` method if it's defined:
32: 
33: .. code:: python
34: 
35:     def _default(self, ast):
36:         ...
37: 
38: Nothing will happen if neither the per-rule method nor ``_default()``
39: are defined.
40: 
41: The per-rule methods in classes implementing the semantics provide
42: enough opportunity to do rule post-processing operations, like
43: verifications (for inadequate use of keywords as identifiers), or `AST`_
44: transformation:
45: 
46: .. code:: python
47: 
48:     class MyLanguageSemantics:
49:         def identifier(self, ast):
50:             if my_lange_module.is_keyword(ast):
51:                 raise FailedSemantics('"%s" is a keyword' % str(ast))
52:             return ast
53: 
54: For finer-grained control it's enough to declare more rules, as the
55: impact on the parsing times will be minimal.
56: 
57: If preprocessing is required at some point, it's enough to place
58: invocations of empty rules where appropriate:
59: 
60: .. code:: ebnf
61:    :force:
62: 
63:     myrule: first_part preproc {second_part}
64: 
65:     preproc: ()
66: 
67: The abstract parser will honor as a semantic action a method declared
68: as:
69: 
70: .. code:: python
71: 
72:     def preproc(self, ast):
73:         ...
74: 
75: 
76: .. _Abstract Syntax Tree: http://en.wikipedia.org/wiki/Abstract_syntax_tree
77: .. _AST: http://en.wikipedia.org/wiki/Abstract_syntax_tree
78: .. _PEG: http://en.wikipedia.org/wiki/Parsing_expression_grammar
79: .. _Python: http://python.org
80: .. _keywords: https://en.wikipedia.org/wiki/Reserved_word

(End of file - total 80 lines)
</content>
```

**Tool: glob**

**Input:**
```json
{
  "pattern": "**/*.py",
  "path": "/Users/apalala/art/tiexiu/tatsu/tatsu"
}
```

**Output:**
```
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/semantics.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/strtools.py
/Users/apalala/art/tiexiu/tatsu/tatsu/invoke_tasks.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/choice.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/model.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/basic.py
/Users/apalala/art/tiexiu/tatsu/tatsu/ngcodegen/ngparser_gen.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/named.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/state.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/ctx.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/context.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/version.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/indent.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/leftrec/leftrecmarker.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/infos.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/decorator/basic.py
/Users/apalala/art/tiexiu/tatsu/tatsu/railroads/walker.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/rulelike.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/parproc.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/walkers.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/unicode_characters.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/undefined.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/typetools.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/testing.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/timetools.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/regextools.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/safeeval.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/proxy.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/deprecate.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/color.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/debugging.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/boundeddict.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/configs.py
/Users/apalala/art/tiexiu/tatsu/tatsu/tool/ideps.py
/Users/apalala/art/tiexiu/tatsu/tatsu/tool/cli.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/asjson.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/abctools.py
/Users/apalala/art/tiexiu/tatsu/tatsu/tool/api.py
/Users/apalala/art/tiexiu/tatsu/tatsu/tool/bench.py
/Users/apalala/art/tiexiu/tatsu/tatsu/tool/__main__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/tool/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/semantics.py
/Users/apalala/art/tiexiu/tatsu/tatsu/railroads/unicode.py
/Users/apalala/art/tiexiu/tatsu/tatsu/railroads/railmath.py
/Users/apalala/art/tiexiu/tatsu/tatsu/railroads/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/parsing.py
/Users/apalala/art/tiexiu/tatsu/tatsu/pygraphviz_diagrams.py
/Users/apalala/art/tiexiu/tatsu/tatsu/objectmodel/node.py
/Users/apalala/art/tiexiu/tatsu/tatsu/objectmodel/builder.py
/Users/apalala/art/tiexiu/tatsu/tatsu/objectmodel/synth.py
/Users/apalala/art/tiexiu/tatsu/tatsu/objectmodel/basenode.py
/Users/apalala/art/tiexiu/tatsu/tatsu/objectmodel/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/ngcodegen/grammar_gen.py
/Users/apalala/art/tiexiu/tatsu/tatsu/ngcodegen/boilerplt.py
/Users/apalala/art/tiexiu/tatsu/tatsu/ngcodegen/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/ngcodegen/ngmodel_gen.py
/Users/apalala/art/tiexiu/tatsu/tatsu/model.py
/Users/apalala/art/tiexiu/tatsu/tatsu/input/textlines.py
/Users/apalala/art/tiexiu/tatsu/tatsu/input/buffer.py
/Users/apalala/art/tiexiu/tatsu/tatsu/input/text.py
/Users/apalala/art/tiexiu/tatsu/tatsu/input/infos.py
/Users/apalala/art/tiexiu/tatsu/tatsu/input/null.py
/Users/apalala/art/tiexiu/tatsu/tatsu/input/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/infos.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/syntax.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/pattern.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/leftrec/graphtools.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/math.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/closure.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/leftrec/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/deprecated.py
/Users/apalala/art/tiexiu/tatsu/tatsu/g2e/__main__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/g2e/g2etool.py
/Users/apalala/art/tiexiu/tatsu/tatsu/diagrams.py
/Users/apalala/art/tiexiu/tatsu/tatsu/exceptions.py
/Users/apalala/art/tiexiu/tatsu/tatsu/g2e/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/decorators.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/ast.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/cst.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/_engine.py
/Users/apalala/art/tiexiu/tatsu/tatsu/codegen/rendering.py
/Users/apalala/art/tiexiu/tatsu/tatsu/config.py
/Users/apalala/art/tiexiu/tatsu/tatsu/codegen/cgbase.py
/Users/apalala/art/tiexiu/tatsu/tatsu/codegen/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/_version.py
/Users/apalala/art/tiexiu/tatsu/tatsu/__main__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/_grammar.py
/Users/apalala/art/tiexiu/tatsu/tatsu/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/g2e/semantics.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/__init__.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/moduletools.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/misc.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/itertools.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/genericmain.py
/Users/apalala/art/tiexiu/tatsu/tatsu/util/common.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/decorator/rule.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/decorator/tatsumasu.py
/Users/apalala/art/tiexiu/tatsu/tatsu/contexts/decorator/__init__.py

(Results are truncated: showing first 100 results. Consider using a more specific path or pattern.)
```

---

## Assistant (Build · Big Pickle · 7.0s)

_Thinking:_

I've gathered a lot of information. Let me read a few more key TatSu Python files to understand the project better.

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/tatsu/tatsu/__init__.py"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/tatsu/tatsu/__init__.py</path>
<type>file</type>
<content>
1: # Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
2: # SPDX-License-Identifier: BSD-4-Clause
3: from __future__ import annotations
4: 
5: import sys
6: 
7: from . import config as parserconfig, input as tokenizing
8: from ._grammar import grammar, grammar_path
9: from ._version import __toolname__, __version__, version, version_info
10: from .contexts import ast as ast
11: from .contexts.decorator import isname, leftrec, name, nomemo, rule, tatsumasu
12: from .input import buffer as buffer, buffer as buffering, textlines as textlines
13: from .objectmodel import (
14:     NodeDataclassParams,
15:     NodeDataclassParams as TatSuDataclassParams,
16:     builder as builder,
17:     nodedataclass,
18:     nodedataclass as dataclass,
19:     nodedataclass as tatsudataclass,
20: )
21: from .tool import (  # pylint: disable=W0622
22:     asmodel,
23:     compile,
24:     gencode,
25:     genmodel,
26:     parse,
27:     tatsu_main,
28:     tatsu_main as main,
29:     to_python_model,
30:     to_python_sourcecode,
31: )
32: 
33: 
34: # HACK!
35: # NOTE: this is for backwrds compatibility with legacy generated parsers
36: sys.modules.update(  # noqa: RUF067
37:     {
38:         'tatsu.ast': ast,
39:         'tatsu.builder': builder,
40:         'tatsu.buffering': buffering,
41:         'tatsu.parserconfig': parserconfig,
42:         'tatsu.tokenizing': tokenizing,
43:         'tatsu.tokenizing.buffer': buffer,
44:         'tatsu.tokenizing.textlines': textlines,
45:     }
46: )
47: 
48: 
49: __all__ = [
50:     '__toolname__',
51:     '__version__',
52:     'asmodel',
53:     'builder',
54:     'compile',
55:     'gencode',
56:     'genmodel',
57:     'main',  # some unit tests want this
58:     'parse',
59:     'tatsu_main',
60:     'to_python_model',
61:     'to_python_sourcecode',
62:     'version',
63:     'version_info',
64:     'isname',
65:     'NodeDataclassParams',
66:     'TatSuDataclassParams',
67:     'dataclass',
68:     'nodedataclass',
69:     'tatsudataclass',
70:     'name',
71:     'leftrec',
72:     'nomemo',
73:     'rule',
74:     'tatsumasu',
75:     'grammar_path',
76:     'grammar',
77: ]

(End of file - total 77 lines)
</content>
```

**Tool: read**

**Input:**
```json
{
  "filePath": "/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/model.py"
}
```

**Output:**
```
<path>/Users/apalala/art/tiexiu/tatsu/tatsu/grammars/model.py</path>
<type>file</type>
<content>
1: # Copyright (c) 2017-2026 Juancarlo Añez (apalala@gmail.com)
2: # SPDX-License-Identifier: BSD-4-Clause
3: from __future__ import annotations
4: 
5: import weakref
6: from collections import defaultdict
7: from collections.abc import Iterable, Mapping
8: from copy import copy
9: from dataclasses import field
10: from functools import cached_property
11: from itertools import batched
12: from pathlib import Path
13: from types import SimpleNamespace
14: from typing import Any
15: 
16: from ..config import ParserConfig
17: from ..contexts import AST, CanParse, Ctx, Func, ParseContext, RuleInfo
18: from ..exceptions import GrammarError
19: from ..input import Text
20: from ..objectmodel import ModelBuilderSemantics, Node, nodedataclass
21: from ..util import indent, trim, typename
22: from .math import ffset, kdot
23: 
24: 
25: PEP8_LLEN = 72
26: 
27: _model_classes: list[type[Model]] = []
28: 
29: 
30: def model_classes() -> list[type[Model]]:
31:     return _model_classes
32: 
33: 
34: @nodedataclass
35: class Model(Node, CanParse):
36:     _lookahead: ffset = field(init=False, default_factory=set)
37:     _firstset: ffset = field(init=False, default_factory=set)
38:     _follow_set: ffset = field(init=False, default_factory=set)
39:     _grammar_ref: weakref.ref[Model] | None = field(init=False, default=None)
40: 
41:     def __init_subclass__(cls: type[Model], **kwargs):
42:         super().__init_subclass__(**kwargs)
43:         _model_classes.append(cls)
44: 
45:     @staticmethod
46:     def model_classes() -> list[type[Model]]:
47:         return _model_classes
48: 
49:     def follow_ref(self) -> Model:
50:         return self
51: 
52:     @cached_property
53:     def defines_single(self) -> list[str]:
54:         return []
55: 
56:     @cached_property
57:     def defines_list(self) -> list[str]:
58:         return []
59: 
60:     @property
61:     def grammar(self) -> Grammar:
62:         if self._grammar_ref is None:
63:             raise RuntimeError(f'{typename(self)} not fully initialized')
64:         grammar = self._grammar_ref()
65:         if not isinstance(grammar, Grammar):
66:             raise TypeError(f'{typename(self)} incorrectly initialized {grammar}')
67:         return grammar
68: 
69:     def parse(
70:         self,
71:         text: str | Text,
72:         /,
73:         *,
74:         start: str | None = None,
75:         config: Any = None,
76:         asmodel: bool = False,
77:         **settings: Any,
78:     ) -> Any:
79:         grammar = self.grammar
80:         assert isinstance(grammar, Grammar)
81:         return grammar.parse(
82:             text,
83:             start=start,
84:             config=config,
85:             asmodel=asmodel,
86:             **settings,
87:         )
88: 
89:     def _parse(self, ctx: Ctx) -> Any:
90:         return ()
91: 
92:     def _set_grammar(self, grammar: Grammar):
93:         assert isinstance(grammar, Grammar)
94: 
95:         self._grammar_ref = weakref.ref(grammar)
96:         for child in self.children():
97:             assert isinstance(child, Model)
98:             child._set_grammar(grammar)
99: 
100:     def _add_defined(self, ctx: Ctx):
101:         keys_single = self.defines_single
102:         keys_list = self.defines_list
103:         ctx.define(keys_single, keys_list)
104: 
105:     def lookahead(self, k: int = 1) -> ffset:
106:         if not self._lookahead:
107:             self._lookahead = kdot(self.firstset(k), self.followset(k), k)
108:         return self._lookahead
109: 
110:     @cached_property
111:     def lookaheadlist(self) -> list[Any]:
112:         return sorted(fl[0] for fl in self.lookahead(1) if fl)
113: 
114:     @cached_property
115:     def expecting(self) -> list[str]:
116:         return sorted(str(la) for la in self.lookaheadlist)
117: 
118:     @cached_property
119:     def expectingstr(self) -> str:
120:         return f'Expected one of: {' '.join(repr(s) for s in self.expecting)}'
121: 
122:     def firstset(self, k: int = 1) -> ffset:
123:         if not self._firstset:
124:             self._firstset = self._first(k, defaultdict(set))
125:         return self._firstset
126: 
127:     def followset(self, _k: int = 1) -> ffset:
128:         return self._follow_set
129: 
130:     def missing_rules(self, rulenames: set[str]) -> set[str]:
131:         assert rulenames
132:         return set()
133: 
134:     def _used_rule_names(self):
135:         return set()
136: 
137:     def _first(self, k: int, f: dict[str, ffset]) -> ffset:  # pyright: ignore[reportUnusedParameter]
138:         return set()
139: 
140:     def _follow(self, k, fl, a):
141:         return a
142: 
143:     def is_nullable(self) -> bool:
144:         return self._nullable
145: 
146:     @cached_property
147:     def _nullable(self) -> bool:
148:         return False
149: 
150:     # list of Model that can be invoked at the same position
151:     def callable_at_same_pos(self) -> list[Model]:
152:         return []
153: 
154:     def nodecount(self) -> int:
155:         return 1
156: 
157:     def pretty(self, lean: bool = False) -> str:
158:         return self._pretty(lean=lean)
159: 
160:     def pretty_lean(self):
161:         return self._pretty(lean=True)
162: 
163:     def _pretty(self, lean=False):
164:         return f'{typename(self)}: {id(self)}'
165: 
166:     def railroads(self) -> str:
167:         from .. import railroads
168: 
169:         return railroads.text(self)
170: 
171:     def optimized(self) -> Model:
172:         return self
173: 
174: 
175: @nodedataclass
176: class NULL(Model):
177:     def _parse(self, ctx: Ctx) -> Any:
178:         return ctx.fail() or ()
179: 
180:     def _pretty(self, lean=False):
181:         return NULL.__name__
182: 
183:     @cached_property
184:     def _nullable(self) -> bool:
185:         return False
186: 
187: 
188: @nodedataclass
189: class Void(Model):
190:     def _parse(self, ctx: Ctx) -> Any:
191:         return ctx.void()
192: 
193:     def _pretty(self, lean=False):
194:         return '()'
195: 
196:     @cached_property
197:     def _nullable(self) -> bool:
198:         return True
199: 
200: 
201: @nodedataclass
202: class Box(Model):
203:     name: str | None = field(init=False, default=None)
204:     exp: Model = field(default_factory=NULL)
205: 
206:     def __post_init__(self):
207:         noexp = not self.exp or isinstance(self.exp, NULL)
208:         if noexp and not isinstance(self.ast, AST):
209:             self.exp = self.ast
210:         super().__post_init__()
211:         if isinstance(self.ast, AST):
212:             assert self.exp == self.ast.exp
213:         # assert not isinstance(self.exp, NULL), self.ast
214:         self.exp = self.exp if self.exp is not None else NULL()
215:         assert self.exp is not None, f'{typename(self)}({self.exp})'
216:         assert self.exp, repr(self)
217: 
218:     def _parse(self, ctx: Ctx) -> Any:
219:         return self.exp._parse(ctx)
220: 
221:     @cached_property
222:     def defines_single(self) -> list[str]:
223:         return self.exp.defines_single
224: 
225:     @cached_property
226:     def defines_list(self) -> list[str]:
227:         return self.exp.defines_list
228: 
229:     def missing_rules(self, rulenames: set[str]) -> set[str]:
230:         assert isinstance(self.exp, Model), f'{self!r}:{self.exp=!r}'
231:         return self.exp.missing_rules(rulenames)
232: 
233:     def _used_rule_names(self):
234:         return self.exp._used_rule_names()
235: 
236:     def _first(self, k: int, f: dict[str, ffset]) -> ffset:
237:         return self.exp._first(k, f)
238: 
239:     def _follow(self, k, fl, a):
240:         return self.exp._follow(k, fl, a)
241: 
242:     def nodecount(self) -> int:
243:         return 1 + self.exp.nodecount()
244: 
245:     def _pretty(self, lean=False):
246:         return self.exp._pretty(lean=lean)
247: 
248:     @cached_property
249:     def _nullable(self) -> bool:
250:         return self.exp._nullable
251: 
252:     def callable_at_same_pos(
253:         self,
254:         rulemap: Mapping[str, Rule] | None = None,
255:     ) -> list[Model]:
256:         return [self.exp]
257: 
258:     def optimized(self) -> Model:
259:         return self.clone(exp=self.exp.optimized())
260: 
261: 
262: @nodedataclass
263: class Synth(Box):
264:     """A synthetic placeholder for an unresolved rule."""
265: 
266:     pass
267: 
268: 
269: @nodedataclass
270: class NamedBox(Box):
271:     name: str = field(default='')  # type: ignore
272: 
273: 
274: @nodedataclass
275: class Rule(NamedBox):
276:     params: tuple[str, ...] = field(default_factory=tuple)
277:     kwparams: dict[str, Any] = field(default_factory=dict)
278:     decorators: list[str] = field(default_factory=list)
279:     base: str | None = None
280: 
281:     # these come from parsing a grammar
282:     is_name: bool = False
283:     is_tokn: bool = False
284:     no_memo: bool = False
285: 
286:     # these are used by left recursion analisis
287:     is_memo: bool = True
288:     is_lrec: bool = False
289: 
290:     def __post_init__(self):
291:         super().__post_init__()
292:         self.params = self.params or ()
293:         self.kwparams = self.kwparams or {}
294:         self.decorators = self.decorators or []
295: 
296:         self.is_name = bool(self.is_name) or 'name' in self.decorators
297:         self.is_name = bool(self.is_name) or 'isname' in self.decorators
298: 
299:         if not self.kwparams:
300:             self.kwparams = {}
301:         elif not isinstance(self.kwparams, dict):
302:             self.kwparams = dict(self.kwparams)
303:         assert isinstance(self.kwparams, dict), f'{typename(self)}: {self.kwparams=!r}'
304:         self.is_tokn = self.is_tokn or self.name.lstrip('_')[:1].isupper()
305: 
306:     def missing_rules(self, rulenames: set[str]) -> set[str]:
307:         return self.exp.missing_rules(rulenames)
308: 
309:     def _parse(self, ctx: Ctx) -> Any:
310:         return self._parse_rhs(ctx, self.exp)
311: 
312:     def _parse_rhs(self, ctx: Ctx, exp: Model) -> Any:
313:         # note: BasedRule._parse() calls _parse_rhs() so ruleinfo is a mix
314:         ruleinfo = RuleInfo(
315:             name=self.name,
316:             instance=exp,
317:             func=exp._parse,
318:             no_memo=self.no_memo,
319:             is_name=self.is_name,
320:             is_tokn=self.is_tokn,
321:             params=self.params,
322:             kwparams=self.kwparams,
323:             is_lrec=self.is_lrec,
324:             is_memo=self.memoizable,
325:         )
326:         return ctx.call(ruleinfo)
327: 
328:     def _first(self, k, f) -> ffset:
329:         self._firstset = self.exp._first(k, f) | f[self.name]
330:         return self._firstset
331: 
332:     def _follow(self, k, fl, a):
333:         return self.exp._follow(k, fl, fl[self.name])
334: 
335:     @cached_property
336:     def _nullable(self) -> bool:
337:         return self.exp._nullable
338: 
339:     @property
340:     def memoizable(self):
341:         return self.is_memo and not self.no_memo and not self.is_lrec
342: 
343:     def optimized(self) -> Rule:
344:         clone = copy(self)
345:         clone.exp = self.exp.optimized()
346:         return clone
347: 
348:     @staticmethod
349:     def param_repr(p):
350:         if isinstance(p, int | float) or (isinstance(p, str) and p.isalnum()):
351:             return str(p)
352:         else:
353:             return repr(p)
354: 
355:     def _pretty(self, lean=False):
356:         str_template = "{is_name}{name}{base}{params}:{exp}"
357: 
358:         if lean:
359:             params = ''
360:         else:
361:             params = (
362:                 ', '.join(self.param_repr(p) for p in self.params)
363:                 if self.params
364:                 else ''
365:             )
366: 
367:             skwparams = ''
368:             if self.kwparams:
369:                 skwparams = ', '.join(
370:                     f'{k}={self.param_repr(v)}' for (k, v) in self.kwparams.items()
371:                 )
372: 
373:             if params and skwparams:
374:                 params = f'[{params}, {skwparams}]'
375:             elif skwparams:
376:                 params = f'[{skwparams}]'
377:             elif params:
378:                 params = f'[{params}]'
379: 
380:         base = f' < {self.base!s}' if self.base else ''
381: 
382:         exp = self.exp._pretty(lean=lean)
383:         if len(exp.splitlines()) > 1:
384:             exp = '\n' + indent(exp)
385:         else:
386:             exp = ' ' + exp
387: 
388:         return trim(str_template).format(
389:             name=self.name,
390:             base=base,
391:             params=params,
392:             exp=exp,
393:             is_name='@name\n' if self.is_name else '',
394:         )
395: 
396: 
397: @nodedataclass
398: class Patterns(Model):
399:     whitespace: str | None = None
400:     comments: str | None = None
401:     eol_comments: str | None = None
402: 
403: 
404: @nodedataclass
405: class Grammar(Model):
406:     name: str = 'MyTest'
407:     directives: dict[str, Any] = field(default_factory=dict)
408:     keywords: tuple[str, ...] = field(default_factory=tuple)
409:     rules: tuple[Rule, ...] = field(default_factory=tuple)
410: 
411:     def __init__(
412:         self,
413:         name=None,
414:         rules: Iterable[Rule] | None = None,
415:         *,
416:         config: Any = None,
417:         directives: dict | None = None,
418:         keywords: tuple[str, ...] | None = None,
419:         **settings: Any,
420:     ):
421:         super().__init__()
422:         config = config if isinstance(config, ParserConfig) else ParserConfig()
423:         assert isinstance(rules, Iterable), f'{type(rules)!r} {rules!r}'
424: 
425:         directives = directives or {}
426:         assert isinstance(directives, dict)
427:         self.directives = directives
428: 
429:         config = ParserConfig.new(config=config, **settings)
430:         config = config.hard_override(**directives)
431:         self._config: ParserConfig = config
432: 
433:         keywords = keywords or config.keywords or ()
434:         keywords = tuple(k for k in keywords or () if k)
435:         assert isinstance(keywords, tuple)
436:         if self.config.ignorecase:
437:             keywords = tuple(k.upper() for k in keywords if k)
438:         keywords = tuple(sorted(set(keywords)))
439:         assert isinstance(keywords, tuple)
440:         self.keywords = keywords
441: 
442:         # note: Ctx needs keywords in config
443:         self._config = self._config.override(keywords=self.keywords)
444: 
445:         self.rules = tuple(rules)  # type: ignore
446:         rulemap = {rule.name: rule for rule in rules}
447:         self._rule = SimpleNamespace(**rulemap)
448:         self._rulemap = self._rule.__dict__
449: 
450:         self.name = self._resolve_name(name)
451: 
452:         missing: set[str] = self.missing_rules(set(self.rulemap))
453:         if missing:
454:             msg = '\n'.join(['', *sorted(missing)])
455:             raise GrammarError('Unknown rules, no parser generated:' + msg)
456: 
457:         self._set_grammar(self)
458:         self._calc_lookahead_sets()
459:         self._mark_left_recursion()
460: 
461:     def configure(self, config: ParserConfig | None = None, **settings: Any):
462:         self._config.merge_config(config)
463:         self._config.merge(**settings)
464: 
465:     def _update_patterns(self):
466:         if not hasattr(self, 'patterns'):
467:             self.patterns = Patterns()
468:         self.patterns.whitespace = (
469:             self.patterns.whitespace
470:             or self.config.whitespace
471:             or self.directives.get('whitespace')
472:         )
473:         self.patterns.comments = (
474:             self.patterns.comments
475:             or self.config.comments
476:             or self.directives.get('comments')
477:         )
478:         self.patterns.eol_comments = (
479:             self.patterns.eol_comments
480:             or self.config.eol_comments
481:             or self.directives.get('eol_comments')
482:         )
483: 
484:     @property
485:     def config(self) -> ParserConfig:
486:         return self._config
487: 
488:     @property
489:     def rulemap(self) -> dict[str, Rule]:
490:         return self._rulemap
491: 
492:     @property
493:     def rule(self) -> SimpleNamespace:
494:         return self._rule
495: 
496:     @property
497:     def semantics(self) -> Any:
498:         return self.config.semantics
499: 
500:     @semantics.setter
501:     def semantics(self, value: Any):
502:         self.config.semantics = value  # type: ignore
503: 
504:     @property
505:     def first_sets(self):
506:         return self._firstset
507: 
508:     def missing_rules(self, rulenames: set[str]) -> set[str]:
509:         return set().union(*[rule.missing_rules(rulenames) for rule in self.rules])
510: 
511:     def used_rules(self) -> list[Rule]:
512:         used = self._used_rule_names()
513:         return [rule for rule in self.rules if rule.name in used]
514: 
515:     def _resolve_name(self, name: str | None) -> str:
516:         if name is None:
517:             name = self.directives.get('grammar')
518:         if name is None:
519:             name = self.config.name
520:         if name is None and self.config.source is not None:
521:             name = Path(self.config.source).stem
522:         if name is None:
523:             name = 'My'
524:         return name
525: 
526:     def _mark_left_recursion(self):
527:         from .leftrec import mark_left_recursion
528: 
529:         leftrect_rules = mark_left_recursion(self.rules)
530:         if leftrect_rules and not self.config.left_recursion:
531:             config = self.config
532:             raise GrammarError(
533:                 f'{config.left_recursion=}'
534:                 f' but found left-recursive rules'
535:                 f' {', '.join(repr(r.name) for r in leftrect_rules)}!'
536:             )
537: 
538:     def _used_rule_names(self) -> set[str]:
539:         if not self.rules:
540:             return set()
541: 
542:         used = {'start', self.rules[0].name}
543:         prev: set[str] = set()
544:         while used != prev:
545:             prev = used
546:             used |= set().union(
547:                 *[rule._used_rule_names() for rule in self.rules if rule.name in used],
548:             )
549:         return used
550: 
551:     def _calc_lookahead_sets(self, k: int = 1):
552:         self._calc_first_sets(k=k)
553:         self._calc_follow_sets(k=k)
554:         for rule in self.rules:
555:             if rule.lookaheadlist is None:
556:                 raise GrammarError(f'Impossible lookahead in rule {rule.name}')
557: 
558:     def _calc_first_sets(self, k: int = 1):
559:         f: dict[str, ffset] = defaultdict(set)
560:         f1 = None
561:         while f1 != f:
562:             f1 = copy(f)
563:             for rule in self.rules:
564:                 f[rule.name] |= rule._first(k, f)
565: 
566:         # cache results
567:         for rule in self.rules:
568:             rule._firstset = f[rule.name]
569: 
570:     def _calc_follow_sets(self, k: int = 1):
571:         fl: dict[str, ffset] = defaultdict(set)
572:         fl1 = None
573:         while fl1 != fl:
574:             fl1 = copy(fl)
575:             for rule in self.rules:
576:                 rule._follow(k, fl, set())
577: 
578:         # cache results
579:         for rule in self.rules:
580:             rule._follow_set = fl[rule.name]
581: 
582:     def parse(
583:         self,
584:         text: str | Text,
585:         /,
586:         *,
587:         start: str | None = None,
588:         config: Any = None,
589:         asmodel: bool = False,
590:         **settings: Any,
591:     ) -> Any:
592:         config = self.config.override_config(config)
593:         assert isinstance(config, ParserConfig)
594:         # note: bw-comp: allow overriding directives
595:         config = config.override(start=start, **settings)
596: 
597:         if isinstance(config.semantics, type):
598:             raise TypeError(
599:                 'semantics must be an object instance or None, '
600:                 f'not class {config.semantics!r}',
601:             )
602: 
603:         start = config.effective_start_rule_name()
604:         if start is None:
605:             start = self.rules[0].name
606:             config = config.override(
607:                 start=start,
608:                 start_rule=None,
609:                 rule_name=None,
610:             )
611: 
612:         self._config = config
613:         ctx = self.newctx(asmodel=asmodel)
614:         assert isinstance(ctx, Ctx)
615:         return ctx.parse(text, config=config)
616: 
617:     def newctx(self, asmodel: bool = True) -> Ctx:
618:         return ModelContext(self.rules, config=self.config, asmodel=asmodel)
619: 
620:     def nodecount(self) -> int:
621:         return 1 + sum(r.nodecount() for r in self.rules)
622: 
623:     def _pretty(self, lean: bool = False) -> str:
624:         regex_directives = {'comments', 'eol_comments', 'whitespace'}
625:         string_directives = {'namechars'}
626: 
627:         directives = ''
628:         for name, value in self.directives.items():
629:             if name in regex_directives:
630:                 if '/' in value:
631:                     directives += f'@@{name} :: ?"{value}"\n'
632:                 else:
633:                     directives += f'@@{name} :: /{value}/\n'
634:             elif name in string_directives:
635:                 directives += f'@@{name} :: {value!r}\n'
636:             else:
637:                 directives += f'@@{name} :: {value}\n'
638:         if directives:
639:             directives += '\n'
640: 
641:         keywordsets = []
642:         for batch in batched(sorted(self.keywords), 8):
643:             b = [repr(k) for k in batch if k and k.strip()]
644:             if not b:
645:                 continue
646:             keywordsets += [f'@@keyword :: {' '.join(b)}']
647:         keywords = f"\n\n{'\n'.join(keywordsets)}\n\n" if keywordsets else ""
648: 
649:         rules = (
650:             '\n\n'.join(str(rule._pretty(lean=lean)) for rule in self.rules)
651:         ).rstrip() + '\n\n'
652:         return directives + keywords + rules
653: 
654:     def optimized(self) -> Model:
655:         rules = [r.optimized() for r in self.rules]
656:         return Grammar(
657:             name=self.name,
658:             rules=rules,
659:             config=self.config,
660:             directives=self.directives,
661:         )
662: 
663: 
664: class ModelContext(ParseContext):
665:     def __init__(
666:         self,
667:         rules: Iterable[Rule],
668:         /,
669:         start: str | None = None,
670:         config: ParserConfig | None = None,
671:         asmodel: bool = False,
672:         **settings: Any,
673:     ):
674:         config = ParserConfig.new(config, **settings)
675:         assert isinstance(config, ParserConfig)
676:         super().__init__(start=start, config=config, asmodel=asmodel)
677:         if not self.config.semantics and asmodel:
678:             self.config.semantics = ModelBuilderSemantics()
679: 
680:         self._rulemap: dict[str, Rule] = {rule.name: rule for rule in rules}
681: 
682:     @property
683:     def rulemap(self) -> dict[str, Rule]:
684:         return self._rulemap
685: 
686:     def find_rule(self, name: str) -> Func:
687:         return self.rulemap[name]._parse

(End of file - total 687 lines)
</content>
```

---

## Assistant (Build · Big Pickle · 7.7s)

_Thinking:_

I've now studied the key files per the rules in AGENTS.md. Let me provide a summary of what I've learned.

I've completed studying the files per the rules in `AGENTS.md`:

**Summary:**

1. **Project Overview**: TieXiu is a Rust port of TatSu PEG parser generator with PyO3/maturin Python bindings. Currently in Alpha status with core parsing engine, grammar model, and left recursion support implemented.

2. **Architecture**: Rust modules include `api.rs`, `peg/` (grammar model), `state/` (parsing context), `input/` (cursor trait), `trees/` (CST/AST), `json/` (TatSu JSON import), `ui/`, `util/`

3. **Key Features**: 16-byte handle size, CoW state transitions, packrat/memoization, left recursion support, thread-safe grammars

4. **TatSu Python Source**: Located at `./tatsu/tatsu/` with grammars, contexts, parsing, codegen modules. The model system (`grammars/model.py`) defines Grammar, Rule, and various expression types as dataclasses.

5. **Session History**: Previous work on renaming test files from `*_tests.rs` to `*.rs` pattern (not yet complete - old files remain)

---

## User

Which are the "*_test.rs" files remaining under ./wip/ ?

---

## Assistant (Build · Big Pickle · 5.6s)

_Thinking:_

The user wants to see which *_test.rs files remain under ./wip/. Let me search for them.

**Tool: glob**

**Input:**
```json
{
  "pattern": "**/*_test*.rs",
  "path": "/Users/apalala/art/tiexiu/wip"
}
```

**Output:**
```
No files found
```

---

## Assistant (Build · Big Pickle · 2.9s)

_Thinking:_

No files found with _test*.rs pattern. Let me try a broader search.

**Tool: glob**

**Input:**
```json
{
  "pattern": "**/*.rs",
  "path": "/Users/apalala/art/tiexiu/wip"
}
```

**Output:**
```
/Users/apalala/art/tiexiu/wip/tests/z_bootstrap.rs
/Users/apalala/art/tiexiu/wip/tests/walker.rs
/Users/apalala/art/tiexiu/wip/tests/typetools.rs
/Users/apalala/art/tiexiu/wip/tests/syntax.rs
/Users/apalala/art/tiexiu/wip/tests/strtools.rs
/Users/apalala/art/tiexiu/wip/tests/string.rs
/Users/apalala/art/tiexiu/wip/tests/semantics.rs
/Users/apalala/art/tiexiu/wip/tests/safeeval.rs
/Users/apalala/art/tiexiu/wip/tests/safeeval_stress.rs
/Users/apalala/art/tiexiu/wip/tests/safe_name.rs
/Users/apalala/art/tiexiu/wip/tests/return_value.rs
/Users/apalala/art/tiexiu/wip/tests/regexp.rs
/Users/apalala/art/tiexiu/wip/tests/railroads.rs
/Users/apalala/art/tiexiu/wip/tests/pretty.rs
/Users/apalala/art/tiexiu/wip/tests/pickle.rs
/Users/apalala/art/tiexiu/wip/tests/pattern.rs
/Users/apalala/art/tiexiu/wip/tests/parsing.rs
/Users/apalala/art/tiexiu/wip/tests/parser_equivalence.rs
/Users/apalala/art/tiexiu/wip/tests/parameter.rs
/Users/apalala/art/tiexiu/wip/tests/ngmodel.rs
/Users/apalala/art/tiexiu/wip/tests/model.rs
/Users/apalala/art/tiexiu/wip/tests/misc.rs
/Users/apalala/art/tiexiu/wip/tests/lookahead.rs
/Users/apalala/art/tiexiu/wip/tests/left_recursion.rs
/Users/apalala/art/tiexiu/wip/tests/keyword.rs
/Users/apalala/art/tiexiu/wip/tests/join.rs
/Users/apalala/art/tiexiu/wip/tests/ideps.rs
/Users/apalala/art/tiexiu/wip/tests/firstfollow.rs
/Users/apalala/art/tiexiu/wip/tests/error.rs
/Users/apalala/art/tiexiu/wip/tests/ebnf.rs
/Users/apalala/art/tiexiu/wip/tests/directive.rs
/Users/apalala/art/tiexiu/wip/tests/diagram.rs
/Users/apalala/art/tiexiu/wip/tests/defines.rs
/Users/apalala/art/tiexiu/wip/tests/constants.rs
/Users/apalala/art/tiexiu/wip/tests/codegen.rs
/Users/apalala/art/tiexiu/wip/tests/cli.rs
/Users/apalala/art/tiexiu/wip/tests/buffering.rs
/Users/apalala/art/tiexiu/wip/tests/bench_parse.rs
/Users/apalala/art/tiexiu/wip/tests/ast.rs
/Users/apalala/art/tiexiu/wip/tests/asjson.rs
/Users/apalala/art/tiexiu/wip/tests/alerts.rs
/Users/apalala/art/tiexiu/wip/tests/mod.rs
```

---

## Assistant (Build · Big Pickle · 3.3s)

_Thinking:_

So the glob found no files matching `*_test*.rs` pattern. But the session log mentioned `*_tests.rs` files. Let me check for `*_tests.rs` specifically.

**Tool: glob**

**Input:**
```json
{
  "pattern": "**/*_tests.rs",
  "path": "/Users/apalala/art/tiexiu/wip"
}
```

**Output:**
```
No files found
```

---

## Assistant (Build · Big Pickle · 2.8s)

_Thinking:_

No `*_tests.rs` files remain. Let me also check for any remaining files with different patterns.

No `*_tests.rs` files remain under `./wip/`. The renaming is complete - all 40 test files have been renamed from `*_tests.rs` to `*.rs`.

---

