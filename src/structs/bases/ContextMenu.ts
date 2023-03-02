import { BaseCommand } from "./BaseCommand.js";
import { Interaction, ContextMenuCommandBuilder } from "discord.js";

export interface ContextMenuBase extends BaseCommand {
    name: string;
    execute: (interaction: Interaction) => Promise<void>;
    builder: () => ContextMenuCommandBuilder;
}