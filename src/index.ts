import * as Discord from "discord.js";
import { GatewayIntentBits } from "discord.js";

import dotenv from "dotenv";
import { Prune } from "./commands/Prune.js";
import { ReverseImageMessage } from "./commands/ReverseImageMessage.js";
import { ReverseImageUser } from "./commands/ReverseImageUser.js";
import { Evaluator } from "./meta/Eval.js";
import { CompoundClient } from "./structs/CompoundClient.js";
import { Intervals } from "./structs/Intervals.js";
import { Logger } from "./structs/Logger.js";
import { Threads } from "./commands/Threads.js";
import { Ruin } from "./guildactions/ruin.js";

dotenv.config();
const args = process.argv.slice(2)
    .map((arg) => arg.toLowerCase());

const client = new CompoundClient({
    intents: [
        GatewayIntentBits.Guilds,
        GatewayIntentBits.GuildMembers,
        GatewayIntentBits.MessageContent,
        GatewayIntentBits.GuildMessageReactions,
        GatewayIntentBits.DirectMessages,
        GatewayIntentBits.DirectMessageReactions,
        GatewayIntentBits.GuildMessages
    ]}, 
    process.env.MONGO_CONNECTION_URI as string,
    args.includes("--debug")
);

client.registerCommand("Find Icon Source", new ReverseImageUser());
client.registerCommand("Find Image Source", new ReverseImageMessage());
client.registerCommand("prune", new Prune());
client.registerCommand("threads", new Threads());

new Ruin(client).start();

client.start(
    process.env.DISCORD_TOKEN as string
).then(async () => {
    Logger.info("Client started.");
    await client.loadGuildMembers("933683767908913222");
    Intervals.startStatus(client);
    if (args.includes("--flushcommands")) {
        await client.flushCommands();
    }

    if (args.includes("--debug")) {
        Logger.info("Debug mode enabled.");
        Evaluator.EvalLoop(client);
    }
})