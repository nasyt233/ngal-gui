use std::collections::HashMap;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ScriptNode {
    pub label: String,
    pub speaker: Option<String>,
    pub text: Option<String>,
    pub bg_path: Option<String>,
    pub portrait_path: Option<String>,
    pub portrait_pos: u8,   // 1: 左, 2: 中, 3: 右
    pub portrait_scale: f32, // 缩放比例，默认 1.0
    pub music_path: Option<String>,
    pub voice_path: Option<String>,
    pub choices: Vec<(String, String)>, 
    pub command_action: Option<CommandAction>,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum CommandAction {
    Sleep(f32),
    Input { prompt: String, var_name: String },
    SetVar { expr: String },
    IfJump { condition: String, target_label: String },
    Load { file: Option<String>, target_label: String },
    End,
}

pub struct StoryParser {
    pub nodes: Vec<ScriptNode>,
    pub label_map: HashMap<String, usize>,
    pub variables: HashMap<String, String>,
}

impl StoryParser {
    /// 从文件加载并解析，支持跨文件读取
    pub fn load_from_file(file_path: &str) -> Self {
        let content = std::fs::read_to_string(file_path).unwrap_or_else(|_| {
            eprintln!("[警告] 无法读取剧本文件: {}, 使用默认空内容", file_path);
            String::new()
        });
        Self::parse_content(&content)
    }

    pub fn parse_content(content: &str) -> Self {
        let mut nodes = Vec::new();
        let mut label_map = HashMap::new();
        let mut current_label = "welcome".to_string();
        
        label_map.insert(current_label.clone(), 0);
        
        let mut active_bg = None;
        let mut active_portrait = None;
        let mut active_portrait_pos = 2; // 默认居中
        let mut active_portrait_scale = 1.0;
        let mut active_music = None;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let clean_line = if let Some(idx) = line.find('#') {
                &line[..idx]
            } else {
                line
            }.trim();

            if clean_line.is_empty() {
                continue;
            }

            // 1. 识别标签 [label_name]
            if clean_line.starts_with('[') && clean_line.ends_with(']') {
                current_label = clean_line[1..clean_line.len() - 1].trim().to_string();
                label_map.entry(current_label.clone()).or_insert(nodes.len());
                continue;
            }

            label_map.entry(current_label.clone()).or_insert(nodes.len());

            // 2. 指令解析
            if clean_line.starts_with("bg:") {
                let val = clean_line[3..].trim();
                active_bg = if val.is_empty() { None } else { Some(format!("assets/portraits/{}", val)) };
                nodes.push(ScriptNode {
                    label: current_label.clone(),
                    speaker: None, text: None,
                    bg_path: active_bg.clone(),
                    portrait_path: active_portrait.clone(),
                    portrait_pos: active_portrait_pos,
                    portrait_scale: active_portrait_scale,
                    music_path: active_music.clone(),
                    voice_path: None,
                    choices: vec![], command_action: None,
                });
            } else if clean_line.starts_with("music:") {
                let val = clean_line[6..].trim();
                active_music = if val.is_empty() { None } else { Some(format!("assets/music/{}", val)) };
                nodes.push(ScriptNode {
                    label: current_label.clone(),
                    speaker: None, text: None,
                    bg_path: active_bg.clone(),
                    portrait_path: active_portrait.clone(),
                    portrait_pos: active_portrait_pos,
                    portrait_scale: active_portrait_scale,
                    music_path: active_music.clone(),
                    voice_path: None,
                    choices: vec![], command_action: None,
                });
            } else if clean_line.starts_with("img:") {
                let val = &clean_line[4..].trim();
                if val.is_empty() {
                    active_portrait = None;
                } else {
                    let parts: Vec<&str> = val.split(':').collect();
                    active_portrait = Some(format!("assets/portraits/{}", parts[0]));
                    if parts.len() > 1 {
                        active_portrait_pos = parts[1].parse().unwrap_or(2);
                    }
                    if parts.len() > 2 {
                        let scale_str = parts[2].trim_end_matches('%');
                        if let Ok(s) = scale_str.parse::<f32>() {
                            active_portrait_scale = s / 100.0;
                        }
                    }
                }
                nodes.push(ScriptNode {
                    label: current_label.clone(),
                    speaker: None, text: None,
                    bg_path: active_bg.clone(),
                    portrait_path: active_portrait.clone(),
                    portrait_pos: active_portrait_pos,
                    portrait_scale: active_portrait_scale,
                    music_path: active_music.clone(),
                    voice_path: None,
                    choices: vec![], command_action: None,
                });
            } else if clean_line.starts_with("sleep:") {
                let secs = clean_line[6..].trim().parse().unwrap_or(1.0);
                nodes.push(ScriptNode {
                    label: current_label.clone(),
                    speaker: None, text: None,
                    bg_path: active_bg.clone(), portrait_path: active_portrait.clone(),
                    portrait_pos: active_portrait_pos, portrait_scale: active_portrait_scale,
                    music_path: active_music.clone(), voice_path: None,
                    choices: vec![], // 修复类型匹配
                    command_action: Some(CommandAction::Sleep(secs)),
                });
            } else if clean_line.starts_with("load:") {
                let target_str = clean_line[5..].trim();
                let (file, target_label) = if target_str.contains(".ng:") {
                    let parts: Vec<&str> = target_str.split(".ng:").collect();
                    (Some(format!("assets/dialog/{}.ng", parts[0])), parts[1].to_string())
                } else if target_str.ends_with(".ng") {
                    (Some(format!("assets/dialog/{}", target_str)), "welcome".to_string())
                } else {
                    (None, target_str.to_string())
                };

                nodes.push(ScriptNode {
                    label: current_label.clone(),
                    speaker: None, text: None,
                    bg_path: active_bg.clone(), portrait_path: active_portrait.clone(),
                    portrait_pos: active_portrait_pos, portrait_scale: active_portrait_scale,
                    music_path: active_music.clone(), voice_path: None,
                    choices: vec![], // 修复类型匹配
                    command_action: Some(CommandAction::Load { file, target_label }),
                });
            } else if clean_line.starts_with("choose:") {
                let parts = clean_line[7..].split('|');
                let mut choices = vec![];
                for p in parts {
                    let kv: Vec<&str> = p.split(':').collect();
                    if kv.len() == 2 {
                        choices.push((kv[0].trim().to_string(), kv[1].trim().to_string()));
                    }
                }
                nodes.push(ScriptNode {
                    label: current_label.clone(),
                    speaker: Some("系统".to_string()),
                    text: Some("请选择你的行动：".to_string()),
                    bg_path: active_bg.clone(), portrait_path: active_portrait.clone(),
                    portrait_pos: active_portrait_pos, portrait_scale: active_portrait_scale,
                    music_path: active_music.clone(), voice_path: None,
                    choices,
                    command_action: None,
                });
            } else if clean_line.starts_with("if ") {
                let cond_target: Vec<&str> = clean_line[3..].split(':').collect();
                if cond_target.len() == 2 {
                    nodes.push(ScriptNode {
                        label: current_label.clone(),
                        speaker: None, text: None,
                        bg_path: active_bg.clone(), portrait_path: active_portrait.clone(),
                        portrait_pos: active_portrait_pos, portrait_scale: active_portrait_scale,
                        music_path: active_music.clone(), voice_path: None,
                        choices: vec![],
                        command_action: Some(CommandAction::IfJump {
                            condition: cond_target[0].trim().to_string(),
                            target_label: cond_target[1].trim().to_string(),
                        }),
                    });
                }
            } else if clean_line.starts_with("input:") {
                let parts: Vec<&str> = clean_line[6..].split(':').collect();
                if parts.len() == 2 {
                    nodes.push(ScriptNode {
                        label: current_label.clone(),
                        speaker: None, text: None,
                        bg_path: active_bg.clone(), portrait_path: active_portrait.clone(),
                        portrait_pos: active_portrait_pos, portrait_scale: active_portrait_scale,
                        music_path: active_music.clone(), voice_path: None,
                        choices: vec![],
                        command_action: Some(CommandAction::Input {
                            prompt: parts[0].trim().to_string(),
                            var_name: parts[1].trim().to_string(),
                        }),
                    });
                }
            } else if clean_line == "end" {
                nodes.push(ScriptNode {
                    label: current_label.clone(),
                    speaker: None, text: None,
                    bg_path: active_bg.clone(), portrait_path: active_portrait.clone(),
                    portrait_pos: active_portrait_pos, portrait_scale: active_portrait_scale,
                    music_path: active_music.clone(), voice_path: None,
                    choices: vec![],
                    command_action: Some(CommandAction::End),
                });
            } else if clean_line.contains('=') || clean_line.contains('+') || clean_line.contains('-') || clean_line.contains('*') || clean_line.contains('/') {
                nodes.push(ScriptNode {
                    label: current_label.clone(),
                    speaker: None, text: None,
                    bg_path: active_bg.clone(), portrait_path: active_portrait.clone(),
                    portrait_pos: active_portrait_pos, portrait_scale: active_portrait_scale,
                    music_path: active_music.clone(), voice_path: None,
                    choices: vec![],
                    command_action: Some(CommandAction::SetVar { expr: clean_line.to_string() }),
                });
            } else {
                let mut speaker = None;
                let mut text = clean_line.to_string();
                let mut voice_path = None;

                if let Some(idx) = clean_line.find(':') {
                    let possible_speaker = &clean_line[..idx];
                    if !possible_speaker.contains('\\') && !possible_speaker.contains('/') {
                        speaker = Some(possible_speaker.trim().to_string());
                        let rest = &clean_line[idx + 1..];
                        
                        if let Some(v_idx) = rest.find(':') {
                            text = rest[..v_idx].trim().to_string();
                            let v_file = rest[v_idx + 1..].trim();
                            if !v_file.is_empty() {
                                voice_path = Some(format!("assets/voices/{}", v_file));
                            }
                        } else {
                            text = rest.trim().to_string();
                        }
                    }
                }

                nodes.push(ScriptNode {
                    label: current_label.clone(),
                    speaker,
                    text: Some(text),
                    bg_path: active_bg.clone(),
                    portrait_path: active_portrait.clone(),
                    portrait_pos: active_portrait_pos,
                    portrait_scale: active_portrait_scale,
                    music_path: active_music.clone(),
                    voice_path,
                    choices: vec![],
                    command_action: None,
                });
            }
        }

        Self {
            nodes,
            label_map,
            variables: HashMap::new(),
        }
    }
}
