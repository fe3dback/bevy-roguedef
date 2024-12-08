### **Шаг 3: Добавить базовые враги и систему их появления**

#### **Цель**
Создать базовую систему врагов, включающую их появление на карте, движение к Nexus, и взаимодействие с ним. Это обеспечит основу для построения системы волн и увеличения сложности.

---

#### **Подробный план действий**

1. **Создать структуру для врагов**
    - Определить основные параметры врагов:
      ```rust
      struct Enemy {
          health: f32,         // Здоровье врага
          speed: f32,          // Скорость передвижения
          damage: f32,         // Урон Nexus при атаке
      }
 
      struct EnemyStats {
          max_health: f32,
          base_speed: f32,
          attack_damage: f32,
      }
      ```

2. **Добавить компонент врага в Bevy**
    - Определить компонент `EnemyComponent`:
      ```rust
      #[derive(Component)]
      struct EnemyComponent {
          current_health: f32,
          movement_speed: f32,
          attack_damage: f32,
      }
      ```
    - Создать врагов через систему Bevy:
      ```rust
      fn spawn_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>, stats: EnemyStats, position: Vec3) {
          commands.spawn_bundle(SpriteBundle {
              texture: asset_server.load("enemy_texture.png"),
              transform: Transform {
                  translation: position,
                  ..Default::default()
              },
              ..Default::default()
          })
          .insert(EnemyComponent {
              current_health: stats.max_health,
              movement_speed: stats.base_speed,
              attack_damage: stats.attack_damage,
          });
      }
      ```

3. **Реализовать систему появления врагов**
    - **Очередь волн**:
        - Создать структуру данных для управления волнами:
          ```rust
          struct Wave {
              enemies: Vec<EnemyStats>, // Список врагов в волне
              spawn_interval: f32,      // Интервал между появлением врагов
          }
   
          struct WaveSystem {
              current_wave: usize,
              timer: Timer,
              waves: Vec<Wave>,
          }
          ```
    - **Система спавна врагов**:
        - Добавить систему, которая спавнит врагов с интервалом:
          ```rust
          fn spawn_wave_system(
              mut commands: Commands,
              time: Res<Time>,
              mut wave_system: ResMut<WaveSystem>,
              asset_server: Res<AssetServer>,
          ) {
              if wave_system.timer.tick(time.delta()).just_finished() {
                  let wave = &wave_system.waves[wave_system.current_wave];
                  for enemy_stats in &wave.enemies {
                      spawn_enemy(&mut commands, &asset_server, enemy_stats.clone(), Vec3::new(0.0, 0.0, 0.0));
                  }
                  wave_system.current_wave += 1;
              }
          }
          ```

4. **Добавить движение врагов к Nexus**
    - Реализовать систему, которая направляет врагов к цели:
      ```rust
      fn move_enemies_system(
          mut query: Query<(&mut Transform, &EnemyComponent)>,
          nexus_position: Res<Vec3>,
          time: Res<Time>,
      ) {
          for (mut transform, enemy) in query.iter_mut() {
              let direction = (*nexus_position - transform.translation).normalize();
              transform.translation += direction * enemy.movement_speed * time.delta_seconds();
          }
      }
      ```

5. **Реализовать взаимодействие врагов с Nexus**
    - Проверить, достиг ли враг Nexus:
      ```rust
      fn enemy_attack_system(
          mut commands: Commands,
          mut query: Query<(Entity, &Transform, &EnemyComponent)>,
          nexus_position: Res<Vec3>,
          mut nexus_health: ResMut<NexusHealth>,
      ) {
          for (entity, transform, enemy) in query.iter_mut() {
              if transform.translation.distance(*nexus_position) < 5.0 {
                  nexus_health.current -= enemy.attack_damage;
                  commands.entity(entity).despawn();
              }
          }
      }
      ```

6. **Добавить базовую модель и анимацию**
    - Подключить спрайты или анимации для врагов:
        - Использовать простые 2D спрайты для начального тестирования.
        - Для анимации использовать `AnimationPlugin` Bevy:
          ```rust
          commands.spawn_bundle(SpriteSheetBundle {
              texture_atlas: asset_server.load("enemy_spritesheet.png"),
              transform: Transform {
                  translation: position,
                  ..Default::default()
              },
              ..Default::default()
          });
          ```

---

#### **Расширения**
- **Разные типы врагов**:
    - Добавить вариативность врагов (медленные с высоким здоровьем, быстрые с малым уроном и т.д.).
- **Разнообразие волн**:
    - Усложнить волны (например, увеличивая скорость, добавляя уникальные способности врагов).
- **Атака врагов на другие объекты**:
    - Реализовать возможность врагам атаковать баррикады или защитные сооружения на пути к Nexus.

#### **Результат**
После выполнения этого шага на карте будут появляться враги, которые направляются к Nexus и взаимодействуют с ним, закладывая основу для механики волн и прогрессии сложности.