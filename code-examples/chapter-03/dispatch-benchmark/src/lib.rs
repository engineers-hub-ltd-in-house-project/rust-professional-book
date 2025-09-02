// ベンチマークのためのシンプルなトレイト
pub trait Renderer {
    fn render(&self) -> u64;
}

// トレイトを実装する2つの異なる構造体
pub struct ComponentA {
    pub data: u64,
}

impl Renderer for ComponentA {
    // インライン化を防ぎ、関数呼び出しのオーバーヘッドを分離しやすくする
    #[inline(never)] 
    fn render(&self) -> u64 {
        self.data
    }
}

pub struct ComponentB {
    pub data: u64,
}

impl Renderer for ComponentB {
    #[inline(never)]
    fn render(&self) -> u64 {
        self.data * 2
    }
}

// 静的ディスパッチ（ジェネリクス）を使う関数
// コンパイラは、この関数の型ごとに特化したバージョンを生成する（モノモーフィゼーション）
#[inline(never)]
pub fn process_static<T: Renderer>(item: &T) -> u64 {
    item.render()
}

// 動的ディスパッチ（トレイトオブジェクト）を使う関数
// この関数は一度しかコンパイルされない。呼び出すべき具体的な`render`メソッドは
// 実行時にvtableルックアップを通じて決定される。
#[inline(never)]
pub fn process_dynamic(item: &dyn Renderer) -> u64 {
    item.render()
}
