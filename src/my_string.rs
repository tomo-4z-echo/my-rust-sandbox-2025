use std::alloc::{alloc, dealloc, Layout};
use std::ptr;   // ポインタ操作用

pub struct MyPoorMansString {
    pub ptr: *mut u8,   // 文字列データへのポインタ
    pub len: usize,     // 現在の文字列の長さ（バイト数）
    pub cap: usize,     // 割り当てられたメモリ容量（バイト数）
}

impl MyPoorMansString {
    pub fn new(s: &str) -> Self {
        let len = s.len();
        let cap = len.next_power_of_two().max(4);   // 少なくとも4バイト確保（2の冪乗）
        let layout = Layout::array::<u8>(cap).unwrap();

        // 警告：ここからunsafeコード
        let ptr = unsafe {
            alloc(layout)  // ヒープメモリを確保
        };

        if ptr.is_null() {
            // メモリ確保失敗時の処理
            panic!("memory allocation failed");
        }

        unsafe {
            // 文字列データをコピー
            ptr::copy_nonoverlapping(s.as_ptr(), ptr, len);
        }

        MyPoorMansString { ptr, len, cap }
    }

    pub fn push_str(&mut self, s: &str) {
        let new_len = self.len + s.len();
        if new_len > self.cap {
            // 容量が足りなければ再確保
            let old_layout = Layout::array::<u8>(self.cap).unwrap();
            self.cap = new_len.next_power_of_two();
            let new_layout = Layout::array::<u8>(self.cap).unwrap();

            unsafe {
                let new_ptr = alloc(new_layout);    // 新しいメモリを確保
                if new_ptr.is_null() {
                    panic!("memory re-allocation failed");
                }
                ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);  // 古いデータをコピー
                dealloc(self.ptr, old_layout);  // 古いメモリを解放
                self.ptr = new_ptr; // ポインタを更新
            }
        }
        unsafe {
            // 新しい文字列データを追加
            ptr::copy_nonoverlapping(s.as_ptr(), self.ptr.add(self.len), s.len());
        }
        self.len = new_len;
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            // バイトポインタから&strを作成（UTF-8であることを前提）
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.ptr, self.len))
        }
    }
}

// ドロップトレイトを実装して、スコープを抜けるときにメモリを解放する
impl Drop for MyPoorMansString {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let layout = Layout::array::<u8>(self.cap).unwrap();
            unsafe {
                dealloc(self.ptr, layout);  // メモリを解放
            }
        }
    }
}
