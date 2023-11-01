## spec

```
subcommand: 
  sample-commands:
    regular:
      cmd: "echo hello world" # regular form
    with-args:
      cmd: "echo arg1 is {0}, arg2 is {1}"
    abbreviated: "echo this is abbreviated form" # abbreviated form

  command-with-input-complement:
    cmd: "echo arg1 is {0}, arg2 is {1}"
    0: "echo apple orange lemon" # command for input completion

another-command: "cargo --version" # abbreviated form
```

出力フォーマット

## 名称
YACT (YAML as Command Tree)
YAAC (YAML as Alias Commands)
YAAC (Yet Another Alias Commands)
YAAH (YAML as Alias Hierarchy)


## TODO
プレースホルダがあるけど補完関数が設定されていないときはシェルを模倣するために補間関数に特別な文字列を渡す

### note

rustの開発を手伝ってくれ。
あるプロジェクトの中で、YAMLの本文（serde_yaml::Value）とパスを指定が指定され、パスに存在する値と結果判定を返す関数を作っている。
以下のようなYAMLを想定している。

commands.yaml
```
# step1
subcommand: 
  sample-commands:
    regular:
      cmd: "echo hello world" # regular form
    with-args:
      cmd: "echo arg1 is {0}, arg2 is {1}"
    abbreviated: "echo \"this is abbreviated form\""
    abbreviated-with-args: "echo \"this is abbreviated form (args are {0} and {1})\""

  command-with-input-copmletion:
    cmd: "echo arg1 is {0}, arg2 is {1} and last is {2}"
    0: "echo apple orange lemon" # command for input completion
    1: "echo banana grape"
```

cmd というのは特殊なキーで、以下の2つが等価になることを注意してほしい。
```
some-command: "your command"
```
```
some-command:
  cmd: "your command"
```

以下に示すコードをリファクタしてほしい。
現在は前半でコマンドを特定しているが、ここではコマンドそのものではなく、YAML要素を返すようにして。
つまり、
subcommand, sample-commands, regular というパラメータなら、 regular までを前半で特定する。

後半では regular 要素から、引数（マッチで使われなかったkeys）をプレースホルダにインジェクトして実際のコマンドを返す。


## note

### 現在入力中の引数はスクリプトには渡されないのでオプション系はどうやって処理しよう問題

#### 案
- 入力中の引数もシェルから渡す案
  - 単純にそれをすると、途中まで文字を入れてるやつの背中を押す補完ができない
  - 本質的に入力完了しているのか途中なのかの区別がつかない。区別するようにするならもっと良い方法があるし、「ハイフン付きならシェルから渡す案」のほうがマシ

- ハイフン付きならシェルから渡す案
  - 設計的に妥協
  - これもやっぱり入力中なのか既に入力しているのか区別つかないので、「--add」すると次も「--add」が補完される罠がある。ないと思うが「-」しても次の引数がオプション補完される

- Priority5 の空文字列を上書きする案
  - Priority3でサブコマンドの補完があるので、そこに「-」を入れても何も補完されない問題がある

- 全部シェルで制御する案（最終手段？）
  - シェルにコード書きたくない・・・
  - まあでもデフォルト補完もどうせあるしいいのか・・？
  - copyを出すべきか（コマンドがあるか）はわからないけど、pathの有無は戻り値を工夫して頑張る方法はある
    - うーん、いやもうmessageに入れちゃうか。どうせ使い道無かったし。もしくは result に突っ込んで、シェル側でオプションを適切に除外するという方法

