//! RPCのスキーマ定義。
//!
//! # Compatibility Guideline
//!
//! ## 基本方針
//!
//! * Protocol Buffers のバージョンは 3 を利用する。
//! * 後方互換性を損わないために Protocol Buffers のタグは絶対に再利用しない。
//! * 互換性が崩れる時は `ProcedureId` を変更し、新しく RPC を定義し直す(既存の RPC を消さない)。
//!     * 無停止でのバージョン切り替えなど一時的に互換性のないバージョンが共存できる状態にするため。
//! * RPC のリクエスト/レスポンス型は専用の `struct` を極力利用する。
//!     * 後方互換性を維持したままフィールドの追加/変更ができるようにするため。
//!
//! 互換性を崩す時に他の方針として、新旧で 2 つのフィールドを用意し双方にデータを入れて返すというやり方もあるが、
//! ネットワーク帯域がシビアなユースケースを想定しているためこの方針は採用しない。
//!
//! ## Protocol Buffers の下位互換性のある変更
//!
//! * リクエスト形式に新しくフィールドを追加する
//! * レスポンス形式に新しくフィールドを追加する
//! * Protocol Buffers で認められている型の変更をする
//! * 単一の値を取るフィールドを新しく追加した(新しいというのが重要) `oneof` に追加する
//! * 既存の `oneof` に新しくフィールドを追加する
//!     * ただし、新しく追加したフィールドを古いフォーマットしか知らないクライアントに返してはいけない。
//!
//! [https://developers.google.com/protocol-buffers/docs/proto3#updating](https://developers.google.com/protocol-buffers/docs/proto3#updating) をよく読み十分理解すること。
//!
//! ## Protocol Buffers の下位互換性のない変更
//!
//! * スカラ型(uint64 など)から `message` へ型を変更をする
//! * `EmptyMessageDecoder`/`EmptyMessageEncoder` から別の `Encoder`/`Decoder` への変更をする
//! * リクエスト形式自体(`String` から `struct Foo` など)の変更をする
//! * レスポンス形式自体(`()` から `Result<Option<String>>` など)の変更をする
pub mod config;
pub mod frugalos;
pub mod mds;
