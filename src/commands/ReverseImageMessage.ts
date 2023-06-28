import { ContextMenuCommandBuilder, Interaction, ApplicationCommandType, BaseGuildTextChannel, EmbedBuilder, PermissionFlagsBits } from "discord.js";
import { ReverseImageSearch } from "../shared/ReverseImageSearch.js";
import { CommandType } from "../structs/bases/BaseCommand.js";
import { ContextMenuBase } from "../structs/bases/ContextMenu.js";


export class ReverseImageMessage implements ContextMenuBase {
    public name = "Find Image Source";
    public type = CommandType.ContextMenu;

    public builder() {
        return new ContextMenuCommandBuilder()
            .setName(this.name)
            .setDefaultMemberPermissions(PermissionFlagsBits.SendMessages | PermissionFlagsBits.EmbedLinks)
            .setType(ApplicationCommandType.Message);
    }

    public async execute(interaction: Interaction) {
        if (!interaction.isMessageContextMenuCommand()) return;
        const message = interaction.options.getMessage("message");

        let url = ReverseImageSearch.firstImageLink(message?.content as string);
        if (!message || !message.attachments.first()) {
            if (!url) {
                await interaction.reply({ content: "No image in this message!", ephemeral: true });
                return;
            }
        } else {
            url = message.attachments.first()?.url as string;
        }

        await interaction.deferReply({ ephemeral: false });
        const response = await ReverseImageSearch.search(url, (message?.channel as BaseGuildTextChannel).nsfw);
        
        const embed = new EmbedBuilder()
            .setTitle("Image Search Results")
            .setColor("#275dd8");

        if (response == null) {
            embed.setDescription("Too many searches, please try again later.");
            await interaction.editReply({
                embeds: [embed]
            });
            return;
        }

        if (response.results[0] && parseFloat(response.results[0].header.similarity) > 65) {
            embed.setImage(url as string);
            try {
                console.log(Object.keys(response.results[0].data)
                .filter((key) => !["ext_urls", "fa_id"].includes(key))
                
                .map((key) => ({
                    name: key.charAt(0).toUpperCase() + key.slice(1),
                    value: `${(response.results[0].data as any)[key]}`,
                })))

                embed.addFields(
                    Object.keys(response.results[0].data)
                        .filter((key) => !["ext_urls", "fa_id"].includes(key))
                        .filter((key) => (response.results[0].data as any)[key] !== "")

                        .map((key) => ({
                            name: key.charAt(0).toUpperCase() + key.slice(1),
                            value: `${(response.results[0].data as any)[key]}`,
                        }))
                );

                embed.addFields(
                    response.results[0].data.ext_urls
                        .map((key) => ({
                            name: /^(?:https?:\/\/)?(?:[^@\/\n]+@)?(?:www\.)?([^:\/\n]+)/.exec(key)?.[1] as string,
                            value: `[Here](${key})`,
                            inline: true
                        }))
                );
            } catch (_) {}

        } else {
            embed.setDescription("No results found...");
        }
        await interaction.editReply({
            embeds: [embed]
        });
    }
}
