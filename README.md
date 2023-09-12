# Eightfish implementation on Solana

## impl 1 方案一

还是采用基于一个Dapp模块创建一个系列PDA账户作为存储的方式，存储链上的数据。

## impl 2

采用Solana上最新的cnft 作为存储的方式存储数据。好处就是创建的账户要少，花费的SOl少，缺点是需要自己实现rpc的请求方法。
