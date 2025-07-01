# SeriaClient

Represents the main Seria client.

## Properties

### `gateway`

> The gateway client.<br><b>Type:</b> [`GatewayClient`](GatewayClient.md)

### `http`

> The HTTP client.<br><b>Type:</b> [`HttpClient`](HttpClient.md)

## Methods

### `connect()`

Connect the bot to the gateway.

> Returns [`SeriaResult`](SeriaResult.md)

### `new(http, gateway)`

Create a new Seria client instance.

> | Parameter | Type | Description |
> |--------------|--------|--------------------------------------|
>| **http** | [`HttpClient`](HttpClient.md) | The HTTP client instance. |
> | **gateway** | [`GatewayClient`](GatewayClient.md) | The gateway client instance. |
>
> Returns [`SeriaClient`](SeriaClient.md)