import { Interaction, PermissionFlagsBits, SlashCommandBuilder, TextChannel } from "discord.js";
import { CommandType } from "../structs/bases/BaseCommand.js";
import { SlashCommandBase } from "../structs/bases/SlashCommand.js";

type PruneType = "embeds" | "mentions" | "user" | "all";
export class Prune implements SlashCommandBase {
    public type =  CommandType.Slash;
    public name = "prune";
    public description = "Prune or Purge messages";

    public builder() {
        return new SlashCommandBuilder()
            .setName(this.name)
            .setDMPermission(false)
            .setDefaultMemberPermissions(PermissionFlagsBits.ManageMessages | PermissionFlagsBits.ManageChannels)
            .setDescription(this.description)
            .addSubcommand((subcommand) =>
                subcommand
                    .setName("embeds")
                    .setDescription("Prune embeds")
                    .addIntegerOption((option) =>
                        option
                            .setName("amount")
                            .setDescription("Amount of embeds to prune")
                            .setRequired(true)
                    )
            )
            .addSubcommand((subcommand) =>
                subcommand
                    .setName("mentions")
                    .setDescription("Prune mentions")
                    .addIntegerOption((option) =>
                        option
                            .setName("amount")
                            .setDescription("Amount of mentions to prune")
                            .setRequired(true)
                    )
            )
            .addSubcommand((subcommand) =>
                subcommand
                    .setName("user")
                    .setDescription("Prune messages from a specific user")
                    .addUserOption((option) =>
                        option
                            .setName("user")
                            .setDescription("User to prune messages from")
                            .setRequired(true)
                    )
                    .addIntegerOption((option) =>
                        option
                            .setName("amount")
                            .setDescription("Amount of messages to prune")
                            .setRequired(true)
                    )
            )
            .addSubcommand((subcommand) =>
                subcommand
                    .setName("all")
                    .setDescription("Prune all messages")
                    .addIntegerOption((option) =>
                        option
                            .setName("amount")
                            .setDescription("Amount of messages to prune")
                            .setRequired(true)
                    )
            );
    }

    public async execute(interaction: Interaction) {
        if (!interaction.isCommand()) return;
        if (!interaction.guild) return;

        // Check if the bot has the Manage Messages permission
        if (!interaction.guild.members.cache.get(interaction.client.user.id)?.permissions.has(PermissionFlagsBits.ManageMessages)) {
            await interaction.reply({
                content: "I do not have the Manage Messages permission.",
                ephemeral: true,
            });
            return;
        }

        const subcommand = (interaction.options as any).getSubcommand() as PruneType;
        const amount = (interaction.options as any).getInteger("amount") > 100 ? 100 : (interaction.options as any).getInteger("amount");
        const user = (interaction.options as any).getUser("user");
        const channel = interaction.channel as TextChannel;

        let deleted = 0;
        switch (subcommand) {
            case "embeds":
                const embeds = await channel.messages.fetch({ limit: amount });
                const embedsToPrune = embeds.filter((message) => message.embeds.length > 0);
                deleted = embedsToPrune.size;
                await channel.bulkDelete(embedsToPrune);
                break;
            case "mentions":
                const mentions = await channel.messages.fetch({ limit: amount });
                const mentionsToPrune = mentions.filter((message) => message.mentions.users.size > 0);
                deleted = mentionsToPrune.size;
                await channel.bulkDelete(mentionsToPrune);
                break;
            case "user":
                const userMessages = await channel.messages.fetch({ limit: amount });
                const userMessagesToPrune = userMessages.filter((message) => message.author.id === user?.id);
                deleted = userMessagesToPrune.size;
                await channel.bulkDelete(userMessagesToPrune);
                break;
            case "all":
                await channel.bulkDelete(amount);
                deleted = amount;
                break;
        }

        await interaction.reply({
            content: `Pruned ${deleted} messages.`,
            ephemeral: true,
        });
    };
}