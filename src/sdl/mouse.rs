use std::libc::c_int;

use get_error;

pub mod ll {
	use std::libc::{c_void, c_int, uint8_t, uint16_t, int16_t};

	use Rect;

	pub static SDL_DISABLE: c_int = 0;
	pub static SDL_ENABLE: c_int = 1;
	pub static SDL_QUERY: c_int = -1;

	pub type WMcursor = c_void;
	pub struct SDL_Cursor {
	     area: Rect,
	     hot_x: int16_t,
	     hot_y: int16_t,
	     data: *uint8_t,
	     mask: *uint8_t,
	     save: [*uint8_t, ..2],
	     wm_cursor: *WMcursor,
	}

    extern "C" {
        pub fn SDL_ShowCursor(toggle: c_int) -> c_int;
        pub fn SDL_CreateCursor(data: *uint8_t,
                                      mask: *uint8_t,
                                      w: c_int,
                                      h: c_int,
                                      hot_x: c_int,
                                      hot_y: c_int)
                    -> *SDL_Cursor;
        pub fn SDL_SetCursor(cursor: *SDL_Cursor);
        pub fn SDL_GetCursor() -> *SDL_Cursor;
        pub fn SDL_FreeCursor(cursor: *SDL_Cursor);
        pub fn SDL_WarpMouse(x: uint16_t, y: uint16_t);
    }
}

fn warp_mouse(x: u16, y: u16) {
	unsafe { ll::SDL_WarpMouse(x, y); }
}

#[deriving(Eq)]
pub struct Cursor {
	raw: *ll::SDL_Cursor,
	owned: bool
}

fn wrap_cursor(raw: *ll::SDL_Cursor, owned: bool) -> ~Cursor {
	~Cursor {
		raw: raw,
		owned: owned
	}
}

impl Cursor {
	pub fn new(data: &[u8], mask: &[u8], w: int, h: int, hot_x: int, hot_y: int)
        -> Result<~Cursor, ~str> {
		unsafe {
			let raw = ll::SDL_CreateCursor(data.as_ptr(), mask.as_ptr(),
				                           w as c_int, h as c_int, hot_x as c_int,
				                           hot_y as c_int);

				if raw.is_null() { Err(get_error()) }
				else { Ok(wrap_cursor(raw, true)) }
		}
	}
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe {
        	if self.owned {
        		ll::SDL_FreeCursor(self.raw);
        	}
        }
    }
}

pub fn set_cursor(cursor: &Cursor) {
	unsafe { ll::SDL_SetCursor(cursor.raw); }
}

pub fn get_cursor() -> ~Cursor {
	unsafe { wrap_cursor(ll::SDL_GetCursor(), false) }
}

pub fn set_cursor_visible(visible: bool) {
	unsafe { ll::SDL_ShowCursor(visible as c_int); }
}

pub fn toggle_cursor_visible() {
	unsafe {
		if ll::SDL_ShowCursor(ll::SDL_QUERY) == ll::SDL_ENABLE {
			ll::SDL_ShowCursor(ll::SDL_DISABLE);
		} else {
			ll::SDL_ShowCursor(ll::SDL_ENABLE);
		}
	}
}

pub fn is_cursor_visible() -> bool {
	unsafe { ll::SDL_ShowCursor(ll::SDL_QUERY) == ll::SDL_ENABLE }
}
