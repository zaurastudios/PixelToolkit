## Main Events

Present in: [`src/main/ipc-events/index.ts`](/src/main/ipc-events/index.ts)

- `close-app`: Calls `app.quit()` and closes the close-app.
- `set-title`: Takes an id argument and sets the window title.

## Create Project Events

Present in [`src/main/ipc-events/create.ts`](/src/main/ipc-events/create.ts)

- `open-directory`: Opens the dialog for choosing directory for project files.
  - **Replies:**
    - `opened-directory`: Returns if cancelled or not, path to dir (if not cancelled) and if selected dir is empty (if not cancelled)
- `open-yml`: This is for selecting a pre-existing project.
  - **Replies:**
    - `opened-yml`: As of now just returns dir data (cancelled, filePaths).
- `create-project-in-dir`: Accepts project title (name), project description and project path in an object. Tries creating the default folder paths `/assets/minecraft/textures/block` and a `project.yml` file in the provided path. Then saves these info with a generated UUID in the app's `config.json` file.
  - **Replies:**
    - `error-create`: If an error occurs while trying to create the project dir, it returns with the error message.
    - `created`: If successful returns the project id, so that user can be navigated to that page (`/:id`).
- `selected-path-in-dir`: Checks if the passed path arg is present in the app's `config.json` file.
  - **Replies:**
    - `selected-path-is-in-dir`: Returns boolean.

## Project Events

Present in [`src/main/ipc-events/project.ts`](/src/main/ipc-events/project.ts)

- `get-my-projects`: Used for fetching the config data from the app's `config.json` file.
  - **Replies:**
    - `my-projects`: Returns the config data.
- `get-project-data-with-id`: Accepts an id. Fetches the project file data from the app's `config.json` and then filters with id.
  - **Replies:**
    - `project-data-reply`: Returns false if no project file with that id is found, else returns the project data if found
