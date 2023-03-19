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
await channel.declareQueue({ queue: queueName });
await channel.consume(                                                          
  { queue: queueName },
  async (args, props, data) => {
    console.log(`Properties`, JSON.stringify(args));
    console.log(`Args`, JSON.stringify(props));
    console.log(`Data; `,new TextDecoder().decode(data));
    await channel.ack({ deliveryTag: args.deliveryTag });
  },
);