# IMPORTANT: Playground code base from First Contact AI

- This code and repository are licensed under the MIT or Apache license.
- This is just experimental code for some potential future First Contact Application.
- It might not always be buildable as it may depend on local versions of libraries (e.g., dom-native).
- A more stable version of the code might be published later once core use cases start to crystallize.
- Requires:

| **Requirement** | **Note/Installation**                      |
|-----------------|---------------------------------------|
| **Rust**        | For all backend and tooling           |
| **nodejs/tsc**  | For all frontend and frontend tooling |
| **awesome-cli** | `cargo install awesome-cli`           |
| **webdev**      | `cargo install webdev`                |
| **tauri-cli**   | `cargo install tauri-cli`             |

# Last note on the Data Model (might be outdated)

- `space`
  - `agent` (one or more) - One-click create, might create a default one as we talked about.
  - `drive` (one or more) - Drives will be created on the first drag and drop, but users can add other drives from other spaces.
      - `dsource` (zero or more) - These are the elements the user adds, like a file, a folder, a Google/Microsoft online Word/Excel document, or a GitHub repo (we will add this).
        - `ditem` - These belong to the `dsource` and will be one per "data source item."
           - For example, when `dsource(type: file)` is used, the `ditem` will represent the file. 
           - When a `dsource(type: folder)` is used, we will have one `ditem` for that folder and one for each child item contained within it.
           - A `ditem` will have `dfile_id` pointer.
        - `dfile` - Represents a **sqlite data file** which containg the content element of one ore more `ditem`. 
            - Those _dfiles_ are the one used for vectorization

So, some `dsource` entries will have predefined `ditems`. For instance, a `dsource` of type GitHub might have 3 fixed `ditems`:

- `issues.db3` (might constains couple of tables, for the tabular and content)
- `file_content.fsc.db3` (`fsc` for `file system content` db3 format) (storing all file contents organized by relative path).

This setup enables us to use SQLite to work with these "semi-structured" file formats. We can then leverage technologies like Polars to extend our tabular data processing capabilities.

Additionally, we plan to have a restricted set of **working file formats**, which will be the products of the original `ditem` files. We are still fine-tuning this model but intend to make extensive use of `SQLite`.

Here are some considerations:

- We might create a separate `.db3` file for each `dsource` or file type.
- We will likely have a few types of `.db3` files:
    - One for `text content files`, such as markdown, plain text, Word documents, PDFs, and more. This type of database may be hierarchical, even if it's stored in one table.
    - One for `tabular data`, including Excel and spreadsheets exclusive to Google/Microsoft platforms.

We are exploring various models, including one where a `.db3` file can hold many files of its own type. The decision regarding whether to keep the index/reference in a centralized `.db3` file per datasource or in the main `SurrealDB` database is still pending.

# Tech used

- Frontend
    - [json-schema-to-typescript](https://www.npmjs.com/package/json-schema-to-typescript)

