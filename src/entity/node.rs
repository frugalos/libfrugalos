//! ノード関連のエンティティ定義。
use std::net::SocketAddr;

// FIXME: 構造体にする
/// プロセスローカルでユニークなノードのID。
pub type LocalNodeId = String;

// FIXME: 構造体にする
/// リモートプロセス上のノードのID。
pub type RemoteNodeId = (SocketAddr, String);
