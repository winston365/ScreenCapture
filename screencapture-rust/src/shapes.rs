use egui::{Color32, Pos2, Rect, Shape, Stroke};

/// 그리기 도구 종류
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolType {
    Select,      // 선택/이동
    Rectangle,   // 사각형
    FilledRect,  // 채워진 사각형
    Ellipse,     // 타원
    FilledEllipse, // 채워진 타원
    Arrow,       // 화살표
    Line,        // 직선
    Curve,       // 곡선
    Text,        // 텍스트
    Mosaic,      // 모자이크
    Eraser,      // 지우개
    Number,      // 번호 매기기
}

/// 그리기 객체
#[derive(Clone)]
pub enum DrawShape {
    Rectangle {
        rect: Rect,
        stroke: Stroke,
        filled: bool,
    },
    Ellipse {
        center: Pos2,
        radius_x: f32,
        radius_y: f32,
        stroke: Stroke,
        filled: bool,
    },
    Arrow {
        start: Pos2,
        end: Pos2,
        stroke: Stroke,
    },
    Line {
        points: Vec<Pos2>,
        stroke: Stroke,
    },
    Curve {
        points: Vec<Pos2>,
        stroke: Stroke,
    },
    Text {
        pos: Pos2,
        text: String,
        color: Color32,
        font_size: f32,
    },
    Number {
        center: Pos2,
        number: u32,
        color: Color32,
        radius: f32,
    },
}

impl DrawShape {
    /// egui Shape으로 변환
    pub fn to_egui_shape(&self) -> Vec<Shape> {
        match self {
            DrawShape::Rectangle { rect, stroke, filled } => {
                if *filled {
                    vec![Shape::rect_filled(*rect, 0.0, stroke.color)]
                } else {
                    vec![Shape::rect_stroke(*rect, 0.0, *stroke)]
                }
            }
            DrawShape::Ellipse {
                center,
                radius_x,
                radius_y,
                stroke,
                filled,
            } => {
                let rect = Rect::from_center_size(
                    *center,
                    egui::vec2(*radius_x * 2.0, *radius_y * 2.0),
                );
                if *filled {
                    vec![Shape::ellipse_filled(*center, egui::vec2(*radius_x, *radius_y), stroke.color)]
                } else {
                    vec![Shape::ellipse_stroke(*center, egui::vec2(*radius_x, *radius_y), *stroke)]
                }
            }
            DrawShape::Arrow { start, end, stroke } => {
                let mut shapes = vec![Shape::line_segment([*start, *end], *stroke)];

                // 화살표 머리 그리기
                let dir = (*end - *start).normalized();
                let perp = egui::vec2(-dir.y, dir.x);
                let arrow_size = 10.0;

                let arrow_point1 = *end - dir * arrow_size + perp * arrow_size * 0.5;
                let arrow_point2 = *end - dir * arrow_size - perp * arrow_size * 0.5;

                shapes.push(Shape::line_segment([*end, arrow_point1], *stroke));
                shapes.push(Shape::line_segment([*end, arrow_point2], *stroke));
                shapes
            }
            DrawShape::Line { points, stroke } => {
                if points.len() < 2 {
                    return vec![];
                }
                vec![Shape::line(points.clone(), *stroke)]
            }
            DrawShape::Curve { points, stroke } => {
                if points.len() < 2 {
                    return vec![];
                }
                // 곡선은 여러 작은 선분으로 근사
                vec![Shape::line(points.clone(), *stroke)]
            }
            DrawShape::Text { pos, text, color, font_size } => {
                // Note: Text rendering requires galley which needs fonts context
                // For now, we'll use a simple rect to indicate text position
                // In practice, this should be rendered via painter.text() in the UI code
                let rect = Rect::from_min_size(*pos, egui::vec2(text.len() as f32 * font_size * 0.6, *font_size));
                vec![Shape::rect_stroke(rect, 0.0, Stroke::new(1.0, *color))]
            }
            DrawShape::Number { center, number, color, radius } => {
                vec![Shape::circle_filled(*center, *radius, *color)]
            }
        }
    }

    /// 바운딩 박스 가져오기
    pub fn bounding_rect(&self) -> Rect {
        match self {
            DrawShape::Rectangle { rect, .. } => *rect,
            DrawShape::Ellipse { center, radius_x, radius_y, .. } => {
                Rect::from_center_size(*center, egui::vec2(*radius_x * 2.0, *radius_y * 2.0))
            }
            DrawShape::Arrow { start, end, .. } => {
                Rect::from_two_pos(*start, *end)
            }
            DrawShape::Line { points, .. } | DrawShape::Curve { points, .. } => {
                if points.is_empty() {
                    return Rect::NOTHING;
                }
                let mut min = points[0];
                let mut max = points[0];
                for p in points {
                    min = min.min(*p);
                    max = max.max(*p);
                }
                Rect::from_two_pos(min, max)
            }
            DrawShape::Text { pos, .. } => {
                Rect::from_min_size(*pos, egui::vec2(100.0, 20.0))
            }
            DrawShape::Number { center, radius, .. } => {
                Rect::from_center_size(*center, egui::vec2(*radius * 2.0, *radius * 2.0))
            }
        }
    }

    /// 점이 도형 내부에 있는지 확인
    pub fn contains_point(&self, point: Pos2) -> bool {
        self.bounding_rect().contains(point)
    }
}

/// 색상 팔레트
pub const COLORS: [Color32; 8] = [
    Color32::from_rgb(255, 0, 0),     // 빨강
    Color32::from_rgb(255, 255, 0),   // 노랑
    Color32::from_rgb(0, 255, 0),     // 초록
    Color32::from_rgb(0, 255, 255),   // 청록
    Color32::from_rgb(0, 0, 255),     // 파랑
    Color32::from_rgb(255, 0, 255),   // 보라
    Color32::from_rgb(255, 192, 203), // 분홍
    Color32::from_rgb(0, 0, 0),       // 검정
];

/// 선 두께 옵션
pub const STROKE_WIDTHS: [f32; 5] = [1.0, 2.0, 3.0, 5.0, 8.0];
