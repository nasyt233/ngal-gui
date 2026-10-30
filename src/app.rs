use crate::config::{Config, GameConfig, GAME_CONFIG_PATH};
use crate::fonts::{apply_text_styles, load_system_cjk_font};
use crate::parser::{CommandAction, StoryParser};
use crate::save::SaveData;
use crate::texture::load_texture;
use crate::typewriter::Typewriter;
use eframe::egui::{self, Pos2};
use std::path::Path;

#[derive(PartialEq, Clone, Copy)]
pub enum Screen {
    Menu,
    Settings,
    Game,
    Save,
    Load,
}

pub struct NgalApp {
    pub(crate) screen: Screen,
    pub last_screen: Screen,
    pub(crate) config: Config,
    pub(crate) game_config: GameConfig,
    pub(crate) config_path: String,
    pub(crate) background: Option<egui::TextureHandle>,
    pub(crate) portrait: Option<egui::TextureHandle>,
    pub(crate) current_bg_path: Option<String>,
    #[allow(dead_code)]
    pub(crate) portrait_pos: Pos2,
    pub(crate) portrait_scale: f32,
    pub(crate) portrait_align: u8,
    pub(crate) hide_ui: bool,
    pub(crate) speaker: String,
    pub(crate) typewriter: Typewriter,
    
    pub(crate) parser: StoryParser,
    pub(crate) line_index: usize,
    pub(crate) history: Vec<(String, String)>,
    
    pub(crate) auto_play: bool,
    pub(crate) auto_timer: f32,
    pub(crate) is_skipping: bool,
    pub(crate) show_log: bool,
    pub(crate) waiting_input: bool,
    pub(crate) input_prompt: String,
    pub(crate) input_buffer: String,
    pub(crate) input_var_name: String,
    
    pub(crate) sleep_timer: f32,
}

impl NgalApp {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config, config_path: String) -> Self {
        let ctx = &cc.egui_ctx;
    
        ensure_asset_structure();
        let game_config = GameConfig::load_or_default(GAME_CONFIG_PATH);
        load_system_cjk_font(ctx);
        apply_text_styles(ctx);
    
        let default_bg_path = game_config.get_background_path();
        let script_path = game_config.get_index_path();
        
        let parser = StoryParser::load_from_file(&script_path);
        let default_bg = load_texture(ctx, &default_bg_path);
    
        let speed = config.text_speed;
        let mut app = Self {
            screen: Screen::Menu,
            last_screen: Screen::Menu,
            config,
            game_config,
            config_path,
            background: default_bg,
            current_bg_path: Some(default_bg_path),
            portrait: None,
            portrait_pos: Pos2::new(0.5, 0.85),
            portrait_scale: 0.5,
            portrait_align: 2,
            hide_ui: false,
            speaker: "".to_owned(),
            typewriter: Typewriter::new("", speed),
            parser,
            line_index: 0,
            history: vec![],
            auto_play: false,
            auto_timer: 0.0,
            is_skipping: false,
            show_log: false,
            waiting_input: false,
            input_buffer: String::new(),
            input_var_name: String::new(),
            input_prompt: String::new(),
            sleep_timer: 0.0,
        };
    
        if !app.parser.nodes.is_empty() {
            app.execute_node(ctx);
        }
    
        app
    }

    pub fn start_game(&mut self, ctx: &egui::Context) {
        let script_path = self.game_config.get_index_path();
        self.parser = StoryParser::load_from_file(&script_path);
        self.line_index = 0;
        self.history.clear();
        self.parser.variables.clear();
        self.execute_node(ctx);
        self.screen = Screen::Game;
        self.last_screen = Screen::Game; // 记录来源
    }

    pub fn execute_node(&mut self, ctx: &egui::Context) {
        while self.line_index < self.parser.nodes.len() {
            let node = self.parser.nodes[self.line_index].clone();

            // 背景
            if let Some(bg) = &node.bg_path {
                self.background = load_texture(ctx, bg);
                self.current_bg_path = Some(bg.clone());
            } else if node.bg_path.is_none() && node.speaker.is_none() && node.text.is_none() {
                self.background = None;
                self.current_bg_path = None;
            }

            // 处理立绘
            if let Some(port) = &node.portrait_path {
                self.portrait = load_texture(ctx, port);
                self.portrait_align = node.portrait_pos; // 1, 2, 3
                self.portrait_scale = node.portrait_scale;
            } else {
                self.portrait = None;
            }

            // 处理指令
            if let Some(cmd) = &node.command_action {
                match cmd {
                    CommandAction::Load { file, target_label } => {
                        if let Some(fpath) = file {
                            self.parser = StoryParser::load_from_file(fpath);
                        }
                        if let Some(&idx) = self.parser.label_map.get(target_label) {
                            self.line_index = idx;
                            continue;
                        }
                    }
                    CommandAction::SetVar { expr } => {
                        self.eval_expression(expr);
                    }
                    CommandAction::IfJump { condition, target_label } => {
                        if self.eval_condition(condition) {
                            if let Some(&idx) = self.parser.label_map.get(target_label) {
                                self.line_index = idx;
                                continue;
                            }
                        }
                    }
                    CommandAction::Sleep(secs) => {
                        self.sleep_timer = *secs;
                        self.speaker = "系统".to_string();
                        self.typewriter.set_text(&format!("（等待中... {:.1}s）", secs));
                        return;
                    }
                    CommandAction::Input { prompt, var_name } => {
                        self.speaker = "系统".to_string();
                        let resolved_prompt = self.resolve_text_macros(prompt);
                        self.typewriter.set_text(&resolved_prompt);
                        self.waiting_input = true;
                        self.input_var_name = var_name.clone();
                        self.input_buffer.clear();
                        self.input_prompt = resolved_prompt; 
                        return;
                    }
                    CommandAction::End => {
                        self.screen = Screen::Menu;
                        self.last_screen = Screen::Menu;
                        let default_bg_path = self.game_config.get_background_path();
                        self.background = load_texture(ctx, &default_bg_path);
                        self.current_bg_path = Some(default_bg_path);
                        return;
                    }

                }
            }

            // 文本展示
            if node.text.is_some() || !node.choices.is_empty() {
                let speaker = node.speaker.clone().unwrap_or_else(|| "".to_string());
                let raw_text = node.text.clone().unwrap_or_default();
                let resolved_text = self.resolve_text_macros(&raw_text);

                let resolved_speaker = self.resolve_text_macros(&speaker);

                self.speaker = resolved_speaker;
                self.typewriter.set_text(&resolved_text);
                if !resolved_text.is_empty() {
                    self.history.push((self.speaker.clone(), resolved_text));
                }
                break;
            }

            self.line_index += 1;
        }
    }

    /// 解析文本中的宏：{变量} 以及 $(Shell命令) 以及 \n 换行
    fn resolve_text_macros(&self, input: &str) -> String {
        let mut result = input.to_string();

        for (k, v) in &self.parser.variables {
            result = result.replace(&format!("{{{}}}", k), v);
        }

        while let Some(start) = result.find("$(") {
            if let Some(end) = result[start..].find(')') {
                let end_idx = start + end;
                let cmd = &result[start + 2..end_idx];
                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                    .unwrap_or_else(|_| "[执行失败]".to_string());
                result.replace_range(start..=end_idx, &output);
            } else {
                break;
            }
        }

        result.replace("\\n", "\n")
    }

    pub fn advance(&mut self, ctx: &egui::Context) {
        if self.sleep_timer > 0.0 {
            return;
        }
        if !self.typewriter.is_done() {
            self.typewriter.finish();
            return;
        }
        self.line_index += 1;
        self.execute_node(ctx);
    }

    pub fn jump_to_label(&mut self, label: &str, ctx: &egui::Context) {
        if let Some(&idx) = self.parser.label_map.get(label) {
            self.line_index = idx;
            self.execute_node(ctx);
        }
    }

    fn eval_expression(&mut self, expr: &str) {
        let parts: Vec<&str> = expr.split('=').map(|s| s.trim()).collect();
        if parts.len() != 2 { return; }
        let var = parts[0];
        let val_expr = parts[1];

        if val_expr.contains('+') {
            let ops: Vec<&str> = val_expr.split('+').map(|s| s.trim()).collect();
            let left = self.get_val_num(ops[0]);
            let right = self.get_val_num(ops[1]);
            self.parser.variables.insert(var.to_string(), (left + right).to_string());
        } else if val_expr.contains('-') {
            let ops: Vec<&str> = val_expr.split('-').map(|s| s.trim()).collect();
            let left = self.get_val_num(ops[0]);
            let right = self.get_val_num(ops[1]);
            self.parser.variables.insert(var.to_string(), (left - right).to_string());
        } else if val_expr.contains('*') {
            let ops: Vec<&str> = val_expr.split('*').map(|s| s.trim()).collect();
            let left = self.get_val_num(ops[0]);
            let right = self.get_val_num(ops[1]);
            self.parser.variables.insert(var.to_string(), (left * right).to_string());
        } else if val_expr.contains('/') {
            let ops: Vec<&str> = val_expr.split('/').map(|s| s.trim()).collect();
            let left = self.get_val_num(ops[0]);
            let right = self.get_val_num(ops[1]);
            let res = if right != 0 { left / right } else { 0 };
            self.parser.variables.insert(var.to_string(), res.to_string());
        } else {
            let resolved = self.resolve_val(val_expr);
            self.parser.variables.insert(var.to_string(), resolved);
        }
    }

    fn get_val_num(&self, s: &str) -> i32 {
        if let Ok(n) = s.parse::<i32>() {
            n
        } else {
            self.parser.variables.get(s).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0)
        }
    }

    fn resolve_val(&self, s: &str) -> String {
        self.parser.variables.get(s).cloned().unwrap_or_else(|| s.to_string())
    }

    fn eval_condition(&self, cond: &str) -> bool {
        if cond.contains(">=") {
            let parts: Vec<&str> = cond.split(">=").map(|s| s.trim()).collect();
            if parts.len() == 2 {
                return self.get_val_num(parts[0]) >= self.get_val_num(parts[1]);
            }
        } else if cond.contains("<=") {
            let parts: Vec<&str> = cond.split("<=").map(|s| s.trim()).collect();
            if parts.len() == 2 {
                return self.get_val_num(parts[0]) <= self.get_val_num(parts[1]);
            }
        } else if cond.contains("==") {
            let parts: Vec<&str> = cond.split("==").map(|s| s.trim()).collect();
            if parts.len() == 2 {
                return self.get_val_num(parts[0]) == self.get_val_num(parts[1]);
            }
        }
        false
    }

    pub fn create_save_data(&self) -> SaveData {
        SaveData {
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            speaker: self.speaker.clone(),
            preview_text: self.typewriter.visible_text(),
            line_index: self.line_index,
            background_path: self.current_bg_path.clone(), // 已修复：正确保存背景缩略图路径
            portrait_path: None,
        }
    }

    pub fn load_save_data(&mut self, data: SaveData, ctx: &egui::Context) {
        self.line_index = data.line_index;
        if let Some(bg) = &data.background_path {
            self.background = load_texture(ctx, bg);
            self.current_bg_path = Some(bg.clone());
        } else {
            self.background = None;
            self.current_bg_path = None;
        }
        self.execute_node(ctx);
        self.screen = Screen::Game;
        self.last_screen = Screen::Game;
    }
}

fn ensure_asset_structure() {
    for d in [
        "assets/dialog",
        "assets/music",
        "assets/portraits",
        "assets/voices",
        "save",
    ] {
        let _ = std::fs::create_dir_all(d);
    }

    let game_json_path = GAME_CONFIG_PATH;
    if !Path::new(game_json_path).exists() {
        let default_game_config = r#"{
  "title": "Ngal 视觉小说引擎",
  "logo": "logo.png",
  "background": "bg.png",
  "bgm": "bg.mp3",
  "index": "dialogue.ng",
  "menu_layout": 1
}"#;
        let _ = std::fs::write(game_json_path, default_game_config);
    }

    let dialogue_ng_path = "assets/dialog/dialogue.ng";
    if !Path::new(dialogue_ng_path).exists() || std::fs::metadata(dialogue_ng_path).map(|m| m.len() == 0).unwrap_or(true) {
        let default_story = r#"# ngal v1.0.4示例教学默认剧情
[welcome]
第一章
load:index

[index]
name = 嘉豪
bg:bg.png
music:bg.mp3
img:logo.png:2:50%
系统:欢迎来到 ngal 引擎！
系统:按回车键，等待2秒
sleep:2
系统:2秒等待结束。
系统:你好用户:hello.mp3
系统:当前时间 $(date)
系统:当前用户 $(whoami)\n当前目录 $(pwd)
img:logo2.png
img:
系统:当前默认名字：{name}
input:请输入你的名字:name
{name}：我的名字是：{name}！
变量运算
a = 13
系统:a的值为{a}
b = 78
系统:b的值为{b}
c = a + b
系统:相加计算结果为 {c}
系统:下面是选项选择
score = 10
系统:当前分数为{score}
choose:接受冒险(分数+8):accept|拒绝冒险(分数-5):refuse

[accept]
系统:你接受了冒险！
score = score + 8
系统:当前分数 {score}
load:jx

[refuse]
系统:你拒绝了冒险！
score = score - 5
系统:当前分数 {score}
load:jx

[jx]
system:if判断功能
if score >= 10: good_end
load:bad_end

[good_end]
系统:当前分数大于或等于10
系统:🤓 完美结局！得分 {score}
load:exit

[bad_end]
系统:分数小于10
系统:😭 遗憾结局。得分 {score}
load:exit

[exit]
系统:游戏结束
bg:
music:
end
"#;
        let _ = std::fs::write(dialogue_ng_path, default_story);
    }
}

impl eframe::App for NgalApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.screen {
            Screen::Menu => self.draw_menu(ctx),
            Screen::Settings => self.draw_settings(ctx),
            Screen::Game => self.draw_game(ctx),
            Screen::Save => self.draw_saveload(ctx, true),
            Screen::Load => self.draw_saveload(ctx, false),
        }
    }
}
