这里大概记录下性能分析的过程

# 1. 什么时候开始做的性能分析？目标是什么？

大概是ai coding 两周之后，`im2latex_formulas.lst`的测试通过率超过了98%。

但其实一开始的目标就是熟练掌握rust/wasm的profiling的过程。目标至少是rust wasm版的比katex.js要快。

# 2. profiling的起点是什么？

一个非常令人沮丧的结果。在少量的测试公式上，p95这个指标比katex.js慢5倍。
从四年前一开始到vibe coding介入一周内，rust代码都为了输出正确结果而努力，这个结果也算预料内。

profiling这块工作最开始的结果算是vibe coding帮我再demo中添加了**整体耗时的统计表和散点图**。
ai写代码确实快，我只是有了个想法统计耗时。ai立马让我看到了结果。（虽然这个慢5倍结果在我眼前飘了两周


# 3. 执行环境

从执行环境来说，可以分为两个方向，服务端和浏览器

## 服务端

katex.js可以运行在server端，将公式渲染为html文本。而katex-wasm或许可以在server端直接通过c接口输入公式文本返回html文本。这样就是直接对比了native的binary和nodejs执行的速度了。目前来说，katex-wasm也可以编译到nodejs的wasm。server可能不会直接操作dom tree？sorry，i am not profiver of web。

## 浏览器

katexjs可以直接操作dom tree，rust这边需要通过js-sys/web-sys的接口去操作dom，这一块没有听说wasm相比于js在直接操作dom tree的情况下有什么明显的优势。

另一方面，直接对比katex.js和katex-wasm在browser下 renderToString的性能。


# 4. browser下，renderToString的性能。



## 当前整体性能

在js中，相同renderSetting下，统计结果如下：

// todo

# 5. profiling 遇到的一些问题

## 首屏加载速度？代码预热


browser下还需要考虑的首屏加载速度，我们先不考虑这个问题，因为rust的第一个公式的渲染速度稍慢，大概率是因为wasm体积太大，加载速度的问题。

## wasm-pack的profile，todo

一开始，我还想着去自定一些cargo.toml的profile的配置。比如开启 `lto="fat"`之类的。但是wasm-pack有自己的想法。

`wasm-pack build --help`如下
```
...
      --debug                Deprecated. Renamed to `--dev`
      --dev                  Create a development build. Enable debug info, and disable optimizations
      --release              Create a release build. Enable optimizations and disable debug info
      --profiling            Create a profiling build. Enable optimizations and debug info
      --profile <PROFILE>    User-defined profile with --profile flag
...
```

我并没搞明白 `wasm-pack build --release` 和 `wasm-pack build --profile release` 的区别。
而且在使用`wasm-pack build --release`我遇到了`wasm-opt`工具的崩溃问题，所以我不得不关掉了它。
```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false

[package.metadata.wasm-pack.profile.profiling]
wasm-opt = false

```

## 测试数据的选择

上面结果的散点图横向是公式的长度，纵向是渲染耗时。算是个正相关的结果。

刚开始的测试数据是用来测试功能的支持度，所以涉及了很多比较短小的公式。当然现实情况也不会全是比较长的公式。所以散点图的左下角会比较密集。所以我紧接着统计了一些指标，定量化的对比分析。

测试数据是否应该选择带有error情况的公式？比如katex.ks不支持`\label`。


## regex在wasm的性能

作为rust，我不太相信rust的regex会跟其他语言的regex有什么差距。所以根据profiling的结果，我做了如下调整。