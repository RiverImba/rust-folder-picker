use rfd::FileDialog;

fn main() {
    // println!("Операционная система: {}", std::env::consts::OS);
    // println!("Архитектура: {}", std::env::consts::ARCH);
    //
    // println!("Проверяем переменные окружения:");
    // println!("DISPLAY: {:?}", std::env::var("DISPLAY"));
    // println!("WAYLAND_DISPLAY: {:?}", std::env::var("WAYLAND_DISPLAY"));
    // println!(
    //     "XDG_CURRENT_DESKTOP: {:?}",
    //     std::env::var("XDG_CURRENT_DESKTOP")
    // );
    //
    // println!("Пытаемся открыть диалог...");

    let folder = FileDialog::new().set_title("Выберите папку").pick_folder();

    // println!("Диалог вернул результат: {:?}", folder.is_some());

    match folder {
        Some(path) => println!("Выбрана папка: {:?}", path.display()),
        None => println!("Диалог отменен или произошла ошибка"),
    }
}

