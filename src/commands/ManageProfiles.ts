import { Interaction, CacheType, SlashCommandBuilder } from "discord.js";
import { CommandType } from "../structs/bases/BaseCommand.js";
import { SlashCommandBase } from "../structs/bases/SlashCommand.js";

export class ManageProfiles implements SlashCommandBase {
    type = CommandType.Slash;

    public guilds = [process.env.TEST_GUILD_ID as string]
    public name = "profile";
    public description = "Manage your profiles.";

    public async execute(interaction: Interaction) {
        if (!interaction.isChatInputCommand()) return;
        const subcommand = interaction.options.getSubcommand();

        switch (subcommand) {
            case "create":
                await interaction.reply({
                    content: "Create profile executed!",
                    ephemeral: true,
                });
                break;
            case "list":
                await interaction.reply({
                    content: "List profiles executed!",
                    ephemeral: true,
                });
                break;
            case "view":
                await interaction.reply({
                    content: "View profile executed!",
                    ephemeral: true,
                });
                break;
            case "setAvatar":
                await interaction.reply({
                    content: "Set avatar executed!",
                    ephemeral: true,
                });
                break;
            case "edit":
                await interaction.reply({
                    content: "Edit profile executed!",
                    ephemeral: true,
                });
                break;
            case "delete":
                await interaction.reply({
                    content: "Delete profile executed!",
                    ephemeral: true,
                });
                break;
        }
    }

    public builder() {
        return new SlashCommandBuilder()
            .setName(this.name)
            .setDescription(this.description)
            .addSubcommand((subcommand) =>
                subcommand
                    .setName("create")
                    .setDescription("Create a new profile")
                    // Will prompt modal to fill out fields
            )

            .addSubcommand((subcommand) =>
                subcommand
                    .setName("list")
                    .setDescription("List all your profiles")
            )
            .addSubcommand((subcommand) =>
                subcommand
                    .setName("view")
                    .setDescription("View an existing profile")
                    .addStringOption((option) =>
                        option
                            .setName("profile")
                            .setDescription("The profile to view")
                            .setRequired(true)
                    )
            )
            .addSubcommand((subcommand) =>
                subcommand
                    .setName("setAvatar")
                    .setDescription("Set your profile's avatar")
                    .addStringOption((option) =>
                        option
                            .setName("profile")
                            .setDescription("The profile to set the avatar for")
                            .setRequired(true)
                    )
                    // Prompt for them to add url/image
            )
            .addSubcommand((subcommand) =>
                subcommand
                    .setName("edit")
                    .setDescription("Edit an existing profile")
                    // Will prompt modal to fill out fields
            )

            .addSubcommand((subcommand) =>
                subcommand
                    .setName("delete")
                    .setDescription("Delete an existing profile")
                    .addStringOption((option) =>
                        option
                            .setName("profile")
                            .setDescription("The profile to delete")
                            .setRequired(true)
                    )
            )
    }
}