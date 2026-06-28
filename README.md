# Solana Speedrun — Message Registry (devnet)

Programa Anchor simples deployado na **devnet** que demonstra `initialize` +
uma instrução de mutação de estado (`update_message`).

O código completo está em [`message_registry/`](message_registry/).

## Descrição

Message Registry deixa cada usuário registrar uma mensagem on-chain em uma conta PDA
própria, derivada de `["message", authority]`. A instrução `initialize` cria a conta
gravando o dono (`authority`) e o texto inicial; `update_message` permite que **apenas o
dono** altere o texto (validado via `has_one = authority`). Demonstra criação de conta
(PDA), persistência de estado e checagem de autoridade.

## Submission

- **Program ID (devnet):** `DBfznUhc47xnK5B1wC6zWcqtZzWqu2MTNZckWBi6w5fX`
- **Authority / wallet:** `F5RGmqJnQfGPG8QaJNWyx9r2VBmw7tYgGiQSSAgggf8i`
- **Conta PDA da mensagem:** `ENyjjT3uieR1Qb9ZYpYrETEFbZWEdPNb9ofERE1LVEyA`

### Links do Solana Explorer (cluster=devnet)

- Programa:
  https://explorer.solana.com/address/DBfznUhc47xnK5B1wC6zWcqtZzWqu2MTNZckWBi6w5fX?cluster=devnet
- Transação `initialize` (cria o PDA):
  https://explorer.solana.com/tx/5gYK7EQR2J2HzaYiaAcZFU966uVVKadPqMkad5dqveocA4Jh3LavdFkwbuXjDBZobTS92Qfnsu3Zw4mFHCuvazro?cluster=devnet
- Transação `update_message` (grava "Leonardo - teste 1"):
  https://explorer.solana.com/tx/JAj1ZUsfifnLs338DfsgLC8taCYL7UZw3ZGWdDf5v1D3uU92DGfMZ14oY3GgMSTwcwe64sGuhaoi466xLugGxSN?cluster=devnet

> Na página da transação `update_message`, a seção **"Program Instruction Logs"**
> mostra `Program log: Message updated: Leonardo - teste 1` — confirmando que o
> programa recebeu e gravou a mensagem on-chain. O texto atual lido da conta PDA
> é exatamente `Leonardo - teste 1`.

## Estrutura

- [`message_registry/programs/message_registry/src/`](message_registry/programs/message_registry/src/) — programa Anchor (modular):
  - `state.rs` — conta `MessageAccount { authority, text }`
  - `instructions/initialize.rs` — cria o PDA
  - `instructions/update_message.rs` — atualiza o texto (só o dono)
  - `error.rs`, `constants.rs`, `lib.rs`
- [`message_registry/client/`](message_registry/client/) — cliente Rust isolado que envia as transações de interação contra a devnet
  (usado no lugar do client TypeScript porque o `npm` está bloqueado por um proxy TLS
  corporativo; o canal Rust/Solana funciona).

## Como reproduzir

```bash
cd message_registry

# build + deploy
anchor build
anchor deploy --provider.cluster devnet   # erro de IDL no fim é só o npm bloqueado; o programa é deployado

# interagir (envia initialize + update_message e imprime as signatures)
cd client && cargo run --release
```
