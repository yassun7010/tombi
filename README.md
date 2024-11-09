![Logo](./docs/images/tombi.svg)

Tombi (鳶) is a toolkit for TOML; providing a formatter/linter and LSP server.

## TODO
### Features
- [x] コメントの完全なサポート
    - [x] トップブロックのコメントサポート
    - [x] Array のコメントサポート
    - [x] Inline Table のコメントサポート
- [ ] diagnostics のエラーメッセージの範囲の改善
- [ ] Document のサポート。
- [ ] linter のサポート
- [ ] リリース
- [ ] JSON Schema のサポート
- [ ] syntax tree 側での行・列情報のサポート

### Bugs
- [ ] Local Date 型が誤って IntegerDec としてパースされる
- [ ] Keys に float や int を使った場合、誤ってパースされる
    - [ ] 3.14 を keys に使った場合、3 と 14 の key としてパースされる
    - [ ] 3 を keys に使った場合、3 の key としてパースされる
    - [ ] inf, nan を keys に使った場合、key としてパースされる
