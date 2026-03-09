use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
struct CpuProfile {
    nodes: Vec<CpuNode>,
    #[serde(default)]
    samples: Vec<u64>,
    #[serde(default, rename = "timeDeltas")]
    time_deltas: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CpuNode {
    id: u64,
    #[serde(rename = "callFrame")]
    call_frame: CallFrame,
    #[serde(default)]
    children: Vec<u64>,
    #[serde(default, rename = "hitCount")]
    hit_count: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CallFrame {
    #[serde(default, rename = "functionName")]
    function_name: String,
    #[serde(default)]
    url: String,
    #[serde(default, rename = "lineNumber")]
    line_number: i64,
    #[serde(default, rename = "columnNumber")]
    column_number: i64,
}

#[derive(Debug, Serialize, Clone)]
struct HotspotRow {
    id: u64,
    #[serde(rename = "functionName")]
    function_name: String,
    url: String,
    #[serde(rename = "lineNumber")]
    line_number: i64,
    #[serde(rename = "columnNumber")]
    column_number: i64,
    #[serde(rename = "selfTimeUs")]
    self_time_us: u64,
    #[serde(rename = "selfTimeMs")]
    self_time_ms: f64,
    #[serde(rename = "selfPct")]
    self_pct: f64,
}

#[derive(Debug, Serialize)]
struct SummaryResult {
    #[serde(rename = "totalTimeMs")]
    total_time_ms: f64,
    #[serde(rename = "totalSamples")]
    total_samples: usize,
    hotspots: Vec<HotspotRow>,
}

#[derive(Debug, Serialize)]
struct UrlRow {
    url: String,
    #[serde(rename = "selfTimeUs")]
    self_time_us: u64,
    #[serde(rename = "selfTimeMs")]
    self_time_ms: f64,
    #[serde(rename = "selfPct")]
    self_pct: f64,
}

#[derive(Debug, Serialize)]
struct HotPathRow {
    path: Vec<String>,
    #[serde(rename = "timeUs")]
    time_us: u64,
    #[serde(rename = "timeMs")]
    time_ms: f64,
    pct: f64,
}

#[derive(Debug, Serialize)]
struct BottomUpCaller {
    caller: String,
    #[serde(rename = "timeUs")]
    time_us: u64,
}

#[derive(Debug, Serialize)]
struct BottomUpRow {
    function: String,
    #[serde(rename = "selfTimeUs")]
    self_time_us: u64,
    #[serde(rename = "selfTimeMs")]
    self_time_ms: f64,
    #[serde(rename = "totalTimeUs")]
    total_time_us: u64,
    #[serde(rename = "totalTimeMs")]
    total_time_ms: f64,
    #[serde(rename = "mainCallers")]
    main_callers: Vec<BottomUpCaller>,
}

fn main() {
    if let Err(err) = run() {
        let _ = writeln!(io::stderr(), "cpuprofile-mcp fatal error: {err}");
    }
}

fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();

    loop {
        let msg = match read_message(&mut input)? {
            Some(v) => v,
            None => break,
        };

        if let Some(response) = handle_jsonrpc(msg) {
            write_message(&mut output, &response)?;
        }
    }

    Ok(())
}

fn handle_jsonrpc(msg: Value) -> Option<Value> {
    let id = msg.get("id").cloned();
    let method = msg.get("method").and_then(Value::as_str);

    if method.is_none() {
        return id.map(|id| {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": { "code": -32600, "message": "Invalid Request: missing method" }
            })
        });
    }

    let params = msg.get("params").cloned().unwrap_or_else(|| json!({}));
    let method = method.unwrap_or_default();

    let result = match method {
        "initialize" => Ok(json!({
            "protocolVersion": "2024-11-05",
            "capabilities": { "tools": { "listChanged": false } },
            "serverInfo": { "name": "cpuprofile-mcp", "version": "0.3.0" }
        })),
        "notifications/initialized" => return None,
        "ping" => Ok(json!({})),
        "tools/list" => Ok(tools_list()),
        "tools/call" => handle_tool_call(&params),
        _ => Err((-32601, format!("Method not found: {method}"))),
    };

    if id.is_none() {
        return None;
    }

    Some(match result {
        Ok(value) => json!({ "jsonrpc": "2.0", "id": id, "result": value }),
        Err((code, message)) => json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": code, "message": message }
        }),
    })
}

fn tools_list() -> Value {
    json!({
        "tools": [
            tool_schema("summarize_cpuprofile", "对 .cpuprofile 做整体摘要分析。", json!({
                "type":"object","properties":{
                    "profilePath":{"type":"string"},
                    "topN":{"type":"integer","minimum":1,"maximum":100,"default":20}
                },"required":["profilePath"]
            })),
            tool_schema("inspect_cpuprofile_node", "查看指定 profiling 节点的详细信息。", json!({
                "type":"object","properties":{
                    "profilePath":{"type":"string"},
                    "nodeId":{"type":"integer","minimum":1}
                },"required":["profilePath","nodeId"]
            })),
            tool_schema("summarize_cpuprofile_by_url", "按 URL / 文件维度聚合 CPU 时间。", json!({
                "type":"object","properties":{
                    "profilePath":{"type":"string"},
                    "topN":{"type":"integer","minimum":1,"maximum":100,"default":20}
                },"required":["profilePath"]
            })),
            tool_schema("get_cpuprofile_hot_paths", "提取最热调用路径。", json!({
                "type":"object","properties":{
                    "profilePath":{"type":"string"},
                    "topN":{"type":"integer","minimum":1,"maximum":100,"default":20}
                },"required":["profilePath"]
            })),
            tool_schema("get_cpuprofile_bottom_up", "生成 bottom-up 视图。", json!({
                "type":"object","properties":{
                    "profilePath":{"type":"string"},
                    "topN":{"type":"integer","minimum":1,"maximum":100,"default":20}
                },"required":["profilePath"]
            })),
            tool_schema("diff_cpuprofiles", "对比两份 .cpuprofile。", json!({
                "type":"object","properties":{
                    "beforeProfilePath":{"type":"string"},
                    "afterProfilePath":{"type":"string"},
                    "topN":{"type":"integer","minimum":1,"maximum":100,"default":20}
                },"required":["beforeProfilePath","afterProfilePath"]
            })),
            tool_schema("diagnose_cpuprofile", "自动生成性能瓶颈诊断。", json!({
                "type":"object","properties":{
                    "profilePath":{"type":"string"},
                    "topN":{"type":"integer","minimum":1,"maximum":100,"default":10}
                },"required":["profilePath"]
            }))
        ]
    })
}

fn tool_schema(name: &str, description: &str, input_schema: Value) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema,
    })
}

fn handle_tool_call(params: &Value) -> Result<Value, (i64, String)> {
    let name = params
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| (-32602, "tools/call missing name".to_string()))?;
    let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));

    match name {
        "summarize_cpuprofile" => {
            let profile_path = get_str(&args, "profilePath", name)?;
            let top_n = get_top_n(&args, 20);
            let profile = load_profile(profile_path).map_err(|e| (-32603, e))?;
            let summary = summarize_profile_from_data(&profile, top_n);
            Ok(text_and_data(summary_to_markdown(&summary), json!(summary)))
        }
        "inspect_cpuprofile_node" => {
            let profile_path = get_str(&args, "profilePath", name)?;
            let node_id = args
                .get("nodeId")
                .and_then(Value::as_u64)
                .ok_or_else(|| (-32602, "inspect_cpuprofile_node requires nodeId".to_string()))?;
            let profile = load_profile(profile_path).map_err(|e| (-32603, e))?;
            Ok(inspect_node(&profile, node_id))
        }
        "summarize_cpuprofile_by_url" => {
            let profile_path = get_str(&args, "profilePath", name)?;
            let top_n = get_top_n(&args, 20);
            let profile = load_profile(profile_path).map_err(|e| (-32603, e))?;
            let total_time_us: u64 = profile.time_deltas.iter().copied().sum();
            let rows = summarize_by_url(&profile, top_n);
            let markdown = by_url_to_markdown(&rows, total_time_us as f64 / 1000.0, profile.samples.len());
            Ok(text_and_data(markdown, json!({
                "totalTimeMs": total_time_us as f64 / 1000.0,
                "totalSamples": profile.samples.len(),
                "byUrl": rows
            })))
        }
        "get_cpuprofile_hot_paths" => {
            let profile_path = get_str(&args, "profilePath", name)?;
            let top_n = get_top_n(&args, 20);
            let profile = load_profile(profile_path).map_err(|e| (-32603, e))?;
            let rows = get_hot_paths(&profile, top_n);
            Ok(text_and_data(hot_paths_markdown(&rows), json!({"paths": rows})))
        }
        "get_cpuprofile_bottom_up" => {
            let profile_path = get_str(&args, "profilePath", name)?;
            let top_n = get_top_n(&args, 20);
            let profile = load_profile(profile_path).map_err(|e| (-32603, e))?;
            let rows = get_bottom_up(&profile, top_n);
            Ok(text_and_data(bottom_up_markdown(&rows), json!({"functions": rows})))
        }
        "diff_cpuprofiles" => {
            let before = get_str(&args, "beforeProfilePath", name)?;
            let after = get_str(&args, "afterProfilePath", name)?;
            let top_n = get_top_n(&args, 20);
            let before_p = load_profile(before).map_err(|e| (-32603, e))?;
            let after_p = load_profile(after).map_err(|e| (-32603, e))?;
            let diff = diff_profiles(&before_p, &after_p, top_n);
            Ok(text_and_data(diff_markdown(&diff), json!(diff)))
        }
        "diagnose_cpuprofile" => {
            let profile_path = get_str(&args, "profilePath", name)?;
            let top_n = get_top_n(&args, 10);
            let profile = load_profile(profile_path).map_err(|e| (-32603, e))?;
            let diagnosis = diagnose_profile(&profile, top_n);
            Ok(text_and_data(diagnosis["report"].as_str().unwrap_or_default().to_string(), diagnosis))
        }
        _ => Err((-32601, format!("Unknown tool: {name}"))),
    }
}

fn text_and_data(text: String, structured: Value) -> Value {
    json!({
        "content": [{"type":"text","text":text}],
        "structuredContent": structured
    })
}

fn get_str<'a>(v: &'a Value, key: &str, tool: &str) -> Result<&'a str, (i64, String)> {
    v.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| (-32602, format!("{tool} requires {key}")))
}

fn get_top_n(v: &Value, default: u64) -> usize {
    v.get("topN")
        .and_then(Value::as_u64)
        .unwrap_or(default)
        .clamp(1, 100) as usize
}

fn load_profile(profile_path: &str) -> Result<CpuProfile, String> {
    let path = Path::new(profile_path);
    let raw = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
    let profile: CpuProfile = serde_json::from_str(&raw)
        .map_err(|e| format!("Failed to parse {} as JSON: {e}", path.display()))?;

    if profile.samples.len() != profile.time_deltas.len() {
        return Err(format!(
            "Invalid .cpuprofile: samples ({}) and timeDeltas ({}) length mismatch",
            profile.samples.len(),
            profile.time_deltas.len()
        ));
    }

    Ok(profile)
}

fn display_name(frame: &CallFrame) -> String {
    let fn_name = if frame.function_name.is_empty() {
        "(anonymous)"
    } else {
        frame.function_name.as_str()
    };
    let url = if frame.url.is_empty() { "(internal)" } else { frame.url.as_str() };
    format!("{} @ {}:{}:{}", fn_name, url, frame.line_number + 1, frame.column_number + 1)
}

fn build_parent_map(profile: &CpuProfile) -> HashMap<u64, u64> {
    let mut parent = HashMap::new();
    for node in &profile.nodes {
        for child in &node.children {
            parent.entry(*child).or_insert(node.id);
        }
    }
    parent
}

fn inspect_node(profile: &CpuProfile, node_id: u64) -> Value {
    let parent = build_parent_map(profile);
    let node = profile.nodes.iter().find(|n| n.id == node_id);
    match node {
        Some(node) => {
            let text = format!(
                "Node {}: {} (hitCount={}, parent={:?}, children={:?})",
                node.id,
                display_name(&node.call_frame),
                node.hit_count,
                parent.get(&node.id),
                node.children
            );
            json!({
                "content": [{"type":"text","text": text}],
                "structuredContent": {
                    "found": true,
                    "nodeId": node.id,
                    "functionName": if node.call_frame.function_name.is_empty() { "(anonymous)" } else { &node.call_frame.function_name },
                    "url": if node.call_frame.url.is_empty() { "(internal)" } else { &node.call_frame.url },
                    "line": node.call_frame.line_number + 1,
                    "column": node.call_frame.column_number + 1,
                    "hitCount": node.hit_count,
                    "parentId": parent.get(&node.id),
                    "children": node.children,
                    "rawNode": node
                }
            })
        }
        None => json!({
            "content": [{"type":"text","text": format!("Node {} not found.", node_id)}],
            "structuredContent": {"found": false}
        }),
    }
}

fn summarize_profile_from_data(profile: &CpuProfile, top_n: usize) -> SummaryResult {
    let node_by_id: HashMap<u64, &CpuNode> = profile.nodes.iter().map(|n| (n.id, n)).collect();
    let total_time_us: u64 = profile.time_deltas.iter().copied().sum();

    let mut node_time: HashMap<u64, u64> = HashMap::new();
    for (node_id, dt) in profile.samples.iter().zip(profile.time_deltas.iter()) {
        *node_time.entry(*node_id).or_insert(0) += *dt;
    }

    let mut rows: Vec<HotspotRow> = node_time
        .into_iter()
        .map(|(id, self_time_us)| {
            let frame = node_by_id.get(&id).map(|n| &n.call_frame);
            let function_name = frame
                .map(|f| f.function_name.as_str())
                .filter(|name| !name.is_empty())
                .unwrap_or("(anonymous)")
                .to_string();
            let url = frame.map(|f| f.url.clone()).unwrap_or_default();
            let line_number = frame.map(|f| f.line_number + 1).unwrap_or(1);
            let column_number = frame.map(|f| f.column_number + 1).unwrap_or(1);

            HotspotRow {
                id,
                function_name,
                url,
                line_number,
                column_number,
                self_time_us,
                self_time_ms: self_time_us as f64 / 1000.0,
                self_pct: pct(self_time_us, total_time_us),
            }
        })
        .collect();

    rows.sort_by(|a, b| b.self_time_us.cmp(&a.self_time_us));
    rows.truncate(top_n);

    SummaryResult {
        total_time_ms: total_time_us as f64 / 1000.0,
        total_samples: profile.samples.len(),
        hotspots: rows,
    }
}

fn summarize_by_url(profile: &CpuProfile, top_n: usize) -> Vec<UrlRow> {
    let node_by_id: HashMap<u64, &CpuNode> = profile.nodes.iter().map(|n| (n.id, n)).collect();
    let total_time_us: u64 = profile.time_deltas.iter().copied().sum();

    let mut by_url: HashMap<String, u64> = HashMap::new();
    for (node_id, dt) in profile.samples.iter().zip(profile.time_deltas.iter()) {
        let url = node_by_id
            .get(node_id)
            .map(|n| n.call_frame.url.as_str())
            .filter(|u| !u.is_empty())
            .unwrap_or("(internal)")
            .to_string();
        *by_url.entry(url).or_insert(0) += *dt;
    }

    let mut rows: Vec<UrlRow> = by_url
        .into_iter()
        .map(|(url, self_time_us)| UrlRow {
            url,
            self_time_us,
            self_time_ms: self_time_us as f64 / 1000.0,
            self_pct: pct(self_time_us, total_time_us),
        })
        .collect();

    rows.sort_by(|a, b| b.self_time_us.cmp(&a.self_time_us));
    rows.truncate(top_n);
    rows
}

fn get_hot_paths(profile: &CpuProfile, top_n: usize) -> Vec<HotPathRow> {
    let node_by_id: HashMap<u64, &CpuNode> = profile.nodes.iter().map(|n| (n.id, n)).collect();
    let parent = build_parent_map(profile);
    let total_time_us: u64 = profile.time_deltas.iter().copied().sum();

    let mut by_path: HashMap<String, (Vec<String>, u64)> = HashMap::new();
    for (node_id, dt) in profile.samples.iter().zip(profile.time_deltas.iter()) {
        let mut chain: Vec<String> = Vec::new();
        let mut current = Some(*node_id);
        while let Some(cid) = current {
            if let Some(node) = node_by_id.get(&cid) {
                chain.push(display_name(&node.call_frame));
            } else {
                chain.push(format!("unknown#{cid}"));
                break;
            }
            current = parent.get(&cid).copied();
        }
        chain.reverse();
        let key = chain.join(" -> ");
        let entry = by_path.entry(key).or_insert((chain, 0));
        entry.1 += *dt;
    }

    let mut rows: Vec<HotPathRow> = by_path
        .into_values()
        .map(|(path, time_us)| HotPathRow {
            path,
            time_us,
            time_ms: time_us as f64 / 1000.0,
            pct: pct(time_us, total_time_us),
        })
        .collect();

    rows.sort_by(|a, b| b.time_us.cmp(&a.time_us));
    rows.truncate(top_n);
    rows
}

fn get_bottom_up(profile: &CpuProfile, top_n: usize) -> Vec<BottomUpRow> {
    let node_by_id: HashMap<u64, &CpuNode> = profile.nodes.iter().map(|n| (n.id, n)).collect();
    let parent = build_parent_map(profile);

    let mut self_time: HashMap<String, u64> = HashMap::new();
    let mut total_time: HashMap<String, u64> = HashMap::new();
    let mut callers: HashMap<String, HashMap<String, u64>> = HashMap::new();

    for (leaf_id, dt) in profile.samples.iter().zip(profile.time_deltas.iter()) {
        let mut current = Some(*leaf_id);
        let mut prev_func: Option<String> = None;
        while let Some(cid) = current {
            let Some(node) = node_by_id.get(&cid) else { break };
            let func = display_name(&node.call_frame);
            *total_time.entry(func.clone()).or_insert(0) += *dt;

            if prev_func.is_none() {
                *self_time.entry(func.clone()).or_insert(0) += *dt;
            } else if let Some(callee) = &prev_func {
                let map = callers.entry(callee.clone()).or_default();
                *map.entry(func.clone()).or_insert(0) += *dt;
            }

            prev_func = Some(func);
            current = parent.get(&cid).copied();
        }
    }

    let mut rows: Vec<BottomUpRow> = total_time
        .into_iter()
        .map(|(function, total_us)| {
            let mut caller_rows: Vec<BottomUpCaller> = callers
                .get(&function)
                .map(|m| {
                    m.iter()
                        .map(|(c, t)| BottomUpCaller {
                            caller: c.clone(),
                            time_us: *t,
                        })
                        .collect()
                })
                .unwrap_or_default();
            caller_rows.sort_by(|a, b| b.time_us.cmp(&a.time_us));
            caller_rows.truncate(3);

            let self_us = *self_time.get(&function).unwrap_or(&0);
            BottomUpRow {
                function,
                self_time_us: self_us,
                self_time_ms: self_us as f64 / 1000.0,
                total_time_us: total_us,
                total_time_ms: total_us as f64 / 1000.0,
                main_callers: caller_rows,
            }
        })
        .collect();

    rows.sort_by(|a, b| b.total_time_us.cmp(&a.total_time_us));
    rows.truncate(top_n);
    rows
}

fn diff_profiles(before: &CpuProfile, after: &CpuProfile, top_n: usize) -> Value {
    let b = summarize_profile_from_data(before, usize::MAX);
    let a = summarize_profile_from_data(after, usize::MAX);

    let mut b_map: HashMap<String, u64> = HashMap::new();
    let mut a_map: HashMap<String, u64> = HashMap::new();

    for r in b.hotspots {
        b_map.insert(func_key(&r), r.self_time_us);
    }
    for r in a.hotspots {
        a_map.insert(func_key(&r), r.self_time_us);
    }

    let mut keys: Vec<String> = b_map.keys().cloned().collect();
    for k in a_map.keys() {
        if !b_map.contains_key(k) {
            keys.push(k.clone());
        }
    }

    let mut deltas = Vec::new();
    for k in keys {
        let before_us = *b_map.get(&k).unwrap_or(&0);
        let after_us = *a_map.get(&k).unwrap_or(&0);
        let delta = after_us as i64 - before_us as i64;
        deltas.push(json!({
            "function": k,
            "beforeUs": before_us,
            "afterUs": after_us,
            "deltaUs": delta
        }));
    }

    deltas.sort_by(|x, y| {
        y["deltaUs"]
            .as_i64()
            .unwrap_or(0)
            .cmp(&x["deltaUs"].as_i64().unwrap_or(0))
    });

    let regressions: Vec<Value> = deltas.iter().filter(|d| d["deltaUs"].as_i64().unwrap_or(0) > 0).take(top_n).cloned().collect();
    let mut improvements: Vec<Value> = deltas.iter().filter(|d| d["deltaUs"].as_i64().unwrap_or(0) < 0).cloned().collect();
    improvements.sort_by(|x, y| x["deltaUs"].as_i64().unwrap_or(0).cmp(&y["deltaUs"].as_i64().unwrap_or(0)));
    improvements.truncate(top_n);

    json!({
        "beforeTotalMs": b.total_time_ms,
        "afterTotalMs": a.total_time_ms,
        "totalDeltaMs": a.total_time_ms - b.total_time_ms,
        "regressions": regressions,
        "improvements": improvements
    })
}

fn diagnose_profile(profile: &CpuProfile, top_n: usize) -> Value {
    let summary = summarize_profile_from_data(profile, top_n);
    let by_url = summarize_by_url(profile, top_n);
    let hot_paths = get_hot_paths(profile, top_n.min(5));
    let bottom_up = get_bottom_up(profile, top_n.min(5));

    let bottlenecks: Vec<String> = summary
        .hotspots
        .iter()
        .map(|h| format!("{} ({:.2}% self)", h.function_name, h.self_pct))
        .collect();

    let mut advice = Vec::new();
    if let Some(top) = by_url.first() {
        advice.push(format!("优先检查文件 {}，其 self time 占比 {:.2}%", top.url, top.self_pct));
    }
    if let Some(path) = hot_paths.first() {
        advice.push(format!("优先优化最热调用链：{}", path.path.join(" -> ")));
    }
    if let Some(bu) = bottom_up.first() {
        advice.push(format!("关注函数 {}，其 total time {:.3}ms", bu.function, bu.total_time_ms));
    }

    let report = format!(
        "总体采样时间 {:.3}ms，共 {} 个样本。\n主要瓶颈：{}\n建议：{}",
        summary.total_time_ms,
        summary.total_samples,
        if bottlenecks.is_empty() { "无".to_string() } else { bottlenecks.join("; ") },
        if advice.is_empty() { "无".to_string() } else { advice.join("; ") }
    );

    json!({
        "summary": summary,
        "bottlenecks": bottlenecks,
        "hotPaths": hot_paths,
        "bottomUp": bottom_up,
        "suggestions": advice,
        "report": report
    })
}

fn func_key(r: &HotspotRow) -> String {
    format!("{} @ {}:{}:{}", r.function_name, r.url, r.line_number, r.column_number)
}

fn pct(part: u64, total: u64) -> f64 {
    if total == 0 { 0.0 } else { part as f64 * 100.0 / total as f64 }
}

fn summary_to_markdown(summary: &SummaryResult) -> String {
    let mut lines = vec![
        format!("Total sampled time: {:.3} ms", summary.total_time_ms),
        format!("Total samples: {}", summary.total_samples),
        "".to_string(),
        format!("Top {} hotspots by self time:", summary.hotspots.len()),
        "".to_string(),
        "| # | Function | File | Line:Col | Self Time (ms) | Self % |".to_string(),
        "|---:|---|---|---:|---:|---:|".to_string(),
    ];

    for (idx, row) in summary.hotspots.iter().enumerate() {
        let file = if row.url.is_empty() {
            "(internal)".to_string()
        } else {
            Path::new(&row.url)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| row.url.clone())
        };
        lines.push(format!(
            "| {} | {} | {} | {}:{} | {:.3} | {:.2}% |",
            idx + 1,
            row.function_name,
            file,
            row.line_number,
            row.column_number,
            row.self_time_ms,
            row.self_pct
        ));
    }

    lines.join("\n")
}

fn by_url_to_markdown(rows: &[UrlRow], total_time_ms: f64, total_samples: usize) -> String {
    let mut lines = vec![
        format!("Total sampled time: {:.3} ms", total_time_ms),
        format!("Total samples: {}", total_samples),
        "".to_string(),
        format!("Top {} URLs by self time:", rows.len()),
        "".to_string(),
        "| # | URL | Self Time (ms) | Self % |".to_string(),
        "|---:|---|---:|---:|".to_string(),
    ];
    for (i, r) in rows.iter().enumerate() {
        lines.push(format!("| {} | {} | {:.3} | {:.2}% |", i + 1, r.url, r.self_time_ms, r.self_pct));
    }
    lines.join("\n")
}

fn hot_paths_markdown(rows: &[HotPathRow]) -> String {
    let mut lines = vec![
        format!("Top {} hot paths:", rows.len()),
        "| # | Path | Time (ms) | Pct |".to_string(),
        "|---:|---|---:|---:|".to_string(),
    ];
    for (i, r) in rows.iter().enumerate() {
        lines.push(format!("| {} | {} | {:.3} | {:.2}% |", i + 1, r.path.join(" -> "), r.time_ms, r.pct));
    }
    lines.join("\n")
}

fn bottom_up_markdown(rows: &[BottomUpRow]) -> String {
    let mut lines = vec![
        format!("Top {} bottom-up functions:", rows.len()),
        "| # | Function | Self (ms) | Total (ms) | Main Callers |".to_string(),
        "|---:|---|---:|---:|---|".to_string(),
    ];
    for (i, r) in rows.iter().enumerate() {
        let callers = if r.main_callers.is_empty() {
            "-".to_string()
        } else {
            r.main_callers
                .iter()
                .map(|c| format!("{} ({}us)", c.caller, c.time_us))
                .collect::<Vec<_>>()
                .join("; ")
        };
        lines.push(format!("| {} | {} | {:.3} | {:.3} | {} |", i + 1, r.function, r.self_time_ms, r.total_time_ms, callers));
    }
    lines.join("\n")
}

fn diff_markdown(v: &Value) -> String {
    let before = v["beforeTotalMs"].as_f64().unwrap_or(0.0);
    let after = v["afterTotalMs"].as_f64().unwrap_or(0.0);
    let delta = v["totalDeltaMs"].as_f64().unwrap_or(0.0);
    format!(
        "Total time: before {:.3} ms, after {:.3} ms, delta {:.3} ms\nRegressions: {}\nImprovements: {}",
        before,
        after,
        delta,
        v["regressions"].as_array().map(|a| a.len()).unwrap_or(0),
        v["improvements"].as_array().map(|a| a.len()).unwrap_or(0)
    )
}

fn read_message<R: Read>(reader: &mut R) -> io::Result<Option<Value>> {
    let mut headers = Vec::new();
    let mut one = [0u8; 1];

    loop {
        let n = reader.read(&mut one)?;
        if n == 0 {
            if headers.is_empty() {
                return Ok(None);
            }
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF while reading headers"));
        }
        headers.push(one[0]);
        if headers.ends_with(b"\r\n\r\n") {
            break;
        }
    }

    let header_text = String::from_utf8_lossy(&headers);
    let mut content_length = None;
    for line in header_text.split("\r\n") {
        if let Some((k, v)) = line.split_once(':') {
            if k.eq_ignore_ascii_case("Content-Length") {
                content_length = v.trim().parse::<usize>().ok();
            }
        }
    }

    let len = content_length
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing Content-Length header"))?;

    let mut body = vec![0u8; len];
    reader.read_exact(&mut body)?;
    let value: Value = serde_json::from_slice(&body)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    Ok(Some(value))
}

fn write_message<W: Write>(writer: &mut W, msg: &Value) -> io::Result<()> {
    let body = serde_json::to_vec(msg)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;
    writer.write_all(&body)?;
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_profile() -> CpuProfile {
        CpuProfile {
            nodes: vec![
                CpuNode {
                    id: 1,
                    call_frame: CallFrame {
                        function_name: "root".to_string(),
                        url: "/tmp/a.js".to_string(),
                        line_number: 0,
                        column_number: 0,
                    },
                    children: vec![2, 3],
                    hit_count: 0,
                },
                CpuNode {
                    id: 2,
                    call_frame: CallFrame {
                        function_name: "foo".to_string(),
                        url: "/tmp/a.js".to_string(),
                        line_number: 1,
                        column_number: 0,
                    },
                    children: vec![],
                    hit_count: 2,
                },
                CpuNode {
                    id: 3,
                    call_frame: CallFrame {
                        function_name: "bar".to_string(),
                        url: "/tmp/b.js".to_string(),
                        line_number: 2,
                        column_number: 0,
                    },
                    children: vec![],
                    hit_count: 1,
                },
            ],
            samples: vec![2, 2, 3],
            time_deltas: vec![100, 300, 600],
        }
    }

    #[test]
    fn summarize_profile_orders_by_self_time() {
        let summary = summarize_profile_from_data(&mock_profile(), 10);
        assert_eq!(summary.hotspots[0].function_name, "bar");
        assert_eq!(summary.hotspots[0].self_time_us, 600);
    }

    #[test]
    fn inspect_contains_parent_children_hitcount() {
        let out = inspect_node(&mock_profile(), 2);
        assert_eq!(out["structuredContent"]["hitCount"], 2);
        assert_eq!(out["structuredContent"]["parentId"], 1);
        assert!(out["structuredContent"]["children"].as_array().is_some());
    }

    #[test]
    fn hot_paths_and_bottom_up_work() {
        let p = mock_profile();
        let paths = get_hot_paths(&p, 5);
        let bu = get_bottom_up(&p, 5);
        assert!(!paths.is_empty());
        assert!(!bu.is_empty());
    }

    #[test]
    fn invalid_request_without_method() {
        let resp = handle_jsonrpc(json!({"jsonrpc":"2.0","id":1})).expect("response");
        assert_eq!(resp["error"]["code"], -32600);
    }
}
