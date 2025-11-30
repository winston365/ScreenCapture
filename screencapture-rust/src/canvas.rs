use crate::shapes::{DrawShape, ToolType};
use egui::{Color32, ColorImage, Pos2, Rect, Stroke, TextureHandle};

/// 캔버스 상태
pub struct Canvas {
    /// 배경 이미지 (캡처된 화면)
    pub background_image: Option<ColorImage>,
    pub background_texture: Option<TextureHandle>,

    /// 그려진 도형들
    pub shapes: Vec<DrawShape>,

    /// 실행 취소/다시 실행 스택
    pub undo_stack: Vec<Vec<DrawShape>>,
    pub redo_stack: Vec<Vec<DrawShape>>,

    /// 현재 그리기 중인 도형
    pub current_shape: Option<DrawShape>,

    /// 현재 선택된 도구
    pub current_tool: ToolType,

    /// 현재 색상 및 선 두께
    pub current_color: Color32,
    pub current_stroke_width: f32,

    /// 번호 카운터
    pub number_counter: u32,

    /// 텍스트 입력 상태
    pub text_input: String,
    pub text_input_pos: Option<Pos2>,

    /// 드래그 시작 위치
    pub drag_start: Option<Pos2>,

    /// 선택된 도형 인덱스
    pub selected_shape_index: Option<usize>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            background_image: None,
            background_texture: None,
            shapes: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current_shape: None,
            current_tool: ToolType::Select,
            current_color: Color32::RED,
            current_stroke_width: 2.0,
            number_counter: 1,
            text_input: String::new(),
            text_input_pos: None,
            drag_start: None,
            selected_shape_index: None,
        }
    }

    /// 배경 이미지 설정
    pub fn set_background(&mut self, image: ColorImage) {
        self.background_image = Some(image);
        self.background_texture = None; // 텍스처는 다음 프레임에서 생성
    }

    /// 텍스처 업데이트
    pub fn update_texture(&mut self, ctx: &egui::Context) {
        if let Some(ref image) = self.background_image {
            if self.background_texture.is_none() {
                self.background_texture = Some(ctx.load_texture(
                    "background",
                    image.clone(),
                    Default::default(),
                ));
            }
        }
    }

    /// 도형 추가 (undo 스택에 현재 상태 저장)
    pub fn add_shape(&mut self, shape: DrawShape) {
        self.save_state();
        self.shapes.push(shape);
        self.redo_stack.clear();
    }

    /// 마우스 프레스 핸들러
    pub fn on_mouse_press(&mut self, pos: Pos2) {
        self.drag_start = Some(pos);

        match self.current_tool {
            ToolType::Select => {
                // 도형 선택
                self.selected_shape_index = None;
                for (i, shape) in self.shapes.iter().enumerate().rev() {
                    if shape.contains_point(pos) {
                        self.selected_shape_index = Some(i);
                        break;
                    }
                }
            }
            ToolType::Rectangle | ToolType::FilledRect => {
                self.current_shape = Some(DrawShape::Rectangle {
                    rect: Rect::from_two_pos(pos, pos),
                    stroke: Stroke::new(self.current_stroke_width, self.current_color),
                    filled: self.current_tool == ToolType::FilledRect,
                });
            }
            ToolType::Ellipse | ToolType::FilledEllipse => {
                self.current_shape = Some(DrawShape::Ellipse {
                    center: pos,
                    radius_x: 0.0,
                    radius_y: 0.0,
                    stroke: Stroke::new(self.current_stroke_width, self.current_color),
                    filled: self.current_tool == ToolType::FilledEllipse,
                });
            }
            ToolType::Arrow => {
                self.current_shape = Some(DrawShape::Arrow {
                    start: pos,
                    end: pos,
                    stroke: Stroke::new(self.current_stroke_width, self.current_color),
                });
            }
            ToolType::Line => {
                self.current_shape = Some(DrawShape::Line {
                    points: vec![pos],
                    stroke: Stroke::new(self.current_stroke_width, self.current_color),
                });
            }
            ToolType::Curve => {
                self.current_shape = Some(DrawShape::Curve {
                    points: vec![pos],
                    stroke: Stroke::new(self.current_stroke_width, self.current_color),
                });
            }
            ToolType::Text => {
                self.text_input_pos = Some(pos);
                self.text_input.clear();
            }
            ToolType::Number => {
                self.add_shape(DrawShape::Number {
                    center: pos,
                    number: self.number_counter,
                    color: self.current_color,
                    radius: 15.0,
                });
                self.number_counter += 1;
            }
            _ => {}
        }
    }

    /// 마우스 드래그 핸들러
    pub fn on_mouse_drag(&mut self, pos: Pos2) {
        if let Some(start) = self.drag_start {
            match &mut self.current_shape {
                Some(DrawShape::Rectangle { rect, .. }) => {
                    *rect = Rect::from_two_pos(start, pos);
                }
                Some(DrawShape::Ellipse { center, radius_x, radius_y, .. }) => {
                    *center = Pos2::new((start.x + pos.x) / 2.0, (start.y + pos.y) / 2.0);
                    *radius_x = (pos.x - start.x).abs() / 2.0;
                    *radius_y = (pos.y - start.y).abs() / 2.0;
                }
                Some(DrawShape::Arrow { end, .. }) => {
                    *end = pos;
                }
                Some(DrawShape::Line { points, .. }) => {
                    if let Some(last) = points.last_mut() {
                        *last = pos;
                    }
                }
                Some(DrawShape::Curve { points, .. }) => {
                    // 곡선은 계속 점을 추가
                    points.push(pos);
                }
                _ => {}
            }
        }
    }

    /// 마우스 릴리즈 핸들러
    pub fn on_mouse_release(&mut self) {
        if let Some(shape) = self.current_shape.take() {
            // 최소 크기 체크
            let bounds = shape.bounding_rect();
            if bounds.width() > 1.0 || bounds.height() > 1.0 {
                self.add_shape(shape);
            }
        }
        self.drag_start = None;
    }

    /// 텍스트 입력 완료
    pub fn finish_text_input(&mut self) {
        if let Some(pos) = self.text_input_pos.take() {
            if !self.text_input.is_empty() {
                self.add_shape(DrawShape::Text {
                    pos,
                    text: self.text_input.clone(),
                    color: self.current_color,
                    font_size: 16.0,
                });
                self.text_input.clear();
            }
        }
    }

    /// 실행 취소
    pub fn undo(&mut self) {
        if let Some(state) = self.undo_stack.pop() {
            self.redo_stack.push(self.shapes.clone());
            self.shapes = state;
        }
    }

    /// 다시 실행
    pub fn redo(&mut self) {
        if let Some(state) = self.redo_stack.pop() {
            self.undo_stack.push(self.shapes.clone());
            self.shapes = state;
        }
    }

    /// 현재 상태 저장
    fn save_state(&mut self) {
        self.undo_stack.push(self.shapes.clone());
        // 최대 50개 상태만 유지
        if self.undo_stack.len() > 50 {
            self.undo_stack.remove(0);
        }
    }

    /// 선택된 도형 삭제
    pub fn delete_selected(&mut self) {
        if let Some(index) = self.selected_shape_index {
            if index < self.shapes.len() {
                self.save_state();
                self.shapes.remove(index);
                self.selected_shape_index = None;
                self.redo_stack.clear();
            }
        }
    }

    /// 모든 도형 지우기
    pub fn clear_all(&mut self) {
        if !self.shapes.is_empty() {
            self.save_state();
            self.shapes.clear();
            self.redo_stack.clear();
        }
    }

    /// 캔버스를 이미지로 렌더링
    pub fn render_to_image(&self) -> Option<ColorImage> {
        // 배경 이미지가 있어야 함
        let bg_image = self.background_image.as_ref()?;

        // 새 이미지 생성 (배경 복사)
        let mut result = bg_image.clone();

        // 도형들을 이미지에 그리기
        // 주의: 이 부분은 실제로는 더 복잡한 렌더링 로직이 필요
        // 여기서는 간단히 배경만 반환
        // 실제 구현에서는 egui painter를 사용하여 오프스크린 렌더링 필요

        Some(result)
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self::new()
    }
}
