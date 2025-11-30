# ScreenCapture Rust

**100% Rust로 작성된 고성능 화면 캡처 및 주석 도구**

## 🚀 개요

기존 Qt/C++ 기반 ScreenCapture를 순수 Rust로 재구현한 프로젝트입니다. egui GUI 프레임워크와 Windows API를 사용하여 가볍고 빠른 화면 캡처 도구를 제공합니다.

## ✨ 주요 기능

### 화면 캡처
- ✅ 전체 화면 캡처 (멀티 모니터 지원)
- ✅ 영역 선택 캡처
- ✅ 고해상도(HiDPI) 지원

### 그리기 도구
- **도형**: 사각형, 원형, 화살표
- **선**: 직선, 곡선
- **텍스트 주석**: 자유 텍스트 입력
- **번호 매기기**: 순차적 번호 표시
- **모자이크/지우개**: 민감한 정보 가리기

### 편집 기능
- **실행 취소/다시 실행**: Ctrl+Z / Ctrl+Y
- **색상 선택**: 8가지 기본 색상
- **선 두께 조절**: 5단계 (1px ~ 8px)
- **도형 선택/삭제**: 클릭하여 선택, Delete 키로 삭제

### 내보내기
- **파일 저장**: PNG 형식으로 저장
- **클립보드 복사**: Ctrl+C
- **색상 정보 복사**:
  - RGB 형식 (Ctrl+R)
  - HEX 형식 (Ctrl+H)
  - CMYK 형식

## 🛠️ 기술 스택

| 분류 | 기술 |
|------|------|
| **언어** | Rust 2021 Edition |
| **GUI** | egui 0.30 + eframe |
| **Windows API** | windows-rs 0.58 |
| **이미지 처리** | image 0.25 |
| **클립보드** | arboard 3.4 |
| **빌드 최적화** | LTO, 단일 codegen unit |

## 📦 설치 및 빌드

### 요구 사항
- Rust 1.70 이상
- Windows 10 이상 (프로덕션)
- Linux (개발/테스트 가능)

### 빌드 방법

```bash
# 저장소 클론
cd screencapture-rust

# 개발 빌드
cargo build

# 릴리스 빌드 (최적화됨)
cargo build --release

# 실행
cargo run --release
```

### 릴리스 바이너리 크기
- **Debug**: ~50-80MB
- **Release**: ~5-10MB (strip 후)

## 🎮 사용법

### 기본 워크플로우

1. **캡처 시작**: "전체 화면 캡처" 버튼 클릭
2. **도구 선택**: 상단 툴바에서 원하는 도구 선택
3. **그리기**: 마우스로 화면에 주석 추가
4. **저장/복사**:
   - 💾 저장: 버튼 클릭 또는 Ctrl+S
   - 📋 클립보드 복사: Ctrl+C

### 키보드 단축키

| 키 | 기능 |
|----|------|
| `Ctrl+Z` | 실행 취소 |
| `Ctrl+Y` | 다시 실행 |
| `Ctrl+S` | 파일 저장 |
| `Ctrl+C` | 클립보드 복사 |
| `Ctrl+R` | RGB 색상 복사 |
| `Ctrl+H` | HEX 색상 복사 |
| `Delete` | 선택된 도형 삭제 |
| `ESC` | 프로그램 종료 |

## 🏗️ 프로젝트 구조

```
screencapture-rust/
├── src/
│   ├── main.rs           # 애플리케이션 진입점 및 UI
│   ├── capture.rs        # Windows 화면 캡처 (GDI)
│   ├── canvas.rs         # 그리기 캔버스 & 상태 관리
│   ├── shapes.rs         # 도형 정의 (Rect, Ellipse, Arrow 등)
│   ├── toolbar.rs        # UI 툴바 컴포넌트
│   └── clipboard.rs      # 클립보드 및 파일 저장
├── Cargo.toml            # 프로젝트 설정 및 의존성
└── README.md             # 이 문서
```

## 🔧 아키텍처

### 모듈 구조

```
ScreenCaptureApp (main.rs)
├── Canvas (canvas.rs)
│   ├── background_image: ColorImage
│   ├── shapes: Vec<DrawShape>
│   ├── undo_stack: Vec<Vec<DrawShape>>
│   └── redo_stack: Vec<Vec<DrawShape>>
│
├── Toolbar (toolbar.rs)
│   ├── current_tool: ToolType
│   ├── current_color: Color32
│   └── current_stroke_width: f32
│
└── Capture (capture.rs)
    ├── capture_screen() -> CaptureResult
    ├── capture_region() -> CaptureResult
    └── get_virtual_screen_bounds() -> (i32, i32, i32, i32)
```

### Windows API 통합

- **BitBlt**: 화면 픽셀 복사
- **GetDC / CreateCompatibleDC**: 디바이스 컨텍스트 관리
- **GetDIBits**: 비트맵 데이터 추출
- **GetSystemMetrics**: 가상 화면 크기 조회

### 크로스 플랫폼 지원

```rust
#[cfg(target_os = "windows")]
mod windows_capture { /* 실제 Windows API 구현 */ }

#[cfg(not(target_os = "windows"))]
mod dummy_capture { /* 테스트용 더미 구현 */ }
```

## 📊 성능 비교

| 항목 | Qt/C++ (원본) | Rust (이 프로젝트) |
|------|---------------|-------------------|
| **바이너리 크기** | ~15MB | ~8MB |
| **메모리 사용량** | ~30MB | ~40MB |
| **시작 속도** | ⚡ 즉시 | ⚡ 즉시 |
| **렌더링 성능** | 네이티브 | egui (60fps) |
| **크로스 플랫폼** | ✅ | ✅ (조건부) |
| **개발 생산성** | 중 | 높음 |
| **메모리 안전성** | 수동 관리 | 컴파일 타임 보장 |

## 🚧 현재 상태 및 제한사항

### ✅ 구현 완료
- 화면 캡처 (Windows API)
- 기본 그리기 도구 (사각형, 원, 화살표, 선)
- 실행 취소/다시 실행
- 클립보드 및 파일 저장
- 색상/두께 선택
- 키보드 단축키

### 🚧 개선 필요
- **텍스트 렌더링**: 현재 간단한 박스로만 표시 (egui 폰트 시스템 통합 필요)
- **스크롤 캡처**: 미구현 (원본 Qt 앱에는 있음)
- **모자이크/지우개**: UI만 있고 실제 구현 없음
- **이미지 오프스크린 렌더링**: 저장 시 도형이 포함되지 않음

### 🔮 향후 계획
1. 완전한 텍스트 렌더링 지원
2. 이미지 합성 (배경 + 도형) 렌더링
3. 스크롤 캡처 기능
4. 설정 UI (다국어, 단축키 커스터마이징)
5. 추가 도형 (다각형, 스탬프)
6. 애니메이션 효과

## 🤝 기여

이슈나 풀 리퀘스트를 환영합니다!

### 개발 가이드

```bash
# 코드 포맷
cargo fmt

# 린트 검사
cargo clippy

# 테스트 실행
cargo test
```

## 📄 라이선스

원본 프로젝트의 라이선스를 따릅니다.
Copyright (C) LiuXiaolun 2023-2025

## 🙏 감사의 말

- **원본 프로젝트**: [ScreenCapture](https://github.com/xland/ScreenCapture) by xland
- **egui**: 뛰어난 즉시 모드 GUI 프레임워크
- **windows-rs**: Rust를 위한 Windows API 바인딩

---

**Made with ❤️ and 🦀 Rust**
