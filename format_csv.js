/*
	Primitive script that allows extracting of syscalls oridnals from https://github.com/j00ru/windows-syscalls
	Remember to rename invalid module names
*/


var type = "nt"

// Part 1
var real = "";
var result = {}
var rows = document.getElementsByClassName("js-file-line");
var ids = rows[0].innerText.split("\t");
var invalids = {};


// Part 2 (repeat)
for (let i = 2; i < ids.length; i++) {
	let mod_name = ids[i].toLowerCase().split(" ").join("_").split("(").join("").split(")").join("");
	let entries = "";
	for (const [index, row] of Object.entries(rows)) {
		if (index < 1) {
			continue;
		}
		let collumns = row.innerText.split("\t");
		if (collumns[i] === "" || typeof(collumns[i]) === "undefined") {
			collumns[i] = "u32::MAX"
		}
		entries += "\t\tpub const " + collumns[1] + ": u32 = " + collumns[i] + ";\n";
	}
	if (typeof(result[mod_name]) === "undefined") {
		result[mod_name] = "";
	}
	let table = '\n\t#[cfg(feature = "' + type +'")] ' + "pub use " + type + '::*;\n\t#[cfg(feature = "' + type +'")] mod ' + type + " {\n";
	result[mod_name] += table + entries + "\t}\n";
}

// Part 3
var rev = Object.entries(result);
rev = rev.reverse();
for (const[mod, entries] of rev) {
	real += '\n#[cfg(feature = "' + mod + '")]\n';
	real += "pub mod " + mod + " {" + entries + "}\n";
}
clear();
console.log(real);





// Part 2 generate invalid
for (const [index, row] of Object.entries(rows)) {
	if (index < 1) {
		continue;
	}
	let collumns = row.innerText.split("\t");
	if (typeof(invalids[collumns[1]]) === "undefined") {
		invalids[collumns[1]] =  "\tpub const " + collumns[1] + ": u32 = u32::MAX;\n";
	}
}


// Part 3 generate invalid
var table = '\n#[cfg(feature = "' + type +'")] ' + "pub use " + type + '::*;\n#[cfg(feature = "' + type +'")] mod ' + type + " {\n";
for (const[name, line] of Object.entries(invalids)) {
	real += line;
}
clear();
console.log(table + real + "}\n");

