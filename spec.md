# Carcassonne AI

## 概要
Carcassonne AI の実装

## ユーザー登録
ユーザーに登録してもらう
登録なしでも遊べるようにするかは未定だが、勝率等保存できるのがよさそうだし情報も見たい
Twitter & Google があれば OK ？

Player
- id
- email
- name
- twitter id?
- created_at
- is_ai

AI も Player とみなしてよい

## Game の作成
ゲーム情報を持つ Game を作成して試合を開始する

Game
- id
- player0_id
- player1_id
- player0_point
- player1_point
- created_at
- ended_at

## Game の進行
盤面は move の列で表現
move には tile move と meeple move がある
tile move は { move 番号, タイルの種類, 回転回数, タイルを置く位置 (y, x) }
meeple move は { move 番号, ミープル番号?, 直前のタイルのミープルを置く位置 }
初期盤面は [{ 0, アンダーバー, 0, (0, 0) }, { 1 }] で、ストレートを右につけて道にミープル 0 を置いたときの盤面は [{ 0, アンダーバー, 0, (0, 0) }, { 1 }, { 2, ストレート, 1, (0, 1)}, { 3, 0, 1 }] などと表現される

自分のターンでは API から置くべきタイルが返ってくるのでそのタイルを含む tile move を投げる。配置可能なミープル位置が返ってくるのでそれに応じて meeple move を投げる。

meeple move を投げると events が返ってくるのでそれに応じて描画等する。
events は event { プレイヤーの id, 回収されるミープルの番号, それによって得る点数 } の配列？
それを適当に描画して次のターンを行う

街・道・草原の計算は全て backend のみで行う。適当に id をつけてそれらの disjoint set を管理する

Tile Move
- tile_id
- turn
- pos (y, x)

Meeple Move
- meeple_id
- pos

## Game の終了
最終計算を event と似たような形で行い終了する
勝率など計算できるようにしたいが、バージョンによって変わるので適当な名前で区別できるようにしておく？

AI
- id 
- name
- created_at
