# postflop-solver

> [!IMPORTANT]
> **As of October 2023, I have started developing a poker solver as a business and have decided to suspend development of this open-source project. See [this issue] for more information.**

[this issue]: https://github.com/b-inary/postflop-solver/issues/46

---

Rust で書かれたオープンソースのポストフロップ ソルバー ライブラリ

ドキュメント: https://b-inary.github.io/postflat_solver/postflat_solver/

**関連リポジトリ**
- Web アプリ (WASM ポストフロップ): https://github.com/b-inary/wasm-postflat
- デスクトップ アプリ (デスクトップ ポストフロップ): https://github.com/b-inary/desktop-postflat

**注:**
このライブラリの主な目的は、GUI アプリケーション ([WASM ポストフロップ] および [デスクトップ ポストフロップ]) のバックエンド エンジンとして機能することです。
ユーザー/開発者によるこのライブラリの直接使用は、設計上、重要な目的ではありません。
そのため、バージョンの変更なしで重大な変更が行われることがよくあります。
重大な変更の詳細については、[CHANGES.md](CHANGES.md) を参照してください。
[WASM Postflop]: https://github.com/b-inary/wasm-postflop
[Desktop Postflop]: https://github.com/b-inary/desktop-postflop

## 使用方法

- `Cargo.toml`

```toml
[dependencies]
postflop-solver = { git = "https://github.com/b-inary/postflop-solver" }
```

- 例

例は [examples](examples) ディレクトリにあります。

このリポジトリをクローンした場合は、次のコマンドで例を実行できます。

```sh
$ cargo run --release --example basic
```

## Implementation details

- **Algorithm**: ソルバーは最先端の [Discounted CFR] アルゴリズムを使用します。
  現在、γ の値は、元の論文で推奨されている 2.0 ではなく 3.0 に設定されています。
  また、ソルバーは、反復回数が 4 の累乗になると累積戦略をリセットします。
- **Performance**: ソルバー エンジンは、保守可能なコードでパフォーマンスが最適化されています。
  エンジンはデフォルトでマルチスレッドをサポートし、ホット スポットで安全でない Rust を最大限に活用します。
  開発者はコンパイラからのアセンブリ出力を確認し、SIMD 命令が可能な限り使用されるようにします。
  上記のアルゴリズムと組み合わせると、パフォーマンスは PioSOLVER や GTO+ などの有料ソルバーを上回ります。
- **同型写像(抽象化)**: ソルバーは抽象化を一切行いません。
  ただし、同型チャンス (ターンとリバーのディール) は 1 つにまとめられます。
  たとえば、フロップが単調な場合、ディールされていない 3 つのスーツは同型であるため、3 つのスーツのうち 2 つの計算をスキップできます。
- **精度**: ほとんどの場所で 32 ビット浮動小数点数が使用されます。
  合計を計算する場合、一時的な値には 64 ビット浮動小数点数が使用されます。
  各ゲーム ノードが単一の 32 ビット浮動小数点スケーリング係数を使用して 16 ビット整数で値を格納する圧縮オプションもあります。
- **バンチング効果**: 執筆時点では、これがバンチング効果を処理できる唯一の実装です。
  最大 4 人のフォールド プレイヤー (6 人制ゲーム) をサポートします。
  この実装はカードの組み合わせの数を正しくカウントし、デッキの確率分布を操作するなどのヒューリスティックに依存しません。
  ただし、バンチング効果を有効にすると、ターミナル ノードでの評価の時間計算量が増加し、計算が大幅に遅くなることに注意してください。

[Discounted CFR]: https://arxiv.org/abs/1809.04040

## Crate features

- `bincode`: [bincode] クレート (2.0.0-rc.3) を使用して、`PostFlopGame` 構造体をシリアル化およびデシリアル化します。
  この機能は、ゲームツリーを保存およびロードするために必要です。  
  デフォルトで有効になっています。
- `custom-alloc`: 解決プロセスでカスタム メモリ アロケータを使用します (ナイトリー Rust でのみ使用可能)。
  デフォルトのアロケータの呼び出し回数が大幅に削減されるため、デフォルトのアロケータがそれほど効率的でない場合にこの機能を使用することをお勧めします。
  この機能は、プログラムで解決するときに、最大で 1 つの `PostFlopGame` インスタンスのみが使用可能であると想定していることに注意してください。
  デフォルトでは無効になっています。
- `rayon`: 並列化に [rayon] クレートを使用します。
  デフォルトで有効になっています。
- `zstd`: [zstd] クレートを使用して、ゲームツリーを圧縮および解凍します。
  この機能は、ゲームツリーを圧縮して保存およびロードするために必要です。
  デフォルトでは無効になっています。

[bincode]: https://github.com/bincode-org/bincode
[rayon]: https://github.com/rayon-rs/rayon
[zstd]: https://github.com/gyscos/zstd-rs

## License

Copyright (C) 2022 Wataru Inariba

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
