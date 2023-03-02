import { Interaction } from "discord.js";
import { BaseCommand } from "./BaseCommand.js";

export interface SlashCommandBase extends BaseCommand {
    name: string;
    description: string;
    defaultPermission?: boolean;
    guilds?: string[];
    execute: (interaction: Interaction) => Promise<void>;
}