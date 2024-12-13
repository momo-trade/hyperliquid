use hyperliquid::http::client::HttpClient;
// use hyperliquid::models::Token;
// use std::collections::HashSet;

#[tokio::main]
async fn main() {
    let client = HttpClient::new(false);

    match client.fetch_spot_meta().await {
        Ok(spot_meta) => {
            println!("Tokens:");
            println!("{}", spot_meta.tokens.len());
            for token in spot_meta.tokens {
                println!("{}, {}, {}", token.name, token.index, token.is_canonical);
            }

            println!("Universe:");
            println!("{}", spot_meta.universe.len());
            for universe in spot_meta.universe {
                println!(
                    "{}, {}, {:?}, {}",
                    universe.name, universe.index, universe.tokens, universe.is_canonical
                );
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    // let spot_response = match client.fetch_spot_meta().await {
    //     Ok(spot_meta) => spot_meta,
    //     Err(err) => {
    //         println!("Error: {}", err);
    //         return;
    //     }
    // };

    // let token_map: HashMap<u32, String> = spot_response
    //     .tokens
    //     .iter()
    //     .map(|token| (token.index, token.name.clone()))
    //     .collect();

    // for pair in spot_response.universe {
    //     let token_names: Vec<String> = pair
    //         .tokens
    //         .iter()
    //         .filter_map(|id| token_map.get(id).cloned())
    //         .collect();

    //     // トークン名が2つ揃っている場合のみ表示
    //     if token_names.len() == 2 {
    //         println!("{}/{}", token_names[0], token_names[1]);
    //     } else {
    //         println!("対応するトークン名が見つかりませんでした。");
    //     }
    // }
}
