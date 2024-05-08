# fc-tools

- `list_files`      params `{name_contain?: string, topics: string[]}`
- `summarize_files` params `{name_contain?: string, topics: string[]}`


# Chain

```jsonc
{
	"nodes": [
		{ // [0]
			"agent": "self",
			"name_input": "original_input",
		}, 
		{ // [1]
			"branch": [
				// Category 1 - need tools
				{ // branch arm (if, nodes) // [1, 0]
					"cond": {
						"input": {
							"is_json": true,
							"json_matches": {
								"pointer": "/category",
								"value": 1
							}
						}
					},
					"flow": [{
						"agent": { "name": "fc_tool_executors" }
					}, {
						"agent": { "name": "fc_tool_renderers" }
					}]			
				},
				// Category 2 - just generic agent
				{ // branch arm (if, nodes) (or "else": true)
					"cond": {
						"input": {
							"is_json": true,
							"json_matches": {
								"pointer": "/category",
								"value": 2
							}
						}
					},
					"flow": [{
						"agent": { 
							"uid": "Generic Agent", 
							"input": "${original_input}"
						}, 
					}]					
				}, 

				{
					"flow": [{
						"agent": {
							"name": "fall_back"
						}
					}]
				}

			]
		}
	]
}
```