use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct ProjectRequirements {
    performance_critical: bool,
    memory_constraints: bool,
    team_experience: HashMap<String, i32>,  // 言語 → 経験年数
    development_timeline: i32,              // 月数
    maintenance_period: i32,                // 年数
    concurrent_users: i32,
    data_volume_gb: i32,
}

#[derive(Debug)]
struct LanguageScore {
    language: String,
    performance_score: f64,
    development_speed_score: f64,
    maintenance_score: f64,
    risk_score: f64,
    total_score: f64,
}

struct TechDecisionFramework;

impl TechDecisionFramework {
    fn evaluate_rust(req: &ProjectRequirements) -> LanguageScore {
        let performance_score = if req.performance_critical {
            9.0
        } else {
            7.0
        };
        
        let development_speed_score = match req.team_experience.get("rust").unwrap_or(&0) {
            0 => 3.0,  // 学習コストが高い
            1..=2 => 6.0,
            _ => 8.0,
        };
        
        let maintenance_score = if req.maintenance_period > 5 {
            9.0  // 型安全性が長期的に有利
        } else {
            7.0
        };
        
        let risk_score = if req.development_timeline < 6 {
            4.0  // 短期開発にはリスク
        } else {
            8.0
        };
        
        let total_score = (performance_score + development_speed_score + 
                          maintenance_score + risk_score) / 4.0;
        
        LanguageScore {
            language: "Rust".to_string(),
            performance_score,
            development_speed_score,
            maintenance_score,
            risk_score,
            total_score,
        }
    }
    
    fn evaluate_java(req: &ProjectRequirements) -> LanguageScore {
        let performance_score = if req.performance_critical && req.concurrent_users > 10000 {
            6.0  // GCがボトルネックになる可能性
        } else {
            8.0
        };
        
        let development_speed_score = match req.team_experience.get("java").unwrap_or(&0) {
            0 => 5.0,
            1..=2 => 8.0,
            _ => 9.0,
        };
        
        let maintenance_score = 8.0;  // エコシステムが豊富
        
        let risk_score = 9.0;  // 安定した技術
        
        let total_score = (performance_score + development_speed_score + 
                          maintenance_score + risk_score) / 4.0;
        
        LanguageScore {
            language: "Java".to_string(),
            performance_score,
            development_speed_score,
            maintenance_score,
            risk_score,
            total_score,
        }
    }
    
    fn evaluate_python(req: &ProjectRequirements) -> LanguageScore {
        let performance_score = if req.performance_critical {
            4.0  // 性能がボトルネック
        } else {
            7.0
        };
        
        let development_speed_score = 9.0;  // 開発速度は高速
        
        let maintenance_score = if req.data_volume_gb > 100 {
            5.0  // 大規模データで問題が出やすい
        } else {
            7.0
        };
        
        let risk_score = 8.0;
        
        let total_score = (performance_score + development_speed_score + 
                          maintenance_score + risk_score) / 4.0;
        
        LanguageScore {
            language: "Python".to_string(),
            performance_score,
            development_speed_score,
            maintenance_score,
            risk_score,
            total_score,
        }
    }
    
    fn evaluate_cpp(req: &ProjectRequirements) -> LanguageScore {
        let performance_score = 9.5;  // 非常に高性能
        
        let development_speed_score = match req.team_experience.get("cpp").unwrap_or(&0) {
            0 => 2.0,  // 学習曲線が急
            1..=2 => 5.0,
            _ => 7.0,
        };
        
        let maintenance_score = if req.maintenance_period > 5 {
            5.0  // 複雑さが長期的にコスト
        } else {
            7.0
        };
        
        let risk_score = if req.team_experience.get("cpp").unwrap_or(&0) < &3 {
            3.0  // メモリ管理のリスク
        } else {
            7.0
        };
        
        let total_score = (performance_score + development_speed_score + 
                          maintenance_score + risk_score) / 4.0;
        
        LanguageScore {
            language: "C++".to_string(),
            performance_score,
            development_speed_score,
            maintenance_score,
            risk_score,
            total_score,
        }
    }
    
    fn evaluate_go(req: &ProjectRequirements) -> LanguageScore {
        let performance_score = if req.performance_critical && req.memory_constraints {
            6.5  // GCがある
        } else {
            8.0
        };
        
        let development_speed_score = match req.team_experience.get("go").unwrap_or(&0) {
            0 => 6.0,  // 比較的学習しやすい
            1..=2 => 8.0,
            _ => 9.0,
        };
        
        let maintenance_score = 8.0;  // シンプルな言語設計
        
        let risk_score = 8.0;
        
        let total_score = (performance_score + development_speed_score + 
                          maintenance_score + risk_score) / 4.0;
        
        LanguageScore {
            language: "Go".to_string(),
            performance_score,
            development_speed_score,
            maintenance_score,
            risk_score,
            total_score,
        }
    }
    
    pub fn recommend_language(req: &ProjectRequirements) -> Vec<LanguageScore> {
        let mut scores = vec![
            Self::evaluate_rust(req),
            Self::evaluate_java(req),
            Self::evaluate_python(req),
            Self::evaluate_cpp(req),
            Self::evaluate_go(req),
        ];
        
        scores.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());
        scores
    }
}

fn print_recommendations(scenario: &str, recommendations: &Vec<LanguageScore>) {
    println!("\n{}", "=".repeat(60));
    println!("Scenario: {}", scenario);
    println!("{}", "=".repeat(60));
    
    for (i, score) in recommendations.iter().enumerate() {
        println!("{}. {} (Score: {:.1})", i+1, score.language, score.total_score);
        println!("   Performance: {:.1}, Dev Speed: {:.1}, Maintenance: {:.1}, Risk: {:.1}",
            score.performance_score, score.development_speed_score, 
            score.maintenance_score, score.risk_score);
    }
}

fn main() {
    println!("=== Tech Decision Framework Demo ===");
    
    // シナリオ1: 高性能Webサービス
    let high_performance_web = ProjectRequirements {
        performance_critical: true,
        memory_constraints: true,
        team_experience: {
            let mut exp = HashMap::new();
            exp.insert("java".to_string(), 3);
            exp.insert("python".to_string(), 2);
            exp.insert("rust".to_string(), 1);
            exp.insert("cpp".to_string(), 0);
            exp.insert("go".to_string(), 2);
            exp
        },
        development_timeline: 8,
        maintenance_period: 10,
        concurrent_users: 50000,
        data_volume_gb: 500,
    };
    
    let recommendations = TechDecisionFramework::recommend_language(&high_performance_web);
    print_recommendations("High-performance web service", &recommendations);
    
    // シナリオ2: プロトタイプ開発
    let prototype_project = ProjectRequirements {
        performance_critical: false,
        memory_constraints: false,
        team_experience: {
            let mut exp = HashMap::new();
            exp.insert("python".to_string(), 3);
            exp.insert("java".to_string(), 1);
            exp.insert("rust".to_string(), 0);
            exp.insert("cpp".to_string(), 0);
            exp.insert("go".to_string(), 1);
            exp
        },
        development_timeline: 2,
        maintenance_period: 1,
        concurrent_users: 100,
        data_volume_gb: 10,
    };
    
    let recommendations = TechDecisionFramework::recommend_language(&prototype_project);
    print_recommendations("Rapid prototype development", &recommendations);
    
    // シナリオ3: 組み込みシステム
    let embedded_system = ProjectRequirements {
        performance_critical: true,
        memory_constraints: true,
        team_experience: {
            let mut exp = HashMap::new();
            exp.insert("c".to_string(), 5);
            exp.insert("cpp".to_string(), 3);
            exp.insert("rust".to_string(), 1);
            exp.insert("java".to_string(), 0);
            exp.insert("python".to_string(), 0);
            exp
        },
        development_timeline: 12,
        maintenance_period: 15,
        concurrent_users: 0,
        data_volume_gb: 1,
    };
    
    let recommendations = TechDecisionFramework::recommend_language(&embedded_system);
    print_recommendations("Embedded system development", &recommendations);
}