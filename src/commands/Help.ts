import { Interaction } from "discord.js";
import { CommandType } from "../structs/bases/BaseCommand.js";
import { SlashCommandBase } from "../structs/bases/SlashCommand.js";

export class Help implements SlashCommandBase {
    type =  CommandType.Slash;
    name = "help";
    description = "Displays a list of commands.";


    public async execute(interaction: Interaction) {
        if (!interaction.isChatInputCommand()) return;
        await interaction.reply("Help command executed!");
    };
}