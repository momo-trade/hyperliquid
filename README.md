# Hyperliquid Rust SDK

This is a simple wrapper for interacting with the Hyperliquid exchange using Rust. If you're looking to trade on Hyperliquid with Rust, this SDK is for you. While an official SDK exists, it's not actively maintained, so this project was created to provide a more reliable and user-friendly option.

Whether you're building trading bots, analytics tools, or just experimenting, this SDK aims to make your journey smoother and more efficient. Enjoy coding!

---

## Disclaimer and Important Notices

1. **Use at Your Own Risk:** This SDK is provided as-is without any warranty of any kind. The developers are not responsible for any financial loss or other damages that may result from using this software.
   
2. **Market Risks:** Trading in cryptocurrency and derivatives involves substantial risk. Prices can fluctuate significantly, and users must be aware of market risks before using this SDK.

3. **Accuracy of Data:** While this SDK interfaces with Hyperliquid's API, it does not guarantee the accuracy or reliability of the data provided by the exchange.

4. **API Compliance:** Users are responsible for complying with Hyperliquid's API usage terms and conditions.

5. **Test Before Use:** It is strongly recommended to thoroughly test your implementation in a testnet environment (`is_test = true`) before using this SDK for live trading.

---

## Examples

### Running HTTP Request Example

To run the HTTP request example for fetching spot metadata, use the following command:

```bash
cargo run --example fetch_spot_meta
```

### Running Websocket Example

To run the WebSocket example for fetching live data, use the following command:

```bash
cargo run --example fetch_websocket_data
```

## License
This project is open-source and distributed under the [MIT License](https://opensource.org/licenses/MIT).
