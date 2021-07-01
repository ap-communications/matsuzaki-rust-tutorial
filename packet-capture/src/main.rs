use pnet::datalink;
use pnet::datalink::Channel::Ethernet;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    // 引数の数をチェック
    if args.len() != 2 {
        error!("Please specify target interface name");
        std::process::exit(1);
    }

    let interface_name = &args[1];

    // インターフェースの選択
    let interfaces = datalink::interfaces();

    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interface");

    /* データリンクレイヤのチャンネルを取得 */
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {

        // 正常系処理
        Ok(Ethernet(tx, rx)) => (tx, rx),

        // channel type が想定されない種別
        OK(_) => panic!("Unhandled channel type"),

        // Errの場合
        Err(e) => panic!("Failed to create datalink channel {}", e),
    };

    loop {
        match rx.next() {
            OK(frame) => {

                // 受信データからEthernetフレームを作成
                let frame = EthernetPacket::new(frame).unwrap();

                match frame.get_ethertype() {

                    // matchで腕にブロック使う時はカンマの省略が可能
                    // https://qnighy.hatenablog.com/entry/2017/06/10/220000
                    EtherType::Ipv4 => {
                        // frameの値を指す可変のポインタ
                        ipv4_handler(&frame);
                    }

                    EtherType::Ipv6 => {
                        ipv6_handler(&frame);
                    }

                    // その他の毛＾す全てを包含のワイルドカード処理
                    _ => {
                        info!("Not an IPv4 or IPv6packet");
                    }
                }
            }
            Err(e) => {
                // Errマクロ
                error!("Failed to read: {}", e);
            }
        }
    }
}
