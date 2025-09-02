use std::fmt::Write;

//! ユーザー情報のリストをフォーマットする、いくつかのバージョンの関数。

/// 素朴な実装。
/// ループ内で毎回新しいStringを確保する。
pub fn format_user_list_naive(users: &[(u32, &str)]) -> Vec<String> {
    let mut formatted = Vec::new();
    for (id, name) in users {
        // `format!`は、ループの各イテレーションで新しいStringを確保する。
        formatted.push(format!("User {}: {}", id, name));
    }
    formatted
}

/// より慣用的で、コンパイラが最適化しやすい可能性がある実装。
/// 結果としては、素朴な実装とほぼ同じアロケーションを行う。
pub fn format_user_list_idiomatic(users: &[(u32, &str)]) -> Vec<String> {
    users
        .iter()
        .map(|(id, name)| format!("User {}: {}", id, name))
        .collect()
}

/// アロケーションを削減することで、真に最適化されたバージョン。
/// 全てのユーザー情報を、単一の大きなStringに書き込む。
pub fn format_user_list_single_string(users: &[(u32, &str)]) -> String {
    // おおよそのキャパシティを事前に計算し、再アロケーションを減らす。
    let capacity = users.len() * 25; // 平均的な行の長さを推定
    let mut output = String::with_capacity(capacity);

    for (id, name) in users {
        // `write!`マクロは、既存のStringに書き込むため、
        // （キャパシティを超えない限り）新しいアロケーションを行わない。
        let _ = writeln!(output, "User {}: {}", id, name);
    }
    output
}
