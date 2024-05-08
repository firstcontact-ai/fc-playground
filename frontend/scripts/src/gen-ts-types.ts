import * as fs from "fs";
import { compileFromFile, Options } from 'json-schema-to-typescript';

await main();

async function main() {

	// -- Get args
	let [schema_json_file, dst_ts_file] = process.argv
		.slice(2)
		.map(arg => arg.replace(/^frontend\//, ''));

	// -- Compile & Save
	const ts_content = await compile_json_schema(schema_json_file);
	fs.writeFileSync(dst_ts_file, ts_content);

	// Note: This file will be executed from ./frontend/
	console.log(`Generated - frontend/${dst_ts_file}`);
}

async function compile_json_schema(schema_file: string): Promise<string> {
	const compileConfig: Partial<Options> = {
		additionalProperties: false,
		style: {
			bracketSpacing: true,
			printWidth: 120,
			semi: true,
			singleQuote: false,
			tabWidth: 2,
			trailingComma: 'none',
			useTabs: true
		}
	}

	let ts_content = await compileFromFile(schema_file, compileConfig);
	ts_content = more_format(ts_content);
	return ts_content;
}


function more_format(content: string): string {
	// Remove the interface called PLACEHOLDER
	content = content.replace(/export interface .*PLACEHOLDER[\s\S]*?\}/g, '');

	content = content.replace(/export/g, "\nexport");

	return content
}