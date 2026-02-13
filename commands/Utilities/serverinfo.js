const { SlashCommandBuilder, Utils, ChannelType } = require('discord.js');
const { errorNotServer } = require("../../messageConfig.json")

module.exports = {
    data: new SlashCommandBuilder().setName("serverinfo").setDescription("Presents various information about the current server"),
    async execute(interaction){
        if (!interaction.guildId){
            await interaction.reply(errorNotServer)
            return
        }

        // Extraneous information gets
        const channels = await interaction.guild.channels.fetch(); 

        // Filter out category channels and get the size
        const channelCount = channels.filter(channel => channel.type !== 4).size;
        const voiceChannelCount = channels.filter(channel => channel.type == ChannelType.GuildVoice).size;

        // Building returned info
        let returnedinfo = ""

        returnedinfo += `${interaction.guild.name}@discord.com | guildID: ${interaction.guild.id} + \n`
        returnedinfo += "--[ Description ]--\n"
        returnedinfo += `${interaction.guild.description}\n`
        returnedinfo += "--[ Member Info ]--\n"
        returnedinfo += `Creation date: ${interaction.guild.createdAt}\n`
        returnedinfo += `Member count: ${interaction.guild.memberCount}/${interaction.guild.maximumMembers}\n`
        returnedinfo += "--[ Channel Info ]--\n"
        returnedinfo += `Channel Count: ${channelCount}\n`
        returnedinfo += `Voice Channel Count: ${voiceChannelCount}/${channelCount}\n`
        returnedinfo += `Maximum bitrate: ${interaction.guild.maximumBitrate}bps\n`

        await interaction.reply(returnedinfo);
    }
}