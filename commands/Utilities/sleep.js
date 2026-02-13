const { SlashCommandBuilder, Utils, GuildMemberFlags, ChannelType, PermissionFlagsBits } = require('discord.js');
const { noPermission, errorNotServer } = require("../../messageConfig.json");
const { execute } = require('./serverinfo');

let sleepText = "Fetchmon has spoken. Bedtime!"

module.exports = {
    data: new SlashCommandBuilder().setName("sleep").setDescription("Removes everyone in VC and times them out for one hour."),
    async execute(interaction){
        if (!interaction.guildId){
            await interaction.reply(errorNotServer)
            return
        }

        if (!interaction.member.permissions.has(PermissionFlagsBits.ModerateMembers, true)){
            await interaction.reply(noPermission)
            return
        }

        let channels = await interaction.guild.channels.fetch()

        for (const [,channel] of channels){
            if (!channel.isVoiceBased()){
                continue
            }

            //console.log(channel.members)
            channel.members.each(member => {
                if (!member.moderatable){
                    console.log("this member cannot be managed")
                    return
                }
                member.timeout(60 * 60 * 1000, 'fetchmon.sleep')
            })
        }

        await interaction.reply(
            { content: sleepText, files: ["https://www.drakomatic.net/content/renanight.gif"] }
        )
    }
}