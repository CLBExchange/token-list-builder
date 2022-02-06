# Token List Builder (TLB) by CLB

The Token List Builder is used to build the Certified Token List, a token list composed of multiple other token lists.

Related repos:

- [Certified Token List](https://github.com/CLBExchange/certified-token-list)
- [Solana Token Lists](https://github.com/CLBExchange/token-lists)

## The Certified© Token List

The Certified© Token List (CTL) is a token list composed of all token lists defined in `Lists.toml`.

It exists to make it easier for other projects to maintain their own token lists, without needing to modify the official Solana token list. This is especially useful for AMMs which may have hundreds if not thousands of LP tokens.

The JSON files that make up the CTL can be found [on its GitHub repository](https://github.com/CLBExchange/certified-token-list).

### Adding Tokens to the CTL

It is not possible to directly add tokens to the CTL; however, we support the official Solana Token List.

To add a token to the token list, follow [the official Solana Labs token list instructions](https://github.com/solana-labs/token-list). Their repository has an auto-merge feature.

### Adding a Token List

To add a token list to be tracked by the Certified Token List, add your token list URL to `Lists.toml` and send a pull request. Ensure that it is above the Solana token list.

## License

All code here is licensed under the Apache 2.0 License.
