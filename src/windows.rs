//! Janela sem moldura de verdade no Windows. Sem decoração, o winit ainda
//! deixa o estilo `WS_CAPTION` (só esconde a barra de título), e com ele o
//! DWM desenha a sombra em volta da área transparente e, no Windows 11, os
//! cantos arredondados com borda. Aqui a janela vira `WS_POPUP`; o winit volta
//! o estilo quando muda o tamanho dela (ex.: ao abrir o menu), então isto
//! roda a cada quadro e só age quando o `WS_CAPTION` voltou.

use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::Graphics::Dwm::{
    DWMWA_BORDER_COLOR, DWMWA_COLOR_NONE, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_DONOTROUND,
    DwmSetWindowAttribute,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GWL_STYLE, GetWindowLongPtrW, SWP_FRAMECHANGED, SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE,
    SWP_NOZORDER, SetWindowLongPtrW, SetWindowPos, WS_CAPTION, WS_MAXIMIZEBOX, WS_MINIMIZEBOX,
    WS_POPUP, WS_SYSMENU, WS_THICKFRAME,
};

/// Tira a moldura que o DWM desenharia em volta da janela.
pub fn borderless(window: &impl HasWindowHandle) {
    let Ok(handle) = window.window_handle() else {
        return;
    };
    let RawWindowHandle::Win32(handle) = handle.as_raw() else {
        return;
    };
    let hwnd = handle.hwnd.get() as HWND;
    let frame =
        (WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_THICKFRAME) as isize;
    // SAFETY: `hwnd` é a janela desta aplicação, viva enquanto o eframe roda;
    // os atributos do DWM vão por ponteiro com o tamanho certo.
    unsafe {
        let style = GetWindowLongPtrW(hwnd, GWL_STYLE);
        if style & WS_CAPTION as isize == 0 {
            return;
        }
        SetWindowLongPtrW(hwnd, GWL_STYLE, (style & !frame) | WS_POPUP as isize);
        SetWindowPos(
            hwnd,
            std::ptr::null_mut(),
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
        );
        // Windows 11; os anteriores ignoram atributos que não conhecem.
        let corner = DWMWCP_DONOTROUND;
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE as u32,
            (&corner as *const i32).cast(),
            size_of::<i32>() as u32,
        );
        let border = DWMWA_COLOR_NONE;
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_BORDER_COLOR as u32,
            (&border as *const u32).cast(),
            size_of::<u32>() as u32,
        );
    }
}
