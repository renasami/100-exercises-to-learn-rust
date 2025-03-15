#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached
        assert_eq!(v.capacity(), 2);

        v.push(3); // beyond capacity, needs to resize
                   // +1分のキャパシティがなぜか確保されている。
                   // Rust の標準ライブラリでの Vec の再アロケーションのアルゴリズムでは、要素を追加して現在の容量を超えた場合、通常は現在の容量の約2倍の新しいバッファを確保します。
                   // この例では、最初に容量2の Vec を作成しており、2 要素目で満杯となります。その後、3 番目の要素を push すると、Vec は再アロケーションを行い、2×2＝4 の容量を持つ新しいバッファに切り替えます。
                   // なお、この成長アルゴリズムは内部実装に依存しているため、将来的に変わる可能性はありますが、現在はそのような動作になっています。

        // Can you guess what the new capacity will be?
        // Beware that the standard library makes no guarantees about the
        // algorithm used to resize the vector, so this may change in the future.
        assert_eq!(v.capacity(), 4);
    }
}
