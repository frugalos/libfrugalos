//! サーバ関連のエンティティ定義。
use std::net::{IpAddr, SocketAddr};

// FIXME: 構造体にする
/// サーバのID。
pub type ServerId = String;

/// サーバの要約情報。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSummary {
    /// ID。
    pub id: ServerId,
}

/// サーバ。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    /// ID。
    pub id: ServerId,

    /// シーケンス番号。
    ///
    /// 登録時に自動で採番される。
    #[serde(default)]
    pub seqno: u32,

    /// ホスト情報。
    pub host: IpAddr,

    /// ポート番号。
    pub port: u16,
}
impl Server {
    /// 新しい`Server`インスタンスを生成する。
    pub fn new(id: ServerId, addr: SocketAddr) -> Self {
        Server {
            id,
            seqno: 0,
            host: addr.ip(),
            port: addr.port(),
        }
    }

    /// 要約情報を返す。
    pub fn to_summary(&self) -> ServerSummary {
        ServerSummary {
            id: self.id.clone(),
        }
    }

    /// サーバと通信するためのアドレスを返す。
    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}
// FIXME: デフォルト実装は無くす（今はserdeのために必要）
impl Default for Server {
    fn default() -> Self {
        Server {
            id: String::new(),
            seqno: 0,
            host: From::from([0, 0, 0, 0]),
            port: 0,
        }
    }
}
