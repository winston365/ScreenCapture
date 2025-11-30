mod canvas;
mod capture;
mod clipboard;
mod shapes;
mod toolbar;

use canvas::Canvas;
use capture::capture_screen;
use clipboard::{copy_color_hex, copy_color_rgb, copy_image_to_clipboard, image_buffer_to_color_image, save_image_to_file};
use egui::{CentralPanel, Color32, Key, Pos2, Vec2};
use shapes::ToolType;
use toolbar::{Toolbar, ToolbarAction};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("ScreenCapture Rust - 화면 캡처 및 주석 도구")
            .with_decorations(true)
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "ScreenCapture Rust",
        options,
        Box::new(|_cc| Ok(Box::new(ScreenCaptureApp::new()))),
    )
}

struct ScreenCaptureApp {
    canvas: Canvas,
    toolbar: Toolbar,
    status_message: String,
    show_capture_button: bool,
}

impl ScreenCaptureApp {
    fn new() -> Self {
        Self {
            canvas: Canvas::new(),
            toolbar: Toolbar::new(),
            status_message: "화면 캡처 버튼을 클릭하여 시작하세요".to_string(),
            show_capture_button: true,
        }
    }

    fn capture_screen(&mut self) {
        match capture_screen() {
            Ok(result) => {
                let color_image = image_buffer_to_color_image(&result.image);
                self.canvas.set_background(color_image);
                self.status_message = format!("화면 캡처 완료: {}x{}", result.width, result.height);
                self.show_capture_button = false;
            }
            Err(e) => {
                self.status_message = format!("캡처 실패: {}", e);
            }
        }
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        // Ctrl+Z: 실행 취소
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::Z)) {
            self.canvas.undo();
            self.status_message = "실행 취소".to_string();
        }

        // Ctrl+Y: 다시 실행
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::Y)) {
            self.canvas.redo();
            self.status_message = "다시 실행".to_string();
        }

        // Ctrl+S: 저장
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::S)) {
            self.save_image();
        }

        // Ctrl+C: 클립보드 복사
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::C)) {
            self.copy_to_clipboard();
        }

        // Ctrl+R: RGB 복사
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::R)) {
            if let Err(e) = copy_color_rgb(self.canvas.current_color) {
                self.status_message = format!("RGB 복사 실패: {}", e);
            } else {
                self.status_message = "RGB 색상이 클립보드에 복사되었습니다".to_string();
            }
        }

        // Ctrl+H: HEX 복사
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::H)) {
            if let Err(e) = copy_color_hex(self.canvas.current_color) {
                self.status_message = format!("HEX 복사 실패: {}", e);
            } else {
                self.status_message = "HEX 색상이 클립보드에 복사되었습니다".to_string();
            }
        }

        // Delete: 선택된 도형 삭제
        if ctx.input(|i| i.key_pressed(Key::Delete)) {
            self.canvas.delete_selected();
            self.status_message = "선택된 도형 삭제".to_string();
        }

        // ESC: 종료
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn save_image(&mut self) {
        if let Some(image) = self.canvas.render_to_image() {
            if let Some(path) = clipboard::show_save_dialog() {
                match save_image_to_file(&image, &path) {
                    Ok(_) => {
                        self.status_message = format!("저장 완료: {}", path.display());
                    }
                    Err(e) => {
                        self.status_message = format!("저장 실패: {}", e);
                    }
                }
            }
        } else {
            self.status_message = "저장할 이미지가 없습니다".to_string();
        }
    }

    fn copy_to_clipboard(&mut self) {
        if let Some(image) = self.canvas.render_to_image() {
            match copy_image_to_clipboard(&image) {
                Ok(_) => {
                    self.status_message = "클립보드에 복사되었습니다".to_string();
                }
                Err(e) => {
                    self.status_message = format!("클립보드 복사 실패: {}", e);
                }
            }
        } else {
            self.status_message = "복사할 이미지가 없습니다".to_string();
        }
    }

    fn handle_toolbar_action(&mut self, action: ToolbarAction) {
        match action {
            ToolbarAction::Undo => {
                self.canvas.undo();
                self.status_message = "실행 취소".to_string();
            }
            ToolbarAction::Redo => {
                self.canvas.redo();
                self.status_message = "다시 실행".to_string();
            }
            ToolbarAction::Save => {
                self.save_image();
            }
            ToolbarAction::CopyToClipboard => {
                self.copy_to_clipboard();
            }
            ToolbarAction::CopyColorRGB => {
                if let Err(e) = copy_color_rgb(self.canvas.current_color) {
                    self.status_message = format!("RGB 복사 실패: {}", e);
                } else {
                    self.status_message = "RGB 색상이 클립보드에 복사되었습니다".to_string();
                }
            }
            ToolbarAction::CopyColorHEX => {
                if let Err(e) = copy_color_hex(self.canvas.current_color) {
                    self.status_message = format!("HEX 복사 실패: {}", e);
                } else {
                    self.status_message = "HEX 색상이 클립보드에 복사되었습니다".to_string();
                }
            }
            ToolbarAction::Close => {
                std::process::exit(0);
            }
            ToolbarAction::None => {}
        }
    }
}

impl eframe::App for ScreenCaptureApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 키보드 입력 처리
        self.handle_keyboard(ctx);

        // 캔버스 텍스처 업데이트
        self.canvas.update_texture(ctx);

        // 상단 패널 (도구 모음)
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let action = self.toolbar.ui(ui);
            self.handle_toolbar_action(action);

            // 캡처 버튼
            if self.show_capture_button {
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("🖥️ 전체 화면 캡처").clicked() {
                        self.capture_screen();
                    }
                });
            }
        });

        // 하단 패널 (상태 표시줄)
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status_message);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("도형: {}", self.canvas.shapes.len()));
                    if self.canvas.undo_stack.len() > 0 {
                        ui.label(format!("실행 취소 가능: {}", self.canvas.undo_stack.len()));
                    }
                });
            });
        });

        // 중앙 패널 (캔버스)
        CentralPanel::default().show(ctx, |ui| {
            // 도구 상태 동기화
            self.canvas.current_tool = self.toolbar.current_tool;
            self.canvas.current_color = self.toolbar.current_color;
            self.canvas.current_stroke_width = self.toolbar.current_stroke_width;

            // 배경 이미지 렌더링
            if let Some(ref texture) = self.canvas.background_texture {
                let image_size = texture.size_vec2();
                let available_size = ui.available_size();

                // 화면에 맞게 스케일 조정
                let scale = (available_size.x / image_size.x)
                    .min(available_size.y / image_size.y)
                    .min(1.0);

                let scaled_size = image_size * scale;
                let (rect, response) = ui.allocate_exact_size(scaled_size, egui::Sense::click_and_drag());

                // 배경 이미지 그리기
                ui.painter().image(
                    texture.id(),
                    rect,
                    egui::Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE,
                );

                // 그려진 도형들 렌더링
                for shape in &self.canvas.shapes {
                    for egui_shape in shape.to_egui_shape() {
                        ui.painter().add(egui_shape);
                    }
                }

                // 현재 그리는 중인 도형 렌더링
                if let Some(ref current_shape) = self.canvas.current_shape {
                    for egui_shape in current_shape.to_egui_shape() {
                        ui.painter().add(egui_shape);
                    }
                }

                // 마우스 입력 처리
                if response.clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        self.canvas.on_mouse_press(pos);
                    }
                }

                if response.dragged() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        self.canvas.on_mouse_drag(pos);
                    }
                }

                if response.drag_stopped() {
                    self.canvas.on_mouse_release();
                }

                // 텍스트 입력 처리
                if self.canvas.text_input_pos.is_some() {
                    ui.horizontal(|ui| {
                        ui.label("텍스트:");
                        let text_edit = ui.text_edit_singleline(&mut self.canvas.text_input);
                        if text_edit.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
                            self.canvas.finish_text_input();
                        }
                    });
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.heading("화면 캡처를 시작하려면 상단의 '전체 화면 캡처' 버튼을 클릭하세요");
                });
            }
        });
    }
}
