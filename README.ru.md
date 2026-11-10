# Treepack

![Rust](https://img.shields.io/badge/Rust-2024-dea584?logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue)
[![lang en](https://img.shields.io/badge/lang-en-red)](README.md)

Консольная утилита для рекурсивного сбора текстовых файлов UTF-8 в один Markdown-документ. Пути к файлам записываются в заголовки, содержимое в блоки кода.

## Установка

Установите [Rust и Cargo](https://www.rust-lang.org/tools/install) и выполните из каталога проекта:

```sh
cargo install --path . --locked
```

## Использование

Собрать текущий каталог в `treepack.md`:

```sh
treepack
```

Указать исходный каталог и выходной файл:

```sh
treepack -d ./src -o sources.md
```

| Параметр | Описание | По умолчанию |
| --- | --- | --- |
| `-d, --directory` | Каталог для обхода | `.` |
| `-o, --output` | Выходной файл; если расширение не указано, добавляется `.md` | `treepack.md` |
| `-h, --help` | Показать справку | |
| `-V, --version` | Показать версию | |

Выходной файл перезаписывается и исключается из обхода. Недоступные для чтения и нетекстовые файлы пропускаются. Скрытые файлы и каталоги включаются; правила `.gitignore` не учитываются.

## Лицензия

Лицензия: [Apache-2.0](LICENSE-APACHE) или [MIT](LICENSE-MIT), на ваш выбор.
