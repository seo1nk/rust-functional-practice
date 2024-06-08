# rust-functional-practice

状態遷移をなんかいい感じにRustで書けないか練習してみる

## memo

たとえばこんな感じのをいい感じに書けないかなと思っている

```mermaid
stateDiagram-v2
[*] --> base
base --> open
open --> accepted
open --> rejected
rejected --> reopen
reopen --> accepted
accepted --> close
rejected --> close
close --> [*]
```

一旦こんな感じだとする

- base something
  - id
  - name
- open something
  - base something
  - open id
  - open reason
- accepted something
  - base something
  - accepted id
  - accepted reason id
- close something
  - base something
  - close id
  - close datetime

