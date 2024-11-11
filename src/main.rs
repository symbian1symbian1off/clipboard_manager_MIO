use copypasta::{ClipboardContext, ClipboardProvider};
use std::collections::VecDeque;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut clipboard = ClipboardContext::new().unwrap();
    let mut history: VecDeque<String> = VecDeque::new();
    let max_history = 10;

    loop {
        // دریافت متن از کلیپ‌بورد
        if let Ok(contents) = clipboard.get_contents() {
            // بررسی اینکه متن تکراری نیست
            if history.front() != Some(&contents) {
                // اضافه کردن به تاریخچه
                history.push_front(contents.clone());

                // اگر تاریخچه بیشتر از مقدار مجاز شد، مورد قدیمی را حذف کنید
                if history.len() > max_history {
                    history.pop_back();
                }

                println!("Clipboard History: {:?}", history);
            }
        }

        // تنظیم یک وقفه برای کاهش مصرف پردازنده
        sleep(Duration::from_secs(1));
    }
}

