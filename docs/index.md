# Bevy 学习路线

「能运行一个窗口」只证明平台和依赖链暂时没有报错。学习 Bevy 的目标是把玩法压缩为可验证的状态、数据流与调度关系。

```{toctree}
:maxdepth: 1
:caption: 学习路径
:hidden:

bevy-learning-roadmap
00-app-runtime/README
01-plugins-resources-systems/README
02-debug-visualization/README
stages/stage-00-observability/README
03-ecs-data-model/README
04-query-access-conflicts/README
05-commands-lifecycle/README
06-events-change-detection/README
stages/stage-01-spore-swarm/README
07-fixed-update/README
08-input-mapping/README
09-state-run-conditions/README
stages/stage-02-topdown-controller/README
10-2d-coordinates/README
11-camera-follow/README
12-sprites-layers-z/README
13-ui-world-space/README
stages/stage-03-tactical-map/README
14-collision-minimum-model/README
15-event-driven-damage/README
16-rule-chain-composition/README
stages/stage-04-arena-survival/README
17-asset-server/README
18-animation-state/README
19-audio-ui-feedback/README
20-localization-config/README
stages/stage-05-content-presentation/README
21-plugin-project-structure/README
22-testing-ecs-logic/README
23-performance-profiling/README
24-save-settings-release/README
stages/stage-06-vertical-slice/README
capstone
```

```{rubric} 课程章节
```

<div class="chapter-link-grid">
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 0</span>
		<a href="00-app-runtime/README.html">教程 00：App 运行时</a>
		<a class="chapter-link-rust" href="00-app-runtime/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 0</span>
		<a href="01-plugins-resources-systems/README.html">教程 01：插件、资源与系统</a>
		<a class="chapter-link-rust" href="01-plugins-resources-systems/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 0</span>
		<a href="02-debug-visualization/README.html">教程 02：调试可视化</a>
		<a class="chapter-link-rust" href="02-debug-visualization/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row chapter-link-stage">
		<span class="chapter-link-phase">阶段实践</span>
		<a href="stages/stage-00-observability/README.html">阶段 00：可观测的运行时</a>
		<a class="chapter-link-rust" href="stages/stage-00-observability/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 1</span>
		<a href="03-ecs-data-model/README.html">教程 03：ECS 数据模型</a>
		<a class="chapter-link-rust" href="03-ecs-data-model/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 1</span>
		<a href="04-query-access-conflicts/README.html">教程 04：Query 访问冲突</a>
		<a class="chapter-link-rust" href="04-query-access-conflicts/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 1</span>
		<a href="05-commands-lifecycle/README.html">教程 05：Commands 与生命周期</a>
		<a class="chapter-link-rust" href="05-commands-lifecycle/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 1</span>
		<a href="06-events-change-detection/README.html">教程 06：事件与变更检测</a>
		<a class="chapter-link-rust" href="06-events-change-detection/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row chapter-link-stage">
		<span class="chapter-link-phase">阶段实践</span>
		<a href="stages/stage-01-spore-swarm/README.html">阶段 01：孢子群</a>
		<a class="chapter-link-rust" href="stages/stage-01-spore-swarm/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 2</span>
		<a href="07-fixed-update/README.html">教程 07：Fixed Update 与时间步</a>
		<a class="chapter-link-rust" href="07-fixed-update/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 2</span>
		<a href="08-input-mapping/README.html">教程 08：输入映射</a>
		<a class="chapter-link-rust" href="08-input-mapping/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 2</span>
		<a href="09-state-run-conditions/README.html">教程 09：State 与 Run Condition</a>
		<a class="chapter-link-rust" href="09-state-run-conditions/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row chapter-link-stage">
		<span class="chapter-link-phase">阶段实践</span>
		<a href="stages/stage-02-topdown-controller/README.html">阶段 02：俯视角控制器</a>
		<a class="chapter-link-rust" href="stages/stage-02-topdown-controller/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 3</span>
		<a href="10-2d-coordinates/README.html">教程 10：2D 坐标系</a>
		<a class="chapter-link-rust" href="10-2d-coordinates/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 3</span>
		<a href="11-camera-follow/README.html">教程 11：相机跟随</a>
		<a class="chapter-link-rust" href="11-camera-follow/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 3</span>
		<a href="12-sprites-layers-z/README.html">教程 12：精灵、图层与 Z</a>
		<a class="chapter-link-rust" href="12-sprites-layers-z/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 3</span>
		<a href="13-ui-world-space/README.html">教程 13：UI 与世界空间</a>
		<a class="chapter-link-rust" href="13-ui-world-space/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row chapter-link-stage">
		<span class="chapter-link-phase">阶段实践</span>
		<a href="stages/stage-03-tactical-map/README.html">阶段 03：战术地图</a>
		<a class="chapter-link-rust" href="stages/stage-03-tactical-map/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 4</span>
		<a href="14-collision-minimum-model/README.html">教程 14：碰撞最小模型</a>
		<a class="chapter-link-rust" href="14-collision-minimum-model/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 4</span>
		<a href="15-event-driven-damage/README.html">教程 15：事件驱动伤害</a>
		<a class="chapter-link-rust" href="15-event-driven-damage/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 4</span>
		<a href="16-rule-chain-composition/README.html">教程 16：规则链组成</a>
		<a class="chapter-link-rust" href="16-rule-chain-composition/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row chapter-link-stage">
		<span class="chapter-link-phase">阶段实践</span>
		<a href="stages/stage-04-arena-survival/README.html">阶段 04：Arena 生存</a>
		<a class="chapter-link-rust" href="stages/stage-04-arena-survival/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 5</span>
		<a href="17-asset-server/README.html">教程 17：Asset Server</a>
		<a class="chapter-link-rust" href="17-asset-server/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 5</span>
		<a href="18-animation-state/README.html">教程 18：动画状态</a>
		<a class="chapter-link-rust" href="18-animation-state/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 5</span>
		<a href="19-audio-ui-feedback/README.html">教程 19：音频与 UI 反馈</a>
		<a class="chapter-link-rust" href="19-audio-ui-feedback/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 5</span>
		<a href="20-localization-config/README.html">教程 20：本地化与配置</a>
		<a class="chapter-link-rust" href="20-localization-config/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row chapter-link-stage">
		<span class="chapter-link-phase">阶段实践</span>
		<a href="stages/stage-05-content-presentation/README.html">阶段 05：内容呈现</a>
		<a class="chapter-link-rust" href="stages/stage-05-content-presentation/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 6</span>
		<a href="21-plugin-project-structure/README.html">教程 21：插件化项目结构</a>
		<a class="chapter-link-rust" href="21-plugin-project-structure/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 6</span>
		<a href="22-testing-ecs-logic/README.html">教程 22：测试 ECS 逻辑</a>
		<a class="chapter-link-rust" href="22-testing-ecs-logic/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 6</span>
		<a href="23-performance-profiling/README.html">教程 23：性能剖析</a>
		<a class="chapter-link-rust" href="23-performance-profiling/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row">
		<span class="chapter-link-phase">阶段 6</span>
		<a href="24-save-settings-release/README.html">教程 24：存档、设置与发布</a>
		<a class="chapter-link-rust" href="24-save-settings-release/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
	<div class="chapter-link-row chapter-link-stage">
		<span class="chapter-link-phase">阶段实践</span>
		<a href="stages/stage-06-vertical-slice/README.html">阶段 06：垂直切片</a>
		<a class="chapter-link-rust" href="stages/stage-06-vertical-slice/README.html#rust-in-bevy">Rust in Bevy</a>
	</div>
</div>

```{rubric} 当前基线
```

- Rust stable 与 Cargo
- Bevy `0.19.1`
- 每个章节目录都是可独立运行的 Cargo crate
- 每个章节的 README 同时呈现 Bevy 机制与对应的 Rust 设计说明

阶段实验插入在其对应阶段的最后一篇文章之后。站点发布时会一并复制每个目录的 Cargo 源码，因此 README 中的 `src/main.rs` 链接可直接打开发布版本源码。