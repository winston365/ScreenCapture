use image::{ImageBuffer, Rgba};

/// 화면 캡처 결과
pub struct CaptureResult {
    pub image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub width: u32,
    pub height: u32,
}

#[cfg(target_os = "windows")]
mod windows_capture {
    use super::*;
    use windows::Win32::Foundation::*;
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::UI::WindowsAndMessaging::*;

    /// Windows GDI를 사용하여 전체 가상 화면 캡처
    pub fn capture_screen_impl() -> Result<CaptureResult, String> {
        unsafe {
            // 가상 화면 크기 가져오기 (멀티 모니터 지원)
            let x = GetSystemMetrics(SYSTEM_METRICS_INDEX(76)); // SM_XVIRTUALSCREEN
            let y = GetSystemMetrics(SYSTEM_METRICS_INDEX(77)); // SM_YVIRTUALSCREEN
            let width = GetSystemMetrics(SYSTEM_METRICS_INDEX(78)); // SM_CXVIRTUALSCREEN
            let height = GetSystemMetrics(SYSTEM_METRICS_INDEX(79)); // SM_CYVIRTUALSCREEN

            if width <= 0 || height <= 0 {
                return Err("Invalid screen dimensions".to_string());
            }

            // 화면 DC 가져오기
            let hdc_screen = GetDC(HWND(0));

            // 메모리 DC 생성
            let hdc_mem = CreateCompatibleDC(hdc_screen);

            // 비트맵 생성
            let hbitmap = CreateCompatibleBitmap(hdc_screen, width, height);

            // 비트맵 선택
            let old_bitmap = SelectObject(hdc_mem, hbitmap);

            // 화면을 메모리 DC로 복사
            BitBlt(
                hdc_mem,
                0,
                0,
                width,
                height,
                hdc_screen,
                x,
                y,
                SRCCOPY,
            );

            // BITMAPINFO 구조체 설정
            let mut bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width,
                    biHeight: -height, // top-down DIB
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0 as u32,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [RGBQUAD::default()],
            };

            // 이미지 데이터 버퍼
            let buffer_size = (width * height * 4) as usize;
            let mut buffer: Vec<u8> = vec![0; buffer_size];

            // 비트맵 데이터 가져오기
            GetDIBits(
                hdc_mem,
                hbitmap,
                0,
                height as u32,
                Some(buffer.as_mut_ptr() as *mut _),
                &mut bmi,
                DIB_RGB_COLORS,
            );

            // 리소스 정리
            SelectObject(hdc_mem, old_bitmap);
            DeleteObject(hbitmap);
            DeleteDC(hdc_mem);
            ReleaseDC(HWND(0), hdc_screen);

            // BGRA를 RGBA로 변환
            for chunk in buffer.chunks_exact_mut(4) {
                chunk.swap(0, 2); // B <-> R
            }

            // ImageBuffer 생성
            let image = ImageBuffer::from_raw(width as u32, height as u32, buffer)
                .ok_or_else(|| "Failed to create image buffer".to_string())?;

            Ok(CaptureResult {
                image,
                width: width as u32,
                height: height as u32,
            })
        }
    }

    /// 가상 화면의 경계 가져오기
    pub fn get_virtual_screen_bounds_impl() -> (i32, i32, i32, i32) {
        unsafe {
            let x = GetSystemMetrics(SYSTEM_METRICS_INDEX(76));
            let y = GetSystemMetrics(SYSTEM_METRICS_INDEX(77));
            let width = GetSystemMetrics(SYSTEM_METRICS_INDEX(78));
            let height = GetSystemMetrics(SYSTEM_METRICS_INDEX(79));
            (x, y, width, height)
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod dummy_capture {
    use super::*;

    /// 더미 구현 (테스트용)
    pub fn capture_screen_impl() -> Result<CaptureResult, String> {
        // 1920x1080 빈 이미지 생성 (테스트용)
        let width = 1920;
        let height = 1080;
        let buffer = vec![100u8; (width * height * 4) as usize];

        let image = ImageBuffer::from_raw(width, height, buffer)
            .ok_or_else(|| "Failed to create dummy image buffer".to_string())?;

        Ok(CaptureResult {
            image,
            width,
            height,
        })
    }

    /// 더미 구현
    pub fn get_virtual_screen_bounds_impl() -> (i32, i32, i32, i32) {
        (0, 0, 1920, 1080)
    }
}

/// 화면 캡처 공개 함수
pub fn capture_screen() -> Result<CaptureResult, String> {
    #[cfg(target_os = "windows")]
    return windows_capture::capture_screen_impl();

    #[cfg(not(target_os = "windows"))]
    return dummy_capture::capture_screen_impl();
}

/// 특정 영역만 캡처
pub fn capture_region(x: i32, y: i32, width: i32, height: i32) -> Result<CaptureResult, String> {
    // 전체 화면을 캡처한 후 크롭
    let full_capture = capture_screen()?;
    let cropped = image::imageops::crop_imm(
        &full_capture.image,
        x as u32,
        y as u32,
        width as u32,
        height as u32,
    )
    .to_image();

    Ok(CaptureResult {
        image: cropped,
        width: width as u32,
        height: height as u32,
    })
}

/// 가상 화면의 경계 가져오기
pub fn get_virtual_screen_bounds() -> (i32, i32, i32, i32) {
    #[cfg(target_os = "windows")]
    return windows_capture::get_virtual_screen_bounds_impl();

    #[cfg(not(target_os = "windows"))]
    return dummy_capture::get_virtual_screen_bounds_impl();
}
