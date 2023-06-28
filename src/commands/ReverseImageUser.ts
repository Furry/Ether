import { ContextMenuCommandBuilder, Interaction, ApplicationCommandType, EmbedBuilder, PermissionFlagsBits } from "discord.js";
import { ReverseImageSearch } from "../shared/ReverseImageSearch.js";
import { CommandType } from "../structs/bases/BaseCommand.js";
import { ContextMenuBase } from "../structs/bases/ContextMenu.js";

export class ReverseImageUser implements ContextMenuBase {
    public name = "Find Icon Source";
    public type = CommandType.ContextMenu;

    public builder() {
        return new ContextMenuCommandBuilder()
            .setName(this.name)
            .setDefaultMemberPermissions(PermissionFlagsBits.SendMessages | PermissionFlagsBits.EmbedLinks)
            .setType(ApplicationCommandType.User);
    }

    public async execute(interaction: Interaction) {
        if (!interaction.isContextMenuCommand()) return;
        const user = interaction.options.getUser("user");
        if (!user) return;

        await interaction.deferReply({ ephemeral: false });
        const url = user.displayAvatarURL({ size: 512 });
        const response = await ReverseImageSearch.search(user.displayAvatarURL({ size: 512 }), false);

        const embed = new EmbedBuilder()
            .setTitle("User Search Results")
            .setColor("#275dd8")

        if (response == null) {
            embed.setDescription("Too many searches, please try again later.");
            await interaction.editReply({
                embeds: [embed]
            });
            return;
        }

        if (response.results[0] && parseFloat(response.results[0].header.similarity) > 65) {
            embed.setImage(url as string);
            embed.addFields(
                Object.keys(response.results[0].data)
                    .filter((key) => !["ext_urls", "fa_id", "da_id"].includes(key))
                    .map((key) => ({
                        name: (key.charAt(0).toUpperCase() + key.slice(1)).split("_").join(" "),
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
        } else {
            embed.setDescription("No results found...");
        }
        await interaction.editReply({
            embeds: [embed]
        });
    }
}