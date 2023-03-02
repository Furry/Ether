import * as Discord from "discord.js";
import { GatewayIntentBits } from "discord.js";

import dotenv from "dotenv";
import { ReverseImageMessage } from "./commands/ReverseImageMessage.js";
import { ReverseImageUser } from "./commands/ReverseImageUser.js";
import { CompoundClient } from "./structs/CompoundClient.js";
import { Logger } from "./structs/Logger.js";

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
        GatewayIntentBits.DirectMessageReactions
    ]}, 
    process.env.MONGO_CONNECTION_URI as string,
    args.includes("--debug")
);

client.registerCommand("Find Icon Source", new ReverseImageUser());
client.registerCommand("Find Image Source", new ReverseImageMessage());

client.start(
    process.env.DISCORD_TOKEN as string
).then(async () => {
    Logger.info("Client started.");
    if (args.includes("--flushcommands")) {
        await client.flushCommands();
    }
})