use std::fs;

use rfd::FileDialog;

fn main() {
    // {{{ println!("Операционная система: {}", std::env::consts::OS);
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
    // println!("Пытаемся открыть диалог..."); }}}

    let folder = FileDialog::new().set_title("Выберите папку").pick_folder();

    // println!("Диалог вернул результат: {:?}", folder.is_some());

    match folder {
        Some(path) => {
            println!("Выбрана папка: {:?}", path.display());

            if let Some(parent) = path.parent() {
                if let Some(folder_name) = path.file_name() {
                    let folder_name_str = folder_name.to_string_lossy();
                    let new_folder_name = format!("{}_collected", folder_name_str);
                    let new_path = parent.join(&new_folder_name);

                    match fs::create_dir(new_path) {
                        Ok(()) => println!("Папка {} создана.", &new_folder_name),
                        Err(e) => println!("Произошла ошибка при создании папки: {e}"),
                    }
                } else {
                    println!("Имя папки не найдено");
                }
            } else {
                println!("Не удалось найти родительскую папку.")
            }
        }
        None => println!("Диалог отменен или произошла ошибка"),
    }

    println!("Нажми любую клавишу для завершения.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
}
