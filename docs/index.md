# Bevy 学习路线

「能运行一个窗口」只证明平台和依赖链暂时没有报错。学习 Bevy 的目标是把玩法压缩为可验证的状态、数据流与调度关系。

```{toctree}
:maxdepth: 1
:caption: 学习路径
:hidden:

bevy-learning-roadmap
roadmap
00-app-runtime/README
01-plugins-resources-systems/README
02-debug-visualization/README
03-ecs-data-model/README
04-query-access-conflicts/README
05-commands-lifecycle/README
06-events-change-detection/README
07-fixed-update/README
08-input-mapping/README
09-state-run-conditions/README
10-2d-coordinates/README
11-camera-follow/README
12-sprites-layers-z/README
13-ui-world-space/README
14-collision-minimum-model/README
15-event-driven-damage/README
16-rule-chain-composition/README
17-asset-server/README
18-animation-state/README
19-audio-ui-feedback/README
20-localization-config/README
21-plugin-project-structure/README
22-testing-ecs-logic/README
23-performance-profiling/README
24-save-settings-release/README
stages/stage-00-observability/README
stages/stage-01-spore-swarm/README
stages/stage-02-topdown-controller/README
stages/stage-03-tactical-map/README
stages/stage-04-arena-survival/README
stages/stage-05-content-presentation/README
stages/stage-06-vertical-slice/README
capstone
contributing
```

## 课程章节

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
		<a href="stages/stage-00-observability/README.html">阶段 0：可观测的运行时</a>
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
</div>

## 当前基线

- Rust stable 与 Cargo
- Bevy `0.19.1`
- 每个章节目录都是可独立运行的 Cargo crate
- 每个章节的 README 同时呈现 Bevy 机制与对应的 Rust 设计说明

阶段实验插入在其对应阶段的最后一篇文章之后。站点发布时会一并复制每个目录的 Cargo 源码，因此 README 中的 `src/main.rs` 链接可直接打开发布版本源码。