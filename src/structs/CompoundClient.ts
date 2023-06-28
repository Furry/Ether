import * as Discord from "discord.js";
import { Client, Events, Routes } from "discord.js";
import { Command } from "../types.js";
import { CommandType } from "./bases/BaseCommand.js";
import { Database } from "./Database.js";
import { Logger } from "./Logger.js";

import fs from "fs";

export class CompoundClient extends Client {
    public db: Database;
    public debug: boolean;
    private commands: Map<string, Command> = new Map();
    private static _instance: CompoundClient = null as any;

    public constructor(options: Discord.ClientOptions, mongoUri: string, debugMode: boolean = false) {
        super(options);
        this.db = new Database(mongoUri, "Ether");
        this.debug = debugMode;
        CompoundClient._instance = this;
    }

    public static get instance(): CompoundClient {
        if (CompoundClient._instance == null) {
            Logger.error("Attempted to access client singleton before init!");
        }

        return CompoundClient._instance;
    }

    public async start(token: string) {
        await this.db.connect();
        this.handleEvents();
        return this.login(token);
    }

    public registerCommand(name: string, command: Command) {
        if (this.debug) {
            Logger.info(`Registering command ${name}`);
        }

        this.commands.set(name, command);
    }

    public handleEvents() {
        this.on(Events.ThreadCreate, (thread) => {
            CompoundClient.instance.db.direct.collection("threads").insertOne({
                id: thread.id,
                name: thread.name,
                members: thread.members.cache.map(m => m.id),
                archivedAt: null,
                createdAt: thread.createdAt,
                lastMessageAt: thread.lastMessage?.createdTimestamp ?? null,
                creator: thread.ownerId,
                disableArchive: false
            })
        })

        this.on(Events.ThreadMembersUpdate, (members, removed, thread) => {
            CompoundClient.instance.db.direct.collection("threads").updateOne({id: thread.id}, {
                $set: {
                    members: thread.members.cache.map(m => m.id)
                }
            })
        })

        this.on(Events.InteractionCreate, async (interaction) => {
            if (interaction.isContextMenuCommand() || interaction.isCommand()) {
                if (this.debug) {
                    Logger.info(`Received context menu command ${interaction.commandName}`);
                }
                if (this.commands.has(interaction.commandName)) {
                    const command = this.commands.get(interaction.commandName);
                    if (command) {
                        try {
                            await command.execute(interaction);
                        } catch (err) {
                            Logger.error(`Command ${interaction.commandName} failed to execute: ${err}`);
                        }
                    } else {
                        Logger.error(`Command ${interaction.commandName} is not defined`);
                    }
                }
            }
        })

        this.on(Events.MessageCreate, (message) => {
            if (!message.guild) return;
            if (message.channel.id != "1094404164601253999") return;

            if (message.content.length > 10) {
                let c = message.content.replace(/(\r\n|\n|\r)/gm, "");
                fs.appendFileSync("messages.txt", c + "\n");
            }
        })
    }

    public async flushCommands() {
        Logger.info("Flushing commands...")
        const start = Date.now();
        await this.rest.put(Routes.applicationCommands(
            this.user?.id as string),
            { 
                body:
                    Array.from(this.commands.values())
                        .filter((command) => command.type === CommandType.Slash || command.type === CommandType.ContextMenu)
                        .map((command) => (command as any).builder()) 
            }
        )
        Logger.info(`Flushed commands (${Date.now() - start}ms)`);
    }

    public async loadGuildMembers(guildId: string) {
        Logger.info(`Loading guild members for ${guildId}...`)
        await this.guilds.cache.get(guildId)?.members.fetch();
        Logger.info("Done!");
    }

}