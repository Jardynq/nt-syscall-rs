/*
	Primitive script that allows extracting of syscalls oridnals from 
	https://hfiref0x.github.io/syscalls.html
	Remember to rename invalid module names
*/


var type = "nt"

var to_find = 22000;


// Part 1
var real = "";
var result = {}
var rows = document.getElementsByTagName("table")[0].tBodies[0].rows;
var ids = rows[0].innerText.split("\t");
var invalids = {};


// Part 2 (repeat)
for (let i = 2; i < ids.length; i++) {
	let mod_name = ids[i].toLowerCase().split(" ").join("_").split("(").join("").split(")").join("");
	if (to_find !== undefined && to_find !== "" && to_find != mod_name) {
		continue
	}

	let entries = "";
	for (const [index, row] of Object.entries(rows)) {
		if (index < 1) {
			continue;
		}
		let collumns = row.innerText.split("\t");
		if (collumns[i] === "" || typeof(collumns[i]) === "undefined") {
			collumns[i] = "u32::MAX"
		}
		entries += "\tpub const " + collumns[1] + ": u32 = " + collumns[i] + ";\n";
	}
	if (typeof(result[mod_name]) === "undefined") {
		result[mod_name] = "";
	}
	let table = '\n#[cfg(feature = "' + type + '")]\n' + "pub use " + type + '::*;\n#[cfg(feature = "' + type + '")]\nmod ' + type + " {\n";
	result[mod_name] += table + entries + "}\n";
}

// Part 3
var rev = Object.entries(result);
rev = rev.reverse();
for (const[mod, entries] of rev) {
	real += entries;
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

