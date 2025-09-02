use std::collections::HashMap;

// 明確化のための型エイリアス
pub type Term = u64;
pub type LogIndex = u64;
pub type ServerId = u64;

/// Raftノードの状態。常にこのいずれかの状態にのみ存在する。
pub enum NodeState {
    Follower {
        term: Term,
        voted_for: Option<ServerId>,
    },
    Candidate {
        term: Term,
        votes_received: u64,
    },
    Leader {
        term: Term,
        // 各サーバーに対し、次に送信するログエントリのインデックス
        next_index: HashMap<ServerId, LogIndex>,
        // 各サーバーに対し、複製が成功したと判明しているログの最大インデックス
        match_index: HashMap<ServerId, LogIndex>,
    },
}

/// Raftログの単一エントリ
pub struct LogEntry {
    pub term: Term,
    pub command: Vec<u8>, // ステートマシンに適用されるコマンド
}

/// Raftノード間で送受信されるメッセージ
pub enum Message {
    RequestVote(RequestVoteArgs),
    RequestVoteResponse(RequestVoteResponseArgs),
    AppendEntries(AppendEntriesArgs),
    AppendEntriesResponse(AppendEntriesResponseArgs),
}

// `RequestVote` RPCの引数
pub struct RequestVoteArgs {
    pub term: Term,
    pub candidate_id: ServerId,
    pub last_log_index: LogIndex,
    pub last_log_term: Term,
}

// `RequestVote` RPCの応答
pub struct RequestVoteResponseArgs {
    pub term: Term,
    pub vote_granted: bool,
}

// `AppendEntries` RPCの引数
pub struct AppendEntriesArgs {
    pub term: Term,
    pub leader_id: ServerId,
    pub prev_log_index: LogIndex,
    pub prev_log_term: Term,
    pub entries: Vec<LogEntry>,
    pub leader_commit: LogIndex,
}

// `AppendEntries` RPCの応答
pub struct AppendEntriesResponseArgs {
    pub term: Term,
    pub success: bool,
}
