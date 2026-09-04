# Rust in Bevy：Observability：先设计可观测性，再做更多逻辑

## 对应代码

下面这段来自根项目的实际实现：它不是“概念说明”，而是 `GameState`、`Query`、`ResMut` 和 `FixedUpdate` 在真实代码里如何协作。

```rust
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

fn apply_game_state(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    session: Res<GameSession>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if *state.get() == GameState::Playing && session.health <= 0 {
        next_state.set(GameState::GameOver);
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

fn player_move(
    time: Res<Time>,
    state: Res<State<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    if *state.get() != GameState::Playing {
        return;
    }

    let move_x = (keyboard.pressed(KeyCode::KeyD) as i8 - keyboard.pressed(KeyCode::KeyA) as i8) as f32;
    let move_y = (keyboard.pressed(KeyCode::KeyW) as i8 - keyboard.pressed(KeyCode::KeyS) as i8) as f32;
    let dir = Vec2::new(move_x, move_y);
    let delta = time.delta_secs();

    for (mut transform, mut velocity) in &mut query {
        if dir.length_squared() > 0.0 {
            let dir = dir.normalize();
            velocity.x = dir.x * PLAYER_SPEED;
            velocity.y = dir.y * PLAYER_SPEED;
            transform.translation.x += dir.x * PLAYER_SPEED * delta;
            transform.translation.y += dir.y * PLAYER_SPEED * delta;
        }
    }
}
```

## 这段代码在说什么

这不是“把很多概念堆在一起”的示例，而是一个最小可运行的规则集：

- `GameState` 表明：菜单、游戏中、结束界面是不同阶段，系统必须显式分流；
- `Res<State<GameState>>` 让系统读取当前阶段，而不是从全局变量里偷偷拿状态；
- `ResMut<NextState<GameState>>` 表示状态切换是一个有先后顺序的事件，不是随手 `if` 打断；
- `Query<(&mut Transform, &mut Velocity), With<Player>>` 说明：移动逻辑只操作玩家实体，不碰其他对象；
- `FixedUpdate` 下的 `time.delta_secs()` 表示物理和动作逻辑应该在可复现的时间步里演化，而不是依赖帧率。

换句话说，真实项目里，ECS 的难点不是写出一堆 `System`，而是把“状态归属、数据所有权、调度顺序”想清楚。一个 `System` 只负责一类事实：读取输入、更新 Transform、判定碰撞、改变状态。

## 具体知识与操作手册

### 1. 先分清输入、状态和效果

很多代码写法的问题，不是语法错，而是角色混住：

- 输入层：`ButtonInput<KeyCode>` 负责读键盘；
- 状态层：`GameState` 负责阶段；
- 结果层：`Transform`、`Velocity`、`GameSession` 负责真实世界变化。

如果把输入、状态、视觉反馈写进同一个结构体，后面调试时你会发现“为什么按下一个键，游戏结果和代码路径不一致？”因为系统边界被打破了。

### 2. `Query` 不是“找一个大对象”，而是“筛选集合中的正确实体”

`With<Player>` 这种过滤条件很关键。它意味着：

- 你只改玩家的 `Transform`；
- 你不必在系统里去判断某个实体是不是敌人；
- 代码的读者一眼就知道这条逻辑的作用域。

这也是 ECS 设计的核心：对象不存在“万能长相”，而是通过组件和筛选条件组合成真实的行为。

### 3. `FixedUpdate` 解决的是“时间不是帧率”问题

在 `player_move` 里，速度乘上 `delta` 让移动和帧率解耦。若你把 `delta` 直接写死，或者直接把 `Update` 里的 `time.delta_secs()` 当成固定物理模拟，会在不同机器上出现：

- 速度不稳定；
- 碰撞提前/延后；
- 子弹、敌人和玩家的同步会出现“看起来像随机”的 bug。

这就是固定步的价值：让规则稳定，调试也更可靠。

### 4. 读代码时，先找“谁拥有状态”

一个 Bevy 程序最值得问的不是“这个函数是不是长得很高级”，而是：

- 这个值存在哪里？
- 谁写它？
- 谁读它？
- 它是在 `Resource` 里，还是 `Component` 里？

`GameState` 和 `GameSession` 这类数据要放在资源里，是因为它们描述的是全局游戏事实；`Velocity` 和 `Transform` 属于实体，因为它们是玩家/敌人自身的状态。

## 实战诀窍

- 如果一个系统想同时读取和修改太多东西，先拆出来；
- 把 `State` 访问写进 `run_if` 或分支，别让系统隐式依赖全局变量；
- `Query` 的过滤条件要写得具体，不要把全局判断塞进循环里；
- 如果你怀疑“为什么一帧后会异常”，先看 `delta`、`FixedUpdate` 和 `Input` 的时间来源；
- 先用 `println!` 或 `debug_assert!` 在关卡边界上验证状态变化，再考虑抽象。

## 练习

1. 把 `GameState` 画成一个状态转换图：`Menu -> Playing -> GameOver`，说明每个状态对应哪类系统。
2. 复制一段 `player_move` 代码，改成“角色速度来自资源而不是直接写在组件里”，判断这样做有什么收益和代价。
3. 把 `apply_game_state` 中的状态切换拆成一个独立的系统，说明它和 `player_move` 之间的依赖关系。
4. 试着在同一个 `System` 中读 `State` 和 `GameSession`，写出一个 bug，然后解释为什么它出现了。

## 一句话总结

真正的 Bevy 程序不是靠“炫技 API”取胜，而是靠把状态、时间和实体职责写清楚。`GameState` 决定阶段，`Query` 决定作用域，`FixedUpdate` 保证规则稳定，这三件事决定了后面所有逻辑能否维护。
