# netease_music_lyric_downloder_rs
网易云音乐歌词下载器(rust实现)
----
## ⚙ 编译
```shell
cargo build --release
```
编译产物在`target/release/nml`下， 将其移动到`$PATH`变量指定位置，比如`/usr/bin`
自动补全脚本构建完成后存放在`completions`下
```shell
总计 12K
# zsh补全
-rw-r--r-- 1 sakunia sakunia  769 11月18日 21:00 _nml
# bash补全
-rw-r--r-- 1 sakunia sakunia 1.2K 11月18日 21:00 nml.bash
# fish补全
-rw-r--r-- 1 sakunia sakunia  181 11月18日 21:00 nml.fish
```


## 📚 使用
移动至`$PATH`下后，运行
```shell
nml -h
# 或者
nml --help
```
获取帮助
运行
```shell
# nml <关键字>， 例:
nml 好日子
```
默认保存至当前目录下的`lyrics`文件夹
运行一下命令可以更改存储位置
```shell
# nml -s | --save-path <存储路径> <关键字>
nml -s ./tmpl 好日子
nml --save-path 好日子
```
----
## 💳 License

MIT license ([LICENSE](./LICENSE) or https://opensource.org/licenses/MIT)