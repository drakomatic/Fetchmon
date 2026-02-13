const { log } = require('console');
const { SlashCommandBuilder, Utils } = require('discord.js');
const util = require("util")
const exec = util.promisify(require('child_process').exec);

const stripAnsiCodes = str => str.replace(/[\u001b\u009b][[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]/g, '');

function FixExcessiveLinesAndSpaces(InString){
	let FixedString = ""

	const FixLines = InString.split("\n")
	for (const line of FixLines){
		const trimmed = line.trimEnd()
		if (trimmed.length > 0) {
            FixedString += trimmed + "\n";
        }
	}

	return FixedString
}

module.exports = {
	data: new SlashCommandBuilder().setName('fetch').setDescription('Replies with system information regarding the developers PC.'),
	async execute(interaction) {
		try {
        	const { stdout: logoOut } = await exec("fastfetch -s logo");
        	const { stdout: infoOut } = await exec("fastfetch --logo none");

        	const logo = stripAnsiCodes(logoOut);

        	let BigLogo = FixExcessiveLinesAndSpaces(logo);

        	const MainText = FixExcessiveLinesAndSpaces(stripAnsiCodes(infoOut));

        	await interaction.reply(`\`\`\`\n${BigLogo}\`\`\``);
        	await interaction.channel.send(`${MainText}`);
    	} catch (err) {
        	console.error(err);
        	await interaction.reply("Failed to fetch system info.");
    	}
	},
};