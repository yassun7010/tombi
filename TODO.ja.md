## TODO
### Milestone 2
- [ ] Document のサポート。
- [ ] JSON Schema のサポート

### Milestone 3
- [ ] Linter の機能強化
- [ ] Formatter のオプションサポート

### Bugs
- [x] Local Date 型が誤って IntegerDec としてパースされる
- [ ] Keys に float や int を使った場合、誤ってパースされる
    - [ ] 3.14 を keys に使った場合、3 と 14 の key としてパースされる
    - [ ] 3 を keys に使った場合、3 の key としてパースされる
    - [ ] inf, nan を keys に使った場合、key としてパースされる
- [ ] Array
    - [ ] 複数行で最後にカンマがない場合、カンマを差し込む位置でコメントを考慮する
    - [ ] Array のカンマと要素の末尾コメントの関係を見て、カンマの位置を移動
- [ ] Inline Table
    - [ ] 現行の v1.0.0 では複数行の Inline Table がサポートされていないのでエラーを出力させる。

### Refactor
- [ ] 各crateのエラー型の整理
- [ ] parser のスリム化
