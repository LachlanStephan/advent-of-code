const fs = require("fs");

function main() {
	fs.readFile("../input/binary_diagnostic.txt", "utf-8", (err, f) => {
		if (err) {
			throw err;
		}

		const split_data = splitFile(f);
		const gamma = findGamma(split_data);
		console.log(gamma);
	});
}

function splitFile(file_data) {
	return file_data.split("\n");
}

function findGamma(file_data) {
	const pos_one = [];
	file_data.forEach((data) => {
		pos_one.push(data[0].toString().split(""));
	});

	const most_common = getMostCommonGamma(pos_one);
	return most_common;
}

function getMostCommonGamma(gammas) {
	//
}

main();
