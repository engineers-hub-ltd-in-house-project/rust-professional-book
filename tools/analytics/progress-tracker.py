#!/usr/bin/env python3
"""
執筆進捗追跡ツール
章別の進捗状況を分析・レポート
"""

import os
import sys
import yaml
import json
from pathlib import Path
from datetime import datetime
from collections import defaultdict

class ProgressTracker:
    def __init__(self, project_root):
        self.project_root = Path(project_root)
        self.manuscript_dir = self.project_root / 'manuscript'
        self.chapters_dir = self.manuscript_dir / 'chapters'
        self.stats = defaultdict(dict)
    
    def analyze_chapter(self, chapter_dir):
        """章の進捗を分析"""
        chapter_name = chapter_dir.name
        stats = {}
        
        # メタデータを読み込み
        metadata_file = chapter_dir / 'metadata.yaml'
        if metadata_file.exists():
            with open(metadata_file, 'r', encoding='utf-8') as f:
                metadata = yaml.safe_load(f)
                stats['metadata'] = metadata
        
        # 原稿ファイルを分析
        chapter_file = chapter_dir / 'chapter.md'
        if chapter_file.exists():
            with open(chapter_file, 'r', encoding='utf-8') as f:
                content = f.read()
                stats['word_count'] = len(content)
                stats['char_count'] = len(content.replace(' ', '').replace('\n', ''))
                stats['line_count'] = content.count('\n')
                stats['code_blocks'] = content.count('```')
        
        # ハンズオンの数をカウント
        exercises_dir = chapter_dir / 'exercises'
        if exercises_dir.exists():
            stats['exercises_count'] = len(list(exercises_dir.glob('*.md')))
        else:
            stats['exercises_count'] = 0
        
        # コード例の数をカウント
        code_dir = self.project_root / 'code-examples' / chapter_name.replace('-', '_')
        if code_dir.exists():
            stats['code_examples'] = len(list(code_dir.glob('**/Cargo.toml')))
        else:
            stats['code_examples'] = 0
        
        return stats
    
    def calculate_progress(self, stats):
        """進捗率を計算"""
        if 'metadata' not in stats:
            return 0
        
        metadata = stats['metadata']
        target_pages = metadata.get('pages', 50)
        target_exercises = metadata.get('exercises', 5)
        
        # 1ページあたり約400文字として計算
        chars_per_page = 400
        target_chars = target_pages * chars_per_page
        
        current_chars = stats.get('char_count', 0)
        current_exercises = stats.get('exercises_count', 0)
        
        # 進捗率を計算（文字数70%、演習30%の重み付け）
        text_progress = min(100, (current_chars / target_chars * 100)) if target_chars > 0 else 0
        exercise_progress = min(100, (current_exercises / target_exercises * 100)) if target_exercises > 0 else 0
        
        total_progress = text_progress * 0.7 + exercise_progress * 0.3
        
        return round(total_progress, 1)
    
    def generate_report(self):
        """進捗レポートを生成"""
        report = {
            'generated_at': datetime.now().isoformat(),
            'chapters': {},
            'summary': {
                'total_words': 0,
                'total_chars': 0,
                'total_exercises': 0,
                'total_code_examples': 0,
                'average_progress': 0
            }
        }
        
        progress_values = []
        
        for chapter_dir in sorted(self.chapters_dir.iterdir()):
            if chapter_dir.is_dir():
                chapter_stats = self.analyze_chapter(chapter_dir)
                progress = self.calculate_progress(chapter_stats)
                
                chapter_stats['progress_percentage'] = progress
                report['chapters'][chapter_dir.name] = chapter_stats
                
                # サマリーを更新
                report['summary']['total_words'] += chapter_stats.get('word_count', 0)
                report['summary']['total_chars'] += chapter_stats.get('char_count', 0)
                report['summary']['total_exercises'] += chapter_stats.get('exercises_count', 0)
                report['summary']['total_code_examples'] += chapter_stats.get('code_examples', 0)
                progress_values.append(progress)
        
        if progress_values:
            report['summary']['average_progress'] = round(sum(progress_values) / len(progress_values), 1)
        
        return report
    
    def print_report(self, report):
        """レポートをコンソールに出力"""
        print("\n" + "="*60)
        print("📊 執筆進捗レポート")
        print("="*60)
        print(f"生成日時: {report['generated_at']}")
        print()
        
        # 章別進捗
        print("📚 章別進捗:")
        for chapter_name, stats in report['chapters'].items():
            progress = stats.get('progress_percentage', 0)
            status = stats.get('metadata', {}).get('status', 'unknown')
            
            # プログレスバーを生成
            bar_length = 30
            filled_length = int(bar_length * progress / 100)
            bar = '█' * filled_length + '░' * (bar_length - filled_length)
            
            print(f"  {chapter_name:20} [{bar}] {progress:5.1f}% ({status})")
        
        print()
        print("📈 全体統計:")
        summary = report['summary']
        print(f"  総文字数: {summary['total_chars']:,}")
        print(f"  総単語数: {summary['total_words']:,}")
        print(f"  演習数: {summary['total_exercises']}")
        print(f"  コード例: {summary['total_code_examples']}")
        print(f"  平均進捗: {summary['average_progress']:.1f}%")
        print("="*60)
    
    def save_report(self, report, output_path=None):
        """レポートをJSONファイルに保存"""
        if output_path is None:
            output_path = self.project_root / 'progress-report.json'
        
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(report, f, ensure_ascii=False, indent=2)
        
        print(f"\nReport saved to: {output_path}")

def main():
    if len(sys.argv) < 2:
        print("Usage: python progress-tracker.py <project_root>")
        sys.exit(1)
    
    project_root = sys.argv[1]
    tracker = ProgressTracker(project_root)
    
    report = tracker.generate_report()
    tracker.print_report(report)
    tracker.save_report(report)

if __name__ == "__main__":
    main()