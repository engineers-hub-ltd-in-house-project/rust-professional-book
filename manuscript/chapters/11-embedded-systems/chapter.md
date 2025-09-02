# 第11章: 組み込みシステムとリアルタイム処理

## 学習目標
本章を修了すると、以下が可能になります：
- [ ] 組み込みシステムにおけるRustの利点（安全性、パフォーマンス）を説明できる。
- [ ] `#![no_std]`環境でRustプログラムをビルドし、実行できる。
- [ ] HAL（Hardware Abstraction Layer）の役割を理解し、GPIOなどのペリフェラルを操作できる。
- [ ] `embedded-hal`トレイトを使い、特定のハードウェアに依存しないポータブルなデバイスドライバを記述できる。

---

## 11.1 導入：物理世界を制御するRust

これまでの章では、OSという強力な抽象化の上で動作するアプリケーションについて学んできました。しかし、世の中にはOSが存在しない、あるいはOSが不要なコンピュータが溢れています。エアコン、自動車のエンジン制御ユニット、工場のロボットアーム、IoTデバイスなど、物理的な世界と直接やり取りするこれらの小型コンピュータが**組み込みシステム**です。

歴史的に、この領域はC言語が支配的でした。C言語はハードウェアを直接制御する能力に長けていますが、その代償としてメモリ安全性はプログラマの注意深さに完全に依存します。バッファオーバーフローやダングリングポインタといったバグは、PC上ではアプリケーションのクラッシュで済みますが、組み込みシステムでは物理的な損害や、時には人命に関わる危険を引き起こす可能性があります。

ここに、Rustが登場します。Rustは、C言語に匹敵する低レベルな制御とパフォーマンスを提供しつつ、コンパイル時の静的解析によってメモリ安全性を保証します。GC（ガベージコレクタ）を持たないため、実行時の挙動が予測可能であり、リアルタイム性が要求される処理にも適しています。これらの特性により、Rustは組み込み開発の分野で急速に採用を広げている、現代的で安全な選択肢なのです。

---

## 11.2 OSなしの世界：`#![no_std]`

組み込みシステム開発の第一歩は、標準ライブラリ（`std`）への依存を外すことです。`std`は、ファイルシステム、ネットワーク、メモリ確保など、OSが提供する機能を前提としています。OSのない環境（**ベアメタル**）では、`std`は利用できません。

Rustでは、ソースコードの先頭に`#![no_std]`と記述することで、`std`をリンクしないようにコンパイラに指示します。では、`std`なしで何が使えるのでしょうか？

- **`core`クレート:** `std`のサブセットで、OSに依存しない最も基本的な要素（`Option`, `Result`, `Iterator`、プリミティブ型、アトミック操作など）を提供します。`#![no_std]`環境でも常に利用可能です。
- **`alloc`クレート（オプショナル）:** `Vec`, `String`, `Box`といった、ヒープメモリ確保を必要とするデータ構造を提供します。これを利用するには、開発者が「グローバルアロケータ」を自ら定義し、ヒープメモリの管理方法をコンパイラに教える必要があります。

多くの小規模なマイクロコントローラではヒープメモリ自体を持たないため、`alloc`すら使わず、`core`のみで開発を行います。

### 11.2.1 ハンズオン：Lチカ (Blinky) on Raspberry Pi Pico

組み込み開発の「Hello, World!」は、LEDを点滅させる「Lチカ」です。ここでは、人気の高いマイクロコントローラであるRaspberry Pi Picoをターゲットに、`#![no_std]`環境でのプログラミングを体験します。このハンズオンは`code-examples/chapter-11/pico-blinky/`にあります。

**1. ターゲットとツールチェーンの準備**

```bash
# ARM Cortex-M0+ (PicoのCPU) 用のターゲットを追加
rustup target add thumbv6m-none-eabi

# cargo-generateとflip-linkをインストール
cargo install cargo-generate
cargo install flip-link
```

**2. プロジェクトの生成**

`rp-pico-project-template`というテンプレートから、必要な設定が済んだプロジェクトを生成します。

```bash
cargo generate --git https://github.com/rp-rs/rp-pico-project-template -n pico-blinky
cd pico-blinky
```

**3. コードの記述 (`src/main.rs`)**

生成されたテンプレートには、既にLチカのコードが含まれています。主要な部分を見ていきましょう。

```rust
#![no_std]
#![no_main]

use bsp::entry;
use defmt_rtt as _;
use panic_probe as _;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

use rp_pico as bsp;

#[entry]
fn main() -> ! {
    // 1. ハードウェアの初期化
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
    let clocks = init_clocks_and_plls(/* ... */).unwrap();

    // 2. GPIOピンの初期化
    let pins = bsp::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);
    // Picoの内蔵LEDはGPIO 25番ピン
    let mut led_pin = pins.led.into_push_pull_output();

    // 3. 無限ループでLEDを点滅
    loop {
        led_pin.set_high().unwrap();
        cortex_m::asm::delay(clocks.system_clock.freq().to_Hz() / 2);
        led_pin.set_low().unwrap();
        cortex_m::asm::delay(clocks.system_clock.freq().to_Hz() / 2);
    }
}
```

**4. ビルドと実行**

```bash
cargo build --release
# elf2uf2-rsを使って、Picoに書き込めるUF2ファイルに変換
elf2uf2-rs -d target/thumbv6m-none-eabi/release/pico-blinky target/thumbv6m-none-eabi/release/pico-blinky.uf2
```

生成された`.uf2`ファイルを、PicoをUSBマスストレージモードで接続してドラッグ＆ドロップすれば、LEDが点滅を開始します。これで、OSのないベアメタル環境でRustコードが動作しました。

---

## 11.3 ハードウェア抽象化層 (HAL)

`pico-blinky`の例では、`rp-pico-hal`というクレートが出てきました。これが**HAL (Hardware Abstraction Layer)**です。

マイクロコントローラの機能（GPIO、タイマー、I2C、SPIなど）は、特定のアドレスにある**メモリマップドレジスタ**に特定の値を書き込むことで制御されます。この操作は本質的に`unsafe`であり、非常に低レベルです。

HALは、これらの`unsafe`なレジスタ操作を、安全で高レベルなAPI（例：`led_pin.set_high()`）にカプセル化する役割を担います。これにより、開発者はハードウェアのマニュアルと格闘することなく、安全にペリフェラルを操作できます。

### 11.3.1 `embedded-hal`：ポータブルなドライバのために

さらに重要なのが、`embedded-hal`というクレートです。これは、特定のHALの実装ではなく、**トレイトの集合**を定義します。例えば、

-   `digital::v2::OutputPin`: デジタル出力ピンが持つべき振る舞い（`set_high`, `set_low`など）を定義。
-   `blocking::i2c::WriteRead`: I2C通信が持つべき振る舞いを定義。

各マイコン向けのHAL（`rp-pico-hal`など）は、これらの標準トレイトを自身が提供する具体的な型に対して実装します。その結果、開発者は、具体的なハードウェア（Picoなど）に依存せず、`embedded-hal`のトレイトにジェネリックな**デバイスドライバ**を書くことができます。そのドライバは、`embedded-hal`に準拠したHALを持つあらゆるマイクロコントローラで再利用可能になります。

### 11.3.2 ハンズオン：I2C温度センサーのポータブルなドライバ

この強力な抽象化を、I2C接続の温度センサー（TMP102）のドライバを実装することで体験しましょう。このハンズオンは`code-examples/chapter-11/pico-i2c-driver/`にあります。

**1. ドライバの作成 (`src/driver.rs`)**

ドライバは、具体的なI2Cの実装ではなく、`embedded-hal`の`blocking::i2c::WriteRead`トレイトにジェネリックに実装します。

```rust
use embedded_hal::blocking::i2c;

const SENSOR_ADDR: u8 = 0x48;

// I2Cバスにジェネリックなドライバ
pub struct Tmp102<I2C> {
    i2c: I2C,
}

impl<I2C, E> Tmp102<I2C>
where
    // I2Cは、WriteReadトレイトを実装する任意の型
    I2C: i2c::WriteRead<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn read_temperature(&mut self) -> Result<f32, E> {
        let mut buffer = [0u8; 2];
        // センサーに読み取りを要求
        self.i2c.write_read(SENSOR_ADDR, &[], &mut buffer)?;
        // 読み取った値を温度に変換
        let temp_raw = ((buffer[0] as i16) << 4) | (buffer[1] >> 4);
        Ok(temp_raw as f32 * 0.0625)
    }
}
```

**2. Picoでのドライバ利用 (`src/main.rs`)**

アプリケーション側では、Picoの具体的なI2Cペリフェラルを初期化し、それをドライバに渡します。

```rust
// ... (pico-blinkyと同様の初期化) ...

// 1. PicoのI2Cペリフェラルを初期化
let sda_pin = pins.gpio4.into_mode::<bsp::hal::gpio::FunctionI2C>();
let scl_pin = pins.gpio5.into_mode::<bsp::hal::gpio::FunctionI2C>();

let i2c = bsp::hal::I2C::i2c0(
    pac.I2C0,
    sda_pin,
    scl_pin,
    400.kHz(),
    &mut pac.RESETS,
    &clocks.peripheral_clock,
);

// 2. ドライバをインスタンス化
let mut sensor = Tmp102::new(i2c);

loop {
    // 3. 温度を読み取って表示
    if let Ok(temp) = sensor.read_temperature() {
        defmt::info!("Temperature: {} C", temp);
    }
    cortex_m::asm::delay(clocks.system_clock.freq().to_Hz());
}
```
この`Tmp102`ドライバは、`rp-pico-hal`に一切依存していません。`stm32f4xx-hal`など、別のマイコンのHALが提供するI2C実装を渡せば、全く同じコードが別のハードウェアで動作します。これこそが`embedded-hal`エコシステムの力です。

---

## 11.4 まとめ

本章では、Rustが組み込みシステムという、リソースが極度に制約された環境でいかに強力な選択肢であるかを学びました。

-   `#![no_std]`環境は、OSのないベアメタル環境でRustを動作させるための基本です。
-   HAL（Hardware Abstraction Layer）は、`unsafe`なレジスタ操作を安全な高レベルAPIにカプセル化します。
-   `embedded-hal`トレイトは、ハードウェアの違いを吸収し、ポータブルなデバイスドライバの作成を可能にする、Rust組み込みエコシステムの心臓部です。

Rustの安全性とゼロコスト抽象化は、これまでC言語が支配的だった領域に、より堅牢で生産性の高い開発体験をもたらします。