#!/usr/bin/env python3
"""
Zenn Articles 形式への変換ツール
Markdownファイルを Zenn の記事形式に変換
"""

import os
import sys
import yaml
import re
from pathlib import Path
from datetime import datetime

class ZennConverter:
    def __init__(self, source_dir, output_dir):
        self.source_dir = Path(source_dir)
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
    
    def convert_chapter(self, chapter_path):
        """章を Zenn 記事形式に変換"""
        with open(chapter_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # フロントマターを生成
        frontmatter = self.generate_frontmatter(chapter_path)
        
        # コンテンツを変換
        converted_content = self.convert_content(content)
        
        # 出力ファイル名を生成
        output_filename = self.generate_filename(chapter_path)
        output_path = self.output_dir / output_filename
        
        # ファイルを書き出し
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write(f"---\n{frontmatter}---\n\n{converted_content}")
        
        print(f"Converted: {chapter_path} -> {output_path}")
    
    def generate_frontmatter(self, chapter_path):
        """Zenn用のフロントマターを生成"""
        chapter_name = chapter_path.stem
        
        frontmatter = {
            'title': self.extract_title(chapter_path),
            'emoji': '🦀',
            'type': 'tech',
            'topics': ['rust', 'programming', 'systems'],
            'published': False,
            'publication_name': 'rust_professional_book'
        }
        
        return yaml.dump(frontmatter, allow_unicode=True, default_flow_style=False)
    
    def extract_title(self, chapter_path):
        """章からタイトルを抽出"""
        with open(chapter_path, 'r', encoding='utf-8') as f:
            for line in f:
                if line.startswith('# '):
                    return line[2:].strip()
        return "Untitled"
    
    def convert_content(self, content):
        """コンテンツをZenn形式に変換"""
        # コードブロックの言語指定を確認
        content = re.sub(r'```(\w+)', r'```\1', content)
        
        # 画像パスを調整
        content = re.sub(r'!\[([^\]]*)\]\(([^)]+)\)', 
                         self.convert_image_path, content)
        
        # 内部リンクを調整
        content = re.sub(r'\[([^\]]+)\]\(#([^)]+)\)', 
                         r'[\1](#\2)', content)
        
        return content
    
    def convert_image_path(self, match):
        """画像パスを変換"""
        alt_text = match.group(1)
        image_path = match.group(2)
        # Zenn では画像は別途アップロードが必要
        return f'![{alt_text}]({image_path})'
    
    def generate_filename(self, chapter_path):
        """出力ファイル名を生成"""
        chapter_name = chapter_path.stem
        timestamp = datetime.now().strftime('%Y%m%d')
        return f"{chapter_name}-{timestamp}.md"
    
    def convert_all(self):
        """全章を変換"""
        chapters_dir = self.source_dir / 'manuscript' / 'chapters'
        
        for chapter_dir in sorted(chapters_dir.iterdir()):
            if chapter_dir.is_dir():
                chapter_file = chapter_dir / 'chapter.md'
                if chapter_file.exists():
                    self.convert_chapter(chapter_file)

def main():
    if len(sys.argv) < 2:
        print("Usage: python zenn-converter.py <project_root>")
        sys.exit(1)
    
    project_root = sys.argv[1]
    output_dir = Path(project_root) / 'build' / 'output' / 'zenn'
    
    converter = ZennConverter(project_root, output_dir)
    converter.convert_all()
    
    print(f"\nConversion complete! Files saved to: {output_dir}")

if __name__ == "__main__":
    main()