import { connect, type AmqpConnectOptions } from "https://deno.land/x/amqp/mod.ts";

const config: AmqpConnectOptions = {
    host: "localhost",
    port: 5672,
    username: "user",
    password: "password",
}

const connection = await connect(config);
const channel = await connection.openChannel();

const queueName = "my.queue";
console.log(`Connecting to ${queueName}`)
await channel.declareQueue({ queue: queueName });

for (let i in [1, 2, 3]) {
    await channel.publish(
      { routingKey: queueName },
      { contentType: "application/json" },
      new TextEncoder().encode(JSON.stringify({ foo: "bar", count: i })),
    );

    console.log(`Published message ${i} to ${queueName}`);
}

await connection.close();