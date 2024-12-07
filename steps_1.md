Для реализации шага 1 ("Добавить базового игрока и управление") в контексте игры *Defense of the Nexus* и учитывая, что вы используете Rust и Bevy, вот детализированный план с примером настройки ассетов в формате `ron`:

---

### Расширенный план для шага 1

#### **1. Создание базовой модели игрока**
1.1. Создайте файл ассета для описания базового игрока.
1.2. Используйте простой спрайт для отображения персонажа (например, пиксель-арт в стиле защитников Nexus).
1.3. Включите основные свойства: позицию, скорость перемещения и ограничения области.

---

#### **2. Реализация управления персонажем**
2.1. Настройте систему управления через WASD или стрелки.
2.2. Реализуйте логику перемещения в системе Bevy, используя `Transform` и `Velocity`.
2.3. Ограничьте движение игрока границами карты.

---

#### **3. Ограничение области перемещения**
3.1. Определите границы карты (например, прямоугольник).
3.2. Добавьте проверку в системе перемещения, чтобы игрок не выходил за границы.

---

### Пример файлов ассетов в формате RON

#### **1. Конфигурация персонажа (player.ron)**
```ron
Player(
    sprite: "assets/sprites/player.png", // Путь к изображению спрайта
    size: Vec2(32.0, 32.0),              // Размер спрайта (пиксели)
    speed: 200.0,                        // Скорость перемещения (пикселей/сек)
    bounds: Rect {
        min: Vec2(0.0, 0.0),             // Нижний левый угол карты
        max: Vec2(1024.0, 768.0),        // Верхний правый угол карты
    },
)
```

#### **2. Границы карты (map_bounds.ron)**
```ron
MapBounds(
    width: 1024.0,
    height: 768.0,
)
```

---

### Реализация в коде

#### **1. Загрузка ассетов**
Загрузите данные из файлов `ron` с помощью библиотеки `serde`:
```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Player {
    sprite: String,
    size: (f32, f32),
    speed: f32,
    bounds: Rect,
}

#[derive(Deserialize)]
struct Rect {
    min: (f32, f32),
    max: (f32, f32),
}

// Загрузка из файла RON
fn load_player_config(path: &str) -> Player {
    let content = std::fs::read_to_string(path).expect("Failed to read player config");
    ron::from_str(&content).expect("Failed to parse player config")
}
```

#### **2. Система управления игроком**
```rust
use bevy::prelude::*;

struct Player;
struct Velocity(Vec2);

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        // Нормализация направления и применение скорости
        let move_delta = direction.normalize_or_zero() * velocity.0 * time.delta_seconds();

        // Обновление позиции с ограничением
        let new_position = transform.translation + move_delta.extend(0.0);
        transform.translation = new_position.clamp(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1024.0, 768.0, 0.0),
        );
    }
}
```

#### **3. Регистрация компонентов и систем**
```rust
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(load_player_config("assets/player.ron"))
        .add_startup_system(setup_player.system())
        .add_system(player_movement_system.system())
        .run();
}

fn setup_player(mut commands: Commands, player_config: Res<Player>, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(&player_config.sprite),
            transform: Transform::from_translation(Vec3::new(512.0, 384.0, 0.0)),
            sprite: Sprite::new(player_config.size.into()),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec2::new(player_config.speed, player_config.speed)));
}
```

---

Эти шаги создадут базового игрока, которым можно управлять, а также ограничат область его перемещения в рамках карты. Вы можете расширить систему, добавив анимации и взаимодействие с другими объектами.