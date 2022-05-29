# safe-url-macro

## Rust でコードを書いているタイミングで URL のパースのエラーを検知する

今回作成した safe_url! を使うと
不正なな URL (https//example.com) を指定しているとき
![image](https://user-images.githubusercontent.com/16481886/170857361-c72c8479-2e30-4339-8df4-4ffc6b3128f7.png)

正常な URL (https://example.com) を指定しているとき
![image](https://user-images.githubusercontent.com/16481886/170857359-af16b004-721f-4617-ac9b-3bc48c54acd2.png)

このように実行前にミスを検知できる.

[Qiita の記事](https://qiita.com/narumincho/items/32bfd883197e980c9bbd)
