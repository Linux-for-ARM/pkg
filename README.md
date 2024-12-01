# pkg

`pkg` — программа для работы с метаданными пакетов в руководстве «Linux for ARM». Она может добавлять и удалять пакеты из индекса, а также изменять информацию об уже существующем ПО. Использует простой псевдографический интерфейс на базе `cursive`.

## Возможности

- [X] Отображение списка существующего в LFA программного обеспечения;
- [X] Генерация файлов `wget-list`, `md5sums`;
- [X] Генерация файлов `md/pkgs.md` и `md/**/pkgs/*.md`;
- [ ] Добавление новых пакетов в индекс;
- [ ] Удаление ненужных пакетов из индекса;
- [X] Изменение информации о пакетах в индексе;

## Сборка

### Требования

- Наличие руководства LFA в локальном виде;
- `rustc` для сборки `pkg`;

### Сборка `pkg`

```bash
cargo build --release
cp -v ./target/release/pkg ~/.local/bin
export PATH=$PATH:$HOME/.local/bin
```

<details>
  <summary><b>Донат</b></summary>
  <p>Вы можете отблагодарить автора за проделанную работу:</p>
  <blockquote>2202206252335406 (Сбер; Михаил Сергеевич)</blockquote>
</details>