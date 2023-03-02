import * as Discord from "discord.js";
import { Client, Events, Routes } from "discord.js";
import { Command } from "../types.js";
import { CommandType } from "./bases/BaseCommand.js";
import { Database } from "./Database.js";
import { Logger } from "./Logger.js";

export class CompoundClient extends Client {
    public db: Database;
    public debug: boolean;
    private commands: Map<string, Command> = new Map();

    public constructor(options: Discord.ClientOptions, mongoUri: string, debugMode: boolean = false) {
        super(options);
        this.db = new Database(mongoUri, "Ether");
        this.debug = debugMode;
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
        this.on(Events.InteractionCreate, (interaction) => {
            if (interaction.isContextMenuCommand()) {
                if (this.debug) {
                    Logger.info(`Received context menu command ${interaction.commandName}`);
                }
                if (this.commands.has(interaction.commandName)) {
                    const command = this.commands.get(interaction.commandName);
                    if (command) {
                        command.execute(interaction);
                    } else {
                        Logger.error(`Command ${interaction.commandName} is not defined`);
                    }
                }
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

}