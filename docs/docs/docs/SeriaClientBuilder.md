# SeriaClientBuilder

Represents a builder pattern for constructing a [`SeriaClient`](SeriaClient.md).

## Methods

### `build()`

Build a [`SeriaClient`](SeriaClient.md)

> Returns `SeriaResult<SeriaClient>`

### `token(token)`

The bot token.

> | Parameter | Type | Description |
> |---|---|---|
> **token** | `Impl Into<String>` | <br>The token of the bot.
>
> Return [`SeriaClientBuilder`](SeriaClientBuilder.md)

### `new()`

Create a new builder.

> Return [`SeriaClientBuilder`](SeriaClientBuilder.md)
