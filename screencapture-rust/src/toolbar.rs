use crate::shapes::{ToolType, COLORS, STROKE_WIDTHS};
use egui::{Color32, Response, Ui};

/// 도구 모음 UI
pub struct Toolbar {
    pub current_tool: ToolType,
    pub current_color: Color32,
    pub current_stroke_width: f32,
}

impl Toolbar {
    pub fn new() -> Self {
        Self {
            current_tool: ToolType::Rectangle,
            current_color: COLORS[0],
            current_stroke_width: STROKE_WIDTHS[1],
        }
    }

    /// 도구 모음 UI 렌더링
    pub fn ui(&mut self, ui: &mut Ui) -> ToolbarAction {
        let mut action = ToolbarAction::None;

        ui.horizontal(|ui| {
            ui.label("도구:");

            // 선택 도구
            if self.tool_button(ui, "✋", ToolType::Select, "선택").clicked() {
                self.current_tool = ToolType::Select;
            }

            ui.separator();

            // 도형 도구
            if self.tool_button(ui, "□", ToolType::Rectangle, "사각형").clicked() {
                self.current_tool = ToolType::Rectangle;
            }
            if self.tool_button(ui, "■", ToolType::FilledRect, "채워진 사각형").clicked() {
                self.current_tool = ToolType::FilledRect;
            }
            if self.tool_button(ui, "○", ToolType::Ellipse, "타원").clicked() {
                self.current_tool = ToolType::Ellipse;
            }
            if self.tool_button(ui, "●", ToolType::FilledEllipse, "채워진 타원").clicked() {
                self.current_tool = ToolType::FilledEllipse;
            }

            ui.separator();

            // 선 도구
            if self.tool_button(ui, "→", ToolType::Arrow, "화살표").clicked() {
                self.current_tool = ToolType::Arrow;
            }
            if self.tool_button(ui, "—", ToolType::Line, "직선").clicked() {
                self.current_tool = ToolType::Line;
            }
            if self.tool_button(ui, "~", ToolType::Curve, "곡선").clicked() {
                self.current_tool = ToolType::Curve;
            }

            ui.separator();

            // 기타 도구
            if self.tool_button(ui, "T", ToolType::Text, "텍스트").clicked() {
                self.current_tool = ToolType::Text;
            }
            if self.tool_button(ui, "#", ToolType::Number, "번호").clicked() {
                self.current_tool = ToolType::Number;
            }
            if self.tool_button(ui, "⌧", ToolType::Mosaic, "모자이크").clicked() {
                self.current_tool = ToolType::Mosaic;
            }
            if self.tool_button(ui, "⌫", ToolType::Eraser, "지우개").clicked() {
                self.current_tool = ToolType::Eraser;
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("색상:");

            // 색상 선택
            for color in &COLORS {
                let size = egui::vec2(24.0, 24.0);
                let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

                if ui.is_rect_visible(rect) {
                    let visuals = ui.style().interact(&response);
                    let stroke = if self.current_color == *color {
                        egui::Stroke::new(2.0, Color32::WHITE)
                    } else {
                        visuals.bg_stroke
                    };

                    ui.painter().rect_filled(rect, 2.0, *color);
                    ui.painter().rect_stroke(rect, 2.0, stroke);
                }

                if response.clicked() {
                    self.current_color = *color;
                }

                response.on_hover_text(format!("RGB: {:?}", color));
            }

            ui.separator();

            ui.label("두께:");

            // 선 두께 선택
            for &width in &STROKE_WIDTHS {
                let selected = (self.current_stroke_width - width).abs() < 0.1;
                if ui.selectable_label(selected, format!("{:.0}px", width)).clicked() {
                    self.current_stroke_width = width;
                }
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            // 실행 취소/다시 실행
            if ui.button("↶ 실행 취소 (Ctrl+Z)").clicked() {
                action = ToolbarAction::Undo;
            }
            if ui.button("↷ 다시 실행 (Ctrl+Y)").clicked() {
                action = ToolbarAction::Redo;
            }

            ui.separator();

            // 저장/복사
            if ui.button("💾 저장").clicked() {
                action = ToolbarAction::Save;
            }
            if ui.button("📋 클립보드에 복사").clicked() {
                action = ToolbarAction::CopyToClipboard;
            }

            ui.separator();

            // 색상 정보 복사
            if ui.button("RGB 복사 (Ctrl+R)").clicked() {
                action = ToolbarAction::CopyColorRGB;
            }
            if ui.button("HEX 복사 (Ctrl+H)").clicked() {
                action = ToolbarAction::CopyColorHEX;
            }

            ui.separator();

            // 닫기
            if ui.button("❌ 닫기 (ESC)").clicked() {
                action = ToolbarAction::Close;
            }
        });

        action
    }

    /// 도구 버튼 생성
    fn tool_button(
        &self,
        ui: &mut Ui,
        icon: &str,
        tool: ToolType,
        tooltip: &str,
    ) -> Response {
        let selected = self.current_tool == tool;
        let button = ui.selectable_label(selected, icon);
        button.on_hover_text(tooltip)
    }
}

impl Default for Toolbar {
    fn default() -> Self {
        Self::new()
    }
}

/// 도구 모음에서 발생한 액션
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolbarAction {
    None,
    Undo,
    Redo,
    Save,
    CopyToClipboard,
    CopyColorRGB,
    CopyColorHEX,
    Close,
}
