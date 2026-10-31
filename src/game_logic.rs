use crate::app::NgalApp;
use crate::parser::{CommandAction, StoryParser};
use crate::save::{DialogueState, EngineState, ImageParams, SaveData};
use crate::texture::load_texture;
use eframe::egui;

impl NgalApp {
    pub fn start_game(&mut self, ctx: &egui::Context) {
        let script_path = self.game_config.get_index_path();
        self.current_script_file = script_path.clone();
        self.current_label = "index".to_string();
        self.parser = StoryParser::load_from_file(&script_path);
        
        self.line_index = 0;
        self.history.clear();
        self.parser.variables.clear();
        self.portrait = None;
        self.current_portrait_path = None;
        self.current_bgm = None;

        let default_bg = self.game_config.get_background_path();
        self.background = self.load_cached_texture(ctx, &default_bg);
        self.current_bg_path = Some(default_bg);

        self.execute_node(ctx);
        self.screen = crate::app::Screen::Game;
        self.last_screen = crate::app::Screen::Game;
    }

    pub fn execute_node(&mut self, ctx: &egui::Context) {
        while self.line_index < self.parser.nodes.len() {
            let node = self.parser.nodes[self.line_index].clone();

            // 1. 背景处理
            if let Some(bg) = &node.bg_path {
                if bg.is_empty() {
                    self.background = None;
                    self.current_bg_path = None;
                } else if self.current_bg_path.as_deref() != Some(bg) {
                    self.background = self.load_cached_texture(ctx, bg);
                    self.current_bg_path = Some(bg.clone());
                }
            } 

            // 2. 立绘处理
            if let Some(port) = &node.portrait_path {
                if port.is_empty() {
                    self.portrait = None;
                    self.current_portrait_path = None;
                } else if self.current_portrait_path.as_deref() != Some(port) {
                    self.portrait = self.load_cached_texture(ctx, port);
                    self.current_portrait_path = Some(port.clone());
                }
                self.portrait_align = node.portrait_pos;
                self.portrait_scale = node.portrait_scale;
            }

            // 处理指令
            if let Some(cmd) = &node.command_action {
                match cmd {
                    CommandAction::Load { file, target_label } => {
                        if let Some(fpath) = file {
                            self.current_script_file = fpath.clone();
                            self.parser = StoryParser::load_from_file(fpath);
                            self.current_bg_path = None;
                            self.current_portrait_path = None;
                        }
                        if !target_label.is_empty() {
                            self.current_label = target_label.clone();
                            if let Some(&idx) = self.parser.label_map.get(target_label) {
                                self.line_index = idx;
                                continue;
                            }
                        }
                    }
                    CommandAction::SetVar { expr } => {
                        self.eval_expression(expr);
                    }
                    CommandAction::IfJump { condition, target_label } => {
                        if self.eval_condition(condition) {
                            if !target_label.is_empty() {
                                self.current_label = target_label.clone();
                                if let Some(&idx) = self.parser.label_map.get(target_label) {
                                    self.line_index = idx;
                                    continue;
                                }
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
                        self.screen = crate::app::Screen::Menu;
                        self.last_screen = crate::app::Screen::Menu;
                        let default_bg_path = self.game_config.get_background_path();
                        self.background = self.load_cached_texture(ctx, &default_bg_path);
                        self.current_bg_path = Some(default_bg_path);
                        self.portrait = None;
                        self.current_portrait_path = None;
                        self.current_bgm = None;
                        self.history.clear();
                        self.parser.variables.clear();
                        self.line_index = 0;
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

    /// 按需懒加载纹理器
    pub fn load_cached_texture(&mut self, ctx: &egui::Context, path: &str) -> Option<egui::TextureHandle> {
        if path.is_empty() {
            return None;
        }
        if let Some(tex) = self.texture_cache.get(path) {
            return Some(tex.clone());
        }
        
        if self.texture_cache.len() >= 15 {
            let current_bg = self.current_bg_path.clone();
            let current_port = self.current_portrait_path.clone();
            self.texture_cache.retain(|k, _| {
                Some(k.clone()) == current_bg || Some(k.clone()) == current_port
            });
        }

        if let Some(tex) = load_texture(ctx, path) {
            self.texture_cache.insert(path.to_string(), tex.clone());
            return Some(tex);
        }
        None
    }

    pub fn resolve_text_macros(&self, input: &str) -> String {
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

        if self.waiting_input {
            return;
        }
        if self.line_index < self.parser.nodes.len() && !self.parser.nodes[self.line_index].choices.is_empty() {
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
        self.current_label = label.to_string();
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
        let image_params = self.current_portrait_path.as_ref().map(|filename| ImageParams {
            filename: filename.clone(),
            position: self.portrait_align,
            scale: self.portrait_scale * 100.0,
        });

        SaveData {
            state: EngineState::InDialogue(DialogueState {
                scene_id: self.current_label.clone(),
                cmd_index: self.line_index,
            }),
            menu_selected: 0,
            variables: self.parser.variables.clone(),
            current_image: self.current_portrait_path.clone(),
            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            current_file: self.current_script_file.clone(),
            background: self.current_bg_path.clone(),
            bgm: self.current_bgm.clone(),
            image_params,
            preview_text: self.typewriter.visible_text(),
            background_path: self.current_bg_path.clone(),
        }
    }

    pub fn load_save_data(&mut self, data: SaveData, ctx: &egui::Context) {
        self.current_script_file = data.current_file;
        self.parser = StoryParser::load_from_file(&self.current_script_file);

        match data.state {
            EngineState::InDialogue(diag) => {
                self.current_label = diag.scene_id;
                self.line_index = diag.cmd_index;
            }
        }

        self.parser.variables = data.variables;
        self.current_bgm = data.bgm;

        if let Some(bg) = &data.background {
            self.background = self.load_cached_texture(ctx, bg);
            self.current_bg_path = Some(bg.clone());
        } else {
            self.background = None;
            self.current_bg_path = None;
        }

        if let Some(port) = &data.current_image {
            self.portrait = self.load_cached_texture(ctx, port);
            self.current_portrait_path = Some(port.clone());
            if let Some(params) = data.image_params {
                self.portrait_align = params.position;
                self.portrait_scale = params.scale / 100.0;
            }
        } else {
            self.portrait = None;
            self.current_portrait_path = None;
        }

        self.execute_node(ctx);
        self.screen = crate::app::Screen::Game;
        self.last_screen = crate::app::Screen::Game;
    }
}
